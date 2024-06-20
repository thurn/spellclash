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

use data::card_definitions::ability_definition::{AbilityBuilder, StaticAbility};
use data::card_states::zones::ZoneQueries;
use data::core::function_types::CardPredicate;
use data::core::primitives::Zone;
use data::delegates::game_delegates::GameDelegates;
use rules::queries::combat_queries;

/// Prevent this creature from attacking unless the defending player controls a
/// permanent matching the given predicate.
pub fn cannot_attack_unless_defender_controls(
    predicate: impl CardPredicate,
) -> impl AbilityBuilder {
    StaticAbility::new().delegate(Zone::Battlefield, move |d| {
        d.can_attack_target.this(move |g, s, data, current| {
            current.add_condition(
                s,
                g.battlefield(combat_queries::defending_player(g, data.target))
                    .iter()
                    .any(|&card_id| predicate(g, s, card_id)),
            )
        })
    })
}
