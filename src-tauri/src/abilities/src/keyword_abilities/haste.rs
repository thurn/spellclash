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
use data::delegates::delegate_data::QueryValue;
use data::delegates::game_delegates::GameDelegates;

use crate::core::gain_ability;
use crate::core::gain_ability::GainAbility;

/// > 702.10a. Haste is a static ability.
///
/// > 702.10b. If a creature has haste, it can attack even if it hasn't been
/// > controlled by its controller continuously since their most recent turn
/// > began. (See rule 302.6.)
///
/// > 702.10c. If a creature has haste, its controller can activate its
/// > activated abilities whose cost includes the tap symbol or the untap symbol
/// > even if that creature hasn't been controlled by that player continuously
/// > since their most recent turn began. (See rule 302.6.)
///
/// > 702.10d. Multiple instances of haste on the same creature are redundant.
///
/// <https://yawgatog.com/resources/magic-rules/#R70210>
pub fn ability() -> impl Ability {
    StaticAbility::new().delegates(|d| gain(d, GainAbility::ThisCard))
}

/// Adds the haste ability to the given delegates.
pub fn gain(delegates: &mut GameDelegates, add_ability: GainAbility) {
    gain_ability::add_to_query(&mut delegates.has_haste, add_ability, |_, s, _| {
        QueryValue::set(s, true)
    });
}
