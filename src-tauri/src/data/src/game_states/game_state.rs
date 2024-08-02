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

use std::collections::{BTreeSet, VecDeque};

use ai_core::core::agent_state::AgentState;
use enumset::EnumSet;
#[allow(unused_imports)] // Used in docs
use primitives::game_primitives::{CardId, StackAbilityId};
use primitives::game_primitives::{
    GameId, HasPlayerName, PermanentId, PlayerName, StackItemId, Timestamp, UserId,
};
use rand_xoshiro::Xoshiro256StarStar;
use serde::{Deserialize, Serialize};
use utils::outcome;
use utils::outcome::Outcome;

use crate::actions::agent_action::AgentAction;
use crate::card_states::card_state::{CardState, LostAllAbilities};
use crate::card_states::stack_ability_state::StackAbilityState;
use crate::card_states::zones::{HasZones, ToCardId, ZoneQueries, Zones};
use crate::core::numerics::TurnNumber;
use crate::events::game_events::GlobalEvents;
use crate::game_states::ability_state::AbilityState;
use crate::game_states::combat_state::CombatState;
use crate::game_states::game_phase_step::GamePhaseStep;
use crate::game_states::history_data::{GameHistory, HistoryCounters, HistoryEvent};
use crate::game_states::oracle::Oracle;
use crate::game_states::state_based_event::StateBasedEvent;
use crate::player_states::player_map::PlayerMap;
use crate::player_states::player_state::{PlayerQueries, PlayerState, Players};
use crate::prompts::game_update::UpdateChannel;
use crate::prompts::prompt::PromptResponse;
use crate::properties::duration::Duration;

/// The high-level activity which this [GameState] is being used for.
#[derive(Debug, Clone)]
pub enum GameOperationMode {
    /// Normal gameplay
    Playing,

    /// The [PlayerName] AI player is searching for an action to take.
    AgentSearch(PlayerName),

    /// We are replaying game actions in order to reconstruct a game state from
    /// its serialized representation.
    SerializationReplay(PlayerMap<Vec<PromptResponse>>),
}

/// This is the state of a single ongoing game of Magic (i.e. one duel, not a
/// larger session of the spellclash game client).
#[derive(Clone)]
pub struct GameState {
    /// Unique ID for this game
    pub id: GameId,

    /// Status of the game: whether it is starting, is ongoing, or has ended.
    pub status: GameStatus,

    /// Current game phase step.
    ///
    /// If the game has not yet started, this will be "Untap". If the game has
    /// ended, this will be the step in which the game ended.
    pub step: GamePhaseStep,

    /// Identifies the player whose turn it currently is and the current turn
    /// number.
    ///
    /// If the game has not yet started, this will be turn 0 for player one. If
    /// the game has ended, this will be the turn on which the game ended.
    pub turn: TurnData,

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
    pub priority: PlayerName,

    /// Players whose last game action was to pass priority. When all players
    /// pass priority, the current item on the stack resolves or the current
    /// game step ends.
    pub passed: EnumSet<PlayerName>,

    /// Options controlling overall gameplay
    pub configuration: GameConfiguration,

    /// State for the players within this game
    pub players: Players,

    /// Stores state for all cards and abilities in this game and tracks which
    /// game zone they are in.
    pub zones: Zones,

    /// State associated with abilities in this game.
    pub ability_state: AbilityState,

    /// Channel on which to send game updates.
    ///
    /// If no channel is provided here, game mutations will be applied silently
    /// without returning incremental updates.
    pub updates: Option<UpdateChannel>,

    /// State of creatures participating in the currently active combat phase,
    /// if any.
    pub combat: Option<CombatState>,

    ///  History of events which have happened during this game. See
    /// [GameHistory].
    pub history: GameHistory,

    /// Seed used to initialize the random number generator for this game
    pub rng_seed: u64,

    /// Random number generator to use for this game
    pub rng: Xoshiro256StarStar,

    /// Stores callbacks to invoke in response to game events.
    pub events: GlobalEvents,

    /// Tracks events which have occurred since the last time state-based
    /// actions were checked which may trigger game mutations during the next
    /// state-based action check.
    pub state_based_events: Option<Vec<StateBasedEvent>>,

    /// Reference to the Oracle card database to use with this game.
    ///
    /// This value is populated immediately after deserialization and should
    /// almost always be safe to unwrap. Instead of accessing this field, use
    /// the [Self::oracle] method.
    pub oracle_reference: Option<Box<dyn Oracle>>,

    /// State for an active AI agent, if any
    pub agent_state: Option<AgentState<PlayerName, AgentAction>>,

    /// Current high-level activity which this [GameState] is being used for.
    pub operation_mode: GameOperationMode,

    /// True if the game is currently checking for state-triggered abilities.
    pub checking_state_triggered_abilities: bool,

    /// True if game initialization has been run on this game.
    pub initialized: bool,
}

impl GameState {
    pub fn oracle(&self) -> &dyn Oracle {
        self.oracle_reference.as_ref().expect("Oracle reference not populated").as_ref()
    }

    /// Makes a clone of this game state suitable suitable for use in display
    /// or simulation logic, but which omits undo tracking information, agent
    /// state, and the ability to process incremental visual updates.
    pub fn shallow_clone(&self) -> Self {
        Self { updates: None, agent_state: None, ..self.clone() }
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

    /// Checks if a card has lost all abilities, and returns the [Timestamp] at
    /// which all abilities were removed.
    pub fn has_lost_all_abilities(&self, id: CardId) -> Option<Timestamp> {
        let mut highest = None;
        for lost in &self.card(id)?.lost_all_abilities {
            if lost.duration.is_active(self) && highest < Some(lost.timestamp) {
                highest = Some(lost.timestamp);
            }
        }
        highest
    }
}

impl HasZones for GameState {
    fn zones(&self) -> &Zones {
        &self.zones
    }
}

impl ZoneQueries for GameState {
    fn card(&self, id: impl ToCardId) -> Option<&CardState> {
        self.zones.card(id)
    }

    fn card_mut(&mut self, id: impl ToCardId) -> Option<&mut CardState> {
        self.zones.card_mut(id)
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

    fn hand(&self, player: impl HasPlayerName) -> &BTreeSet<CardId> {
        self.zones.hand(player)
    }

    fn graveyard(&self, player: impl HasPlayerName) -> &VecDeque<CardId> {
        self.zones.graveyard(player)
    }

    fn battlefield(&self, player: impl HasPlayerName) -> &BTreeSet<PermanentId> {
        self.zones.battlefield(player)
    }

    fn battlefield_owned(&self, player: impl HasPlayerName) -> &BTreeSet<PermanentId> {
        self.zones.battlefield_owned(player)
    }

    fn exile(&self, player: impl HasPlayerName) -> &BTreeSet<CardId> {
        self.zones.exile(player)
    }

    fn stack(&self) -> &[StackItemId] {
        self.zones.stack()
    }

    fn command_zone(&self, player: impl HasPlayerName) -> &BTreeSet<CardId> {
        self.zones.command_zone(player)
    }

    fn outside_the_game_zone(&self, player: impl HasPlayerName) -> &BTreeSet<CardId> {
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
#[derive(Debug, Clone, Eq, PartialEq)]
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
#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash, Ord, PartialOrd)]
pub struct TurnData {
    /// Player whose turn it is or was.
    pub active_player: PlayerName,

    /// Turn number for that player.
    ///
    /// The first turn of the game is turn 0.
    pub turn_number: TurnNumber,
}

/// Options controlling overall gameplay
#[derive(Debug, Clone, Copy)]
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
