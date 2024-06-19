// Copyright © spellclash 2024-present
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//   https://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use std::collections::{HashSet, VecDeque};
use std::sync::Arc;

use ai_core::core::agent_state::AgentState;
use enumset::EnumSet;
use rand::SeedableRng;
use rand_xoshiro::Xoshiro256StarStar;
use serde::{Deserialize, Serialize};

use crate::actions::agent_action::AgentAction;
use crate::actions::game_action::GameAction;
use crate::actions::user_action::UserAction;
use crate::card_states::card_kind::CardKind;
use crate::card_states::card_state::CardState;
use crate::card_states::stack_ability_state::StackAbilityState;
use crate::card_states::zones::{ZoneQueries, Zones};
use crate::core::numerics::TurnNumber;
#[allow(unused_imports)] // Used in docs
use crate::core::primitives::{
    CardId, EntityId, GameId, HasCardId, HasPlayerName, HasSource, PlayerName, StackAbilityId,
    StackItemId, UserId, Zone,
};
use crate::decks::deck::Deck;
use crate::delegates::game_delegates::GameDelegates;
use crate::game_states::combat_state::CombatState;
use crate::game_states::game_step::GamePhaseStep;
use crate::game_states::history_data::{GameHistory, HistoryCounters, HistoryEvent};
use crate::game_states::oracle::Oracle;
use crate::game_states::state_based_event::StateBasedEvent;
use crate::game_states::undo_tracker::UndoTracker;
use crate::player_states::player_state::{PlayerQueries, PlayerState, PlayerType, Players};
use crate::prompts::game_update::UpdateChannel;
use crate::prompts::prompt::Prompt;
use crate::state_machines::state_machine_data::StateMachines;

/// This is the state of a single ongoing game of Magic (i.e. one duel, not a
/// larger session of the spellclash game client).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GameState {
    id: GameId,
    status: GameStatus,
    step: GamePhaseStep,
    turn: TurnData,
    priority: PlayerName,
    passed: EnumSet<PlayerName>,
    configuration: GameConfiguration,
    state_machines: StateMachines,
    players: Players,
    zones: Zones,
    #[serde(skip)]
    updates: Option<UpdateChannel>,
    combat: Option<CombatState>,
    history: GameHistory,
    rng: Xoshiro256StarStar,
    undo_tracker: UndoTracker,
    #[serde(skip)]
    delegates: GameDelegates,
    state_based_events: Option<Vec<StateBasedEvent>>,
    #[serde(skip)]
    oracle_reference: Option<Box<dyn Oracle>>,
    #[serde(skip)]
    agent_state: Option<AgentState<PlayerName, AgentAction>>,
    #[serde(skip)]
    current_agent_searcher: Option<PlayerName>,
}

impl GameState {
    pub fn new(
        oracle: Box<dyn Oracle>,
        id: GameId,
        p1: PlayerType,
        p1_deck: Deck,
        p2: PlayerType,
        p2_deck: Deck,
        debug: DebugConfiguration,
    ) -> Self {
        let mut zones = Zones::default();
        let turn = TurnData { active_player: PlayerName::One, turn_number: 0 };
        create_cards_in_deck(oracle.as_ref(), &mut zones, p1_deck, PlayerName::One, turn);
        create_cards_in_deck(oracle.as_ref(), &mut zones, p2_deck, PlayerName::Two, turn);
        GameState {
            id,
            status: GameStatus::Setup,
            step: GamePhaseStep::Untap,
            turn: TurnData { active_player: PlayerName::One, turn_number: 0 },
            priority: PlayerName::One,
            passed: EnumSet::empty(),
            configuration: GameConfiguration::new(PlayerName::One | PlayerName::Two, debug),
            state_machines: StateMachines::default(),
            players: Players::new(p1, p2, 20),
            zones,
            updates: None,
            combat: None,
            history: GameHistory::default(),
            rng: Xoshiro256StarStar::seed_from_u64(3141592653589793),
            undo_tracker: UndoTracker { enabled: true, undo: vec![] },
            delegates: GameDelegates::default(),
            state_based_events: Some(vec![]),
            oracle_reference: Some(oracle),
            agent_state: None,
            current_agent_searcher: None,
        }
    }

    /// Unique ID for this game
    pub fn id(&self) -> GameId {
        self.id
    }

    /// Status of the game: whether it is starting, is ongoing, or has ended.
    pub fn status(&self) -> &GameStatus {
        &self.status
    }

    /// Mutable equivalent of [Self::status].
    pub fn status_mut(&mut self) -> &mut GameStatus {
        &mut self.status
    }

