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
use data::card_states::iter_matching::IterMatching;
use data::card_states::zones::ZoneQueries;
use data::core::function_types::CardPredicate;
use data::delegates::game_delegate_data::CanAttackTarget;
use data::delegates::layer::Layer;
use data::properties::flag::Flag;
use primitives::game_primitives::{PermanentId, Source, Zone, PRINTED_TEXT_TIMESTAMP};

/// Prevent this creature from attacking unless the defending player controls a
/// permanent matching the given predicate.
pub fn cannot_attack_unless_defender_controls(
    predicate: impl CardPredicate<PermanentId>,
) -> impl Ability {
    StaticAbility::new().properties(move |s, p| {
        p.can_attack_target.add_ability(
            s,
            Flag::and(move |g, source, data: &CanAttackTarget| {
                g.battlefield(data.target.defending_player()).any_matching(g, source, predicate)
            }),
        )
    })
}
