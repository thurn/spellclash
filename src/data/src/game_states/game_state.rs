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

use enumset::EnumSet;
use rand_xoshiro::Xoshiro256StarStar;
use serde::{Deserialize, Serialize};
use utils::fail;
use utils::outcome::{Outcome, Value};

use crate::card_states::card_state::CardState;
use crate::card_states::zones::{ZoneQueries, Zones};
use crate::core::numerics::TurnNumber;
use crate::core::primitives::{CardId, GameId, HasCardId, HasPlayerName, PlayerName, UserId};
use crate::delegates::game_delegates::GameDelegates;
use crate::game_states::animation_tracker::{
    AnimationState, AnimationStep, AnimationTracker, GameAnimation,
};
use crate::game_states::combat_state::CombatState;
use crate::game_states::game_step::GamePhaseStep;
use crate::game_states::history_data::GameHistory;
use crate::game_states::undo_state::UndoTracker;
use crate::player_states::player_state::{PlayerQueries, PlayerState, Players};
use crate::prompts::prompt_manager::PromptManager;
use crate::state_machines::state_machine_data::StateMachines;

/// This is the state of a single ongoing game of Magic (i.e. one duel, not a
/// larger session of the spellclash game client).
#[derive(Debug, Clone, Serialize, Deserialize)]
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
    /// If the game has not yet started, this will be player one. if the game
    /// has ended, this will be the player who held priority at the end of the
    /// game.
    pub priority: PlayerName,

    /// Players whose last game action was to pass priority. When all players
    /// pass priority, the current item on the stack resolves or the current
    /// game step ends.
    pub passed: EnumSet<PlayerName>,

    /// Options controlling overall gameplay
    pub configuration: GameConfiguration,

    /// Collection of state machines for handling resolution of multi-step game
    /// updates.
    pub state_machines: StateMachines,

    /// State for the players within this game
    pub players: Players,

    /// Stores state for all cards and abilities in this game and tracks which
    /// game zone they are in.
    pub zones: Zones,

    /// Handles sending prompts for user actions to players in this game
    #[serde(skip)]
    pub prompts: PromptManager,

    /// State of the currently active or most recently completed combat phase.
    ///
    /// If no combat phases have occurred this turn, this will contain
    /// CombatState::default().
    pub combat: CombatState,

    /// Used to track changes to game state in order to update the client. See
    /// [AnimationTracker] for more information.
    #[serde(skip)]
    pub animations: AnimationTracker,

    ///  History of events which have happened during this game. See
    /// [GameHistory].
    pub history: GameHistory,

    /// Random number generator to use for this game
    pub rng: Xoshiro256StarStar,

    /// Handles state tracking for the 'undo' action.
    pub undo_tracker: UndoTracker,

    /// Active Delegates for the game. See [GameDelegates].
    #[serde(skip)]
    pub delegates: GameDelegates,
}

impl GameState {
    pub fn add_animation(&mut self, update: impl FnOnce() -> GameAnimation) {
        if self.animations.state == AnimationState::Track {
            // Snapshot current game state, omit things that aren't important for
            // terminal_ui logic.
            let clone = Self {
                animations: AnimationTracker::new(AnimationState::Ignore),
                undo_tracker: Default::default(),
                ..self.clone()
            };

            self.animations.steps.push(AnimationStep { snapshot: clone, update: update() });
        }
    }

    /// Clears the set of players who have passed priority.
    pub fn clear_passed(&mut self) {
        self.passed = EnumSet::empty();
    }

    /// Shuffles the order of cards in a player's library
    pub fn shuffle_library(&mut self, player: PlayerName) -> Outcome {
        self.zones.shuffle_library(player, &mut self.rng)
    }

    /// Finds the name of the player with the given user ID
    ///
    /// Returns an error this user is not a player in this game.
    pub fn find_player_name(&self, user_id: UserId) -> Value<PlayerName> {
        for name in enum_iterator::all::<PlayerName>() {
            if self.player(name).user_id == Some(user_id) {
                return Ok(name);
            }
        }
        fail!("User {user_id:?} is not a player in game {:?}", self.id);
    }
}

impl ZoneQueries for GameState {
    fn card(&self, id: impl HasCardId) -> &CardState {
        self.zones.card(id)
    }

    fn card_mut(&mut self, id: impl HasCardId) -> &mut CardState {
        self.zones.card_mut(id)
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

    fn stack(&self) -> &[CardId] {
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
#[derive(Debug, Clone, Serialize, Deserialize)]
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

    /// Game has ended and the [PlayerName] player has won.
    GameOver { winner: PlayerName },
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

    /// Set of players in this game
    ///
    /// Currently only 2 players are supported, but I see no reason not to allow
    /// future expansion.
    pub all_players: EnumSet<PlayerName>,
}

impl GameConfiguration {
    pub fn new(all_players: EnumSet<PlayerName>) -> Self {
        Self { deterministic: false, simulation: false, scripted_tutorial: false, all_players }
    }
}