    /// Current game phase step.
    ///
    /// If the game has not yet started, this will be "Untap". If the game has
    /// ended, this will be the step in which the game ended.    
    pub fn step(&self) -> GamePhaseStep {
        self.step
    }

    /// Mutable equivalent of [Self::step].
    pub fn step_mut(&mut self) -> &mut GamePhaseStep {
        &mut self.step
    }

    /// Identifies the player whose turn it currently is and the current turn
    /// number.
    ///
    /// If the game has not yet started, this will be turn 0 for player one. If
    /// the game has ended, this will be the turn on which the game ended.    
    pub fn turn(&self) -> &TurnData {
        &self.turn
    }

    /// Mutable equivalent of [Self::turn].
    pub fn turn_mut(&mut self) -> &mut TurnData {
        &mut self.turn
    }

    /// Player who can currently take a game action.
    ///
    /// This is somewhat more expansive than the 'priority' concept in the CR.
    /// Actions are always handled sequentially, there is no such thing as a
    /// 'simultaneous' action, hence there is always exactly one player who can
    /// currently act.
    ///
    /// If the game has not yet started, this will be player one. If the game
    /// has ended, this will be the player who held priority at the end of the
    /// game.
    pub fn priority(&self) -> PlayerName {
        self.priority
    }

    /// Mutable equivalent of [Self::priority].
    pub fn priority_mut(&mut self) -> &mut PlayerName {
        &mut self.priority
    }

    pub fn oracle(&self) -> &dyn Oracle {
        self.oracle_reference.as_ref().expect("Oracle reference not populated").as_ref()
    }

    /// Players whose last game action was to pass priority. When all players
    /// pass priority, the current item on the stack resolves or the current
    /// game step ends.
    pub fn passed(&self) -> &EnumSet<PlayerName> {
        &self.passed
    }

    /// Mutable equivalent of [Self::passed].
    pub fn passed_mut(&mut self) -> &mut EnumSet<PlayerName> {
        &mut self.passed
    }

    /// Options controlling overall gameplay
    pub fn configuration(&self) -> &GameConfiguration {
        &self.configuration
    }

    /// State for the players within this game
    pub fn players(&self) -> &Players {
        &self.players
    }

    /// Mutable equivalent of [Self::players].
    pub fn players_mut(&mut self) -> &mut Players {
        &mut self.players
    }

    /// Stores state for all cards and abilities in this game and tracks which
    /// game zone they are in.
    pub fn zones(&self) -> &Zones {
        &self.zones
    }

    /// Mutable equivalent of [Self::zones].
    pub fn zones_mut(&mut self) -> &mut Zones {
        &mut self.zones
    }

    /// Channel on which to send game updates.
    ///
    /// If no channel is provided here, game mutations will be applied silently
    /// without returning incremental updates.
    pub fn updates(&self) -> Option<&UpdateChannel> {
        self.updates.as_ref()
    }

    /// Mutable equivalent of [Self::updates]
    pub fn updates_mut(&mut self) -> &mut Option<UpdateChannel> {
        &mut self.updates
    }

    /// State of creatures participating in the currently active combat phase,
    /// if any.
    pub fn combat(&self) -> Option<&CombatState> {
        self.combat.as_ref()
    }

    /// Mutable equivalent of [Self::combat].
    pub fn combat_mut(&mut self) -> &mut Option<CombatState> {
        &mut self.combat
    }

    ///  History of events which have happened during this game. See
    /// [GameHistory].
    pub fn history(&self) -> &GameHistory {
        &self.history
    }

    /// Mutable equivalent of [Self::history].
    pub fn history_mut(&mut self) -> &mut GameHistory {
        &mut self.history
    }

    /// Random number generator to use for this game
    pub fn rng(&self) -> &Xoshiro256StarStar {
        &self.rng
    }

    /// Mutable equivalent of [Self::rng].
    pub fn rng_mut(&mut self) -> &mut Xoshiro256StarStar {
        &mut self.rng
    }

    /// Handles state tracking for the 'undo' action.    
    pub fn undo_tracker(&self) -> &UndoTracker {
        &self.undo_tracker
    }

    /// Mutable equivalent of [Self::undo_tracker].
    pub fn undo_tracker_mut(&mut self) -> &mut UndoTracker {
        &mut self.undo_tracker
    }

    /// Active Delegates for the game. See [GameDelegates].
    pub fn delegates(&self) -> &GameDelegates {
        &self.delegates
    }

    /// Mutable equivalent of [Self::delegates].
    pub fn delegates_mut(&mut self) -> &mut GameDelegates {
        &mut self.delegates
    }

