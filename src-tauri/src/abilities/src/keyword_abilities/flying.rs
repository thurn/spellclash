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

use data::card_definitions::ability_definition::{Ability, StaticAbility};
use data::core::primitives::HasSource;
use data::delegates::delegate_data::{Flag, QueryValue};
use data::delegates::game_delegates::GameDelegates;
use rules::queries::combat_queries;

use crate::core::gain_ability;
use crate::core::gain_ability::GainAbility;

/// > 702.9a. Flying is an evasion ability.
///
/// > 702.9b. A creature with flying can't be blocked except by creatures with
/// > flying and/or reach. A creature with flying can block a creature with or
/// > without flying. (See rule 509, "Declare Blockers Step," and rule 702.17,
/// > "Reach.")
///
/// > 702.9c. Multiple instances of flying on the same creature are redundant.
///
/// <https://yawgatog.com/resources/magic-rules/#R7029>
pub fn ability() -> impl Ability {
    StaticAbility::new().delegates(|d| gain(d, GainAbility::ThisCard))
}

/// Adds the flying ability to the given delegates.
pub fn gain(delegates: &mut GameDelegates, add_ability: GainAbility) {
    gain_ability::add_to_query(&mut delegates.has_flying, add_ability, |_, s, _| {
        Flag::set(s, true)
    });
    gain_ability::add_to_query(&mut delegates.can_be_blocked, add_ability, |g, s, data| {
        Flag::and_predicate(g, s, data.blocker_id, combat_queries::has_flying)
    });
}
