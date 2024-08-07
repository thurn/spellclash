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

use enum_iterator::Sequence;
use enumset::EnumSetType;

#[derive(Debug, Ord, PartialOrd, Hash, EnumSetType, Sequence)]
pub enum GamePhaseStep {
    Untap,
    Upkeep,
    Draw,
    PreCombatMain,
    BeginCombat,
    DeclareAttackers,
    DeclareBlockers,
    FirstStrikeDamage,
    CombatDamage,
    EndCombat,
    PostCombatMain,
    EndStep,
    Cleanup,
}

impl GamePhaseStep {
    pub fn is_main_phase(&self) -> bool {
        matches!(self, GamePhaseStep::PreCombatMain | GamePhaseStep::PostCombatMain)
    }
}