    /// Tracks events which have occurred since the last time state-based
    /// actions were checked which may trigger game mutations during the next
    /// state-based action check.
    pub fn state_based_events(&self) -> Option<&Vec<StateBasedEvent>> {
        self.state_based_events.as_ref()
    }

    /// Mutable equivalent of [Self::state_based_events].
    pub fn state_based_events_mut(&mut self) -> &mut Option<Vec<StateBasedEvent>> {
        &mut self.state_based_events
    }

    /// Reference to the Oracle card database to use with this game.
    ///
    /// This value is populated immediately after deserialization and should
    /// almost always be safe to unwrap. Instead of accessing this field, use
    /// the [Self::oracle] method.
    pub fn internal_oracle_reference_mut(&mut self) -> &mut Option<Box<dyn Oracle>> {
        &mut self.oracle_reference
    }

    pub fn agent_state(&self) -> Option<&AgentState<PlayerName, AgentAction>> {
        self.agent_state.as_ref()
    }

    pub fn agent_state_mut(&mut self) -> &mut Option<AgentState<PlayerName, AgentAction>> {
        &mut self.agent_state
    }

    pub fn current_search_agent(&self) -> Option<PlayerName> {
        self.current_agent_searcher
    }

    pub fn current_search_agent_mut(&mut self) -> &mut Option<PlayerName> {
        &mut self.current_agent_searcher
    }

    /// Changes the controller for a card.
    ///
    /// Panics if this card was not found on the battlefield.
    pub fn change_controller(
        &mut self,
        source: impl HasSource,
        id: impl HasCardId,
        controller: PlayerName,
    ) {
        self.zones.change_controller(source, id, controller, self.turn)
    }

    /// Makes a clone of this game state suitable suitable for use in display
    /// or simulation logic, but which omits undo tracking information, agent
    /// state, and the ability to process incremental visual updates.
    pub fn shallow_clone(&self) -> Self {
        Self {
            updates: None,
            undo_tracker: UndoTracker { enabled: false, undo: vec![] },
            agent_state: None,
            ..self.clone()
        }
    }

    /// Shuffles the order of cards in a player's library
    pub fn shuffle_library(&mut self, player: PlayerName) {
        self.zones.shuffle_library(player, &mut self.rng)
    }

    /// Finds the name of the player with the given user ID
    ///
    /// Panics if this user is not a player in this game.
    pub fn find_player_name(&self, user_id: UserId) -> PlayerName {
        for name in enum_iterator::all::<PlayerName>() {
            if self.player(name).player_type.user_id() == Some(user_id) {
                return name;
            }
        }
        panic!("User {user_id:?} is not a player in game {:?}", self.id);
    }

    /// Returns the player whose turn it is
    pub fn active_player(&self) -> PlayerName {
        self.turn.active_player
    }

    /// Adds a current [HistoryEvent] for the current turn.
    pub fn add_history_event(&mut self, event: HistoryEvent) {
        self.history.add_event(self.turn, event)
    }

    /// Returns a reference to the [HistoryCounters] for the [PlayerName]
    /// player in the current turn.
    pub fn history_counters(&self, player: PlayerName) -> &HistoryCounters {
        self.history.counters_for_turn(self.turn, player)
    }

    /// Mutable equivalent of [Self::history_counters].
    pub fn history_counters_mut(&mut self, player: PlayerName) -> &mut HistoryCounters {
        self.history.counters_for_turn_mut(self.turn, player)
    }

    /// Adds a new tracked [StateBasedEvent].
    pub fn add_state_based_event(&mut self, event: StateBasedEvent) {
        if let Some(events) = &mut self.state_based_events {
            events.push(event);
        } else {
            self.state_based_events = Some(vec![event]);
        }
    }
}

impl ZoneQueries for GameState {
    fn card(&self, id: impl HasCardId) -> &CardState {
        self.zones.card(id)
    }

    fn card_mut(&mut self, id: impl HasCardId) -> &mut CardState {
        self.zones.card_mut(id)
    }

    fn card_entity(&self, id: EntityId) -> Option<&CardState> {
        self.zones.card_entity(id)
    }

    fn card_entity_mut(&mut self, id: EntityId) -> Option<&mut CardState> {
        self.zones.card_entity_mut(id)
    }

    fn stack_ability(&self, id: StackAbilityId) -> &StackAbilityState {
        self.zones.stack_ability(id)
    }

    fn stack_ability_mut(&mut self, id: StackAbilityId) -> &mut StackAbilityState {
        self.zones.stack_ability_mut(id)
    }

    fn library(&self, player: impl HasPlayerName) -> &VecDeque<CardId> {
        self.zones.library(player)
    }

