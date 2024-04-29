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

use rand_xoshiro::Xoshiro256StarStar;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::core::primitives::{PlayerName, TurnNumber};
use crate::delegates::game_delegates::GameDelegates;
use crate::game_states::animation_tracker::AnimationTracker;
use crate::game_states::combat_state::CombatState;
use crate::game_states::game_step::GamePhaseStep;
use crate::game_states::history_data::GameHistory;
use crate::game_states::undo_state::UndoTracker;
use crate::state_machines::state_machine_data::StateMachines;

/// This is the state of a single ongoing game of Magic (i.e. one duel, not a
/// larger session of the spellclash game client).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GameState {
    /// Unique ID for this game
    pub id: Uuid,

    /// Current game phase step.
    ///
    /// If the game has not yet started, this will be "Upkeep". If the game has
    /// ended, this will be the step in which the game ended.
    pub step: GamePhaseStep,

    /// Identifies the player whose turn it currently is and the current turn
    /// number.
    ///
    /// If the game has not yet started, this will be turn 0 for player one. If
    /// the game has ended, this will be the turn on which the game ended.
    pub current_turn: TurnData,

    /// Options controlling overall gameplay
    pub configuration: GameConfiguration,

    /// Collection of state machines for handling resolution of multi-step game
    /// updates.
    pub state_machines: StateMachines,

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

    /// Next sorting key to use for card moves.
    ///
    /// Do not reference this directly, it is automatically updated by
    /// [Self::next_sorting_key] and [Self::move_card_internal].
    pub next_sorting_key: u32,

    /// Random number generator to use for this game
    pub rng: Xoshiro256StarStar,

    /// Handles state tracking for the 'undo' action.
    pub undo_tracker: UndoTracker,

    /// Active Delegates for the game. See [GameDelegates].
    #[serde(skip)]
    pub delegates: GameDelegates,
}

/// Identifies a turn within the game.
#[derive(Debug, Clone, Copy, Serialize, Deserialize, Eq, PartialEq, Hash)]
pub struct TurnData {
    /// Player whose turn it is or was.
    pub turn: PlayerName,

    /// Turn number for that player
    pub turn_number: TurnNumber,
}

/// Options controlling overall gameplay
#[derive(Debug, Clone, Default, Copy, Serialize, Deserialize)]
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
}
