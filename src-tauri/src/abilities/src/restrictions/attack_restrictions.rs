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
use data::card_definitions::registry::Registry;
use data::card_states::iter_matching::IterMatching;
use data::card_states::zones::{ToCardId, ZoneQueries};
use data::core::function_types::CardPredicate;
use data::core::primitives::{CardId, PermanentId, Source, Zone};
use data::delegates::game_delegate_data::CanAttackTarget;
use data::delegates::game_delegates::GameDelegates;
use data::delegates::scope::Scope;
use data::game_states::game_state::GameState;
use data::properties::flag::Flag;
use rules::queries::combat_queries;

/// Prevent this creature from attacking unless the defending player controls a
/// permanent matching the given predicate.
pub fn cannot_attack_unless_defender_controls(
    registry: &mut Registry,
    predicate: impl CardPredicate<PermanentId>,
) -> impl Ability {
    let can_attack_target =
        registry.add(move |g: &GameState, s: Source, data: &CanAttackTarget| {
            Some(g.battlefield(data.target.defending_player()).any_matching(g, s, predicate))
        });

    StaticAbility::new()
        .initialize(move |q| q.can_attack_target.add_static(Flag::and_predicate(can_attack_target)))
}