    fn hand(&self, player: impl HasPlayerName) -> &HashSet<CardId> {
        self.zones.hand(player)
    }

    fn graveyard(&self, player: impl HasPlayerName) -> &VecDeque<CardId> {
        self.zones.graveyard(player)
    }

    fn battlefield(&self, player: impl HasPlayerName) -> &HashSet<CardId> {
        self.zones.battlefield(player)
    }

    fn battlefield_owned(&self, player: impl HasPlayerName) -> &HashSet<CardId> {
        self.zones.battlefield_owned(player)
    }

    fn exile(&self, player: impl HasPlayerName) -> &HashSet<CardId> {
        self.zones.exile(player)
    }

    fn stack(&self) -> &[StackItemId] {
        self.zones.stack()
    }

    fn command_zone(&self, player: impl HasPlayerName) -> &HashSet<CardId> {
        self.zones.command_zone(player)
    }

    fn outside_the_game_zone(&self, player: impl HasPlayerName) -> &HashSet<CardId> {
        self.zones.outside_the_game_zone(player)
    }
}

impl PlayerQueries for GameState {
    fn player(&self, name: PlayerName) -> &PlayerState {
        self.players.player(name)
    }

    fn player_mut(&mut self, name: PlayerName) -> &mut PlayerState {
        self.players.player_mut(name)
    }
}

/// Status of the game: whether it is starting, is ongoing, or has ended.
#[derive(Debug, Clone, Serialize, Deserialize, Eq, PartialEq)]
pub enum GameStatus {
    /// Initial step of game setup. Players reveal commanders, companions,
    /// sticker sheets, etc.
    ///
    /// See <https://yawgatog.com/resources/magic-rules/#R1032>
    Setup,

    /// Players resolve mulligans in sequence.
    ///
    /// See <https://yawgatog.com/resources/magic-rules/#R1035>
    ResolveMulligans,

    /// Players take actions with cards in their opening hands
    ///
    /// See <https://yawgatog.com/resources/magic-rules/#R1036>
    PreGameActions,

    /// Game is currently ongoing
    Playing,

    /// Game has ended and the [PlayerName] players have won.
    ///
    /// If the winner set is empty, the game has ended in a draw.
    GameOver { winners: EnumSet<PlayerName> },
}

/// Identifies a turn within the game.
#[derive(Debug, Clone, Copy, Serialize, Deserialize, Eq, PartialEq, Hash)]
pub struct TurnData {
    /// Player whose turn it is or was.
    pub active_player: PlayerName,

    /// Turn number for that player.
    ///
    /// The first turn of the game is turn 0.
    pub turn_number: TurnNumber,
}

/// Options controlling overall gameplay
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct GameConfiguration {
    /// If true, all random choices within this game will be made
    /// deterministically using a seeded random number generator. Useful for
    /// e.g. unit tests.
    pub deterministic: bool,

    /// Whether to run in simulation mode and thus disable update tracking
    pub simulation: bool,

    /// Whether to overwrite the normal game behavior with the standard
    /// pre-scripted new player experience.
    pub scripted_tutorial: bool,

    /// Set of players currently in this game, i.e. who have not yet lost
    ///
    /// Currently only 2 players are supported, but I see no reason not to allow
    /// future expansion.
    pub all_players: EnumSet<PlayerName>,

    /// Debug options for this game
    pub debug: DebugConfiguration,
}

impl GameConfiguration {
    pub fn new(all_players: EnumSet<PlayerName>, debug: DebugConfiguration) -> Self {
        Self {
            deterministic: false,
            simulation: false,
            scripted_tutorial: false,
            all_players,
            debug,
        }
    }
}

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Serialize, Deserialize)]
pub struct DebugConfiguration {
    /// If true, all cards are revealed to all players
    pub reveal_all_cards: bool,

    /// Allows the user in a game to take actions as though they were another
    /// specified player.
    pub act_as_player: Option<DebugActAsPlayer>,
}

/// Allows a player to take actions for another player during debugging
#[derive(Debug, Clone, Copy, Eq, PartialEq, Serialize, Deserialize)]
pub struct DebugActAsPlayer {
    pub id: UserId,
    pub name: PlayerName,
}

fn create_cards_in_deck(
    oracle: &dyn Oracle,
    zones: &mut Zones,
    deck: Deck,
    owner: PlayerName,
    turn: TurnData,
) {
    for (&id, &quantity) in &deck.cards {
        for _ in 0..quantity {
            zones.create_card_in_library(oracle.card(id), CardKind::Normal, owner, turn);
        }
    }
}
