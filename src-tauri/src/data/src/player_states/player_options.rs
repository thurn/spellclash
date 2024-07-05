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

use enumset::EnumSet;
use serde::{Deserialize, Serialize};

use crate::game_states::game_phase_step::GamePhaseStep;

/// Configurable options for a player within a game
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlayerOptions {
    /// Steps in which the game should pause for priority during this player's
    /// turn
    ///
    /// If a player does not have a stop set for a given step, they will
    /// automatically use the "pass priority" action whenever the stack is empty
    /// during that step.
    ///
    /// Note that if no attackers are declared during combat, the following
    /// combat steps will be skipped even if there is a stop present here.
    pub active_turn_stops: EnumSet<GamePhaseStep>,

    /// Steps in which the game should pause for priority during other players'
    /// turns.
    pub inactive_turn_stops: EnumSet<GamePhaseStep>,

    /// If true, the player will automatically pass priority if they have no
    /// other legal game actions available.
    ///
    /// Certain specific steps (main phases, end of opponent turn) always stop
    /// the game even if this option is enabled.
    pub auto_pass: bool,

    /// Should this player retain priority after putting something on the stack?
    pub hold_priority: bool,

    /// Should this player receive priority after each item on the stack
    /// resolves?
    pub resolve_individual_stack_items: bool,
}

impl Default for PlayerOptions {
    fn default() -> Self {
        Self {
            active_turn_stops: GamePhaseStep::PreCombatMain
                | GamePhaseStep::DeclareAttackers
                | GamePhaseStep::DeclareBlockers
                | GamePhaseStep::PostCombatMain,
            inactive_turn_stops: GamePhaseStep::DeclareAttackers
                | GamePhaseStep::DeclareBlockers
                | GamePhaseStep::EndStep,
            auto_pass: true,
            hold_priority: false,
            resolve_individual_stack_items: false,
        }
    }
}
