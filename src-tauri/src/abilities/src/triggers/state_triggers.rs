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

use data::card_definitions::ability_definition::{Ability, TriggeredAbility};
use data::card_states::iter_matching::IterMatching;
use data::card_states::zones::ZoneQueries;
use data::core::function_types::{CardMutation, CardPredicate};
use enumset::EnumSet;
use primitives::game_primitives::{AbilityId, CardId, HasSource, PermanentId, Zone};
use rules::mutations::trigger_extension::TriggerExt;
/// If this card's controller controls no permanents matching `predicate`,
/// applies `mutation` to it.
pub fn when_controls_no(
    predicate: impl CardPredicate<PermanentId>,
    mutation: impl CardMutation<CardId>,
) -> impl Ability {
    TriggeredAbility::new()
        .events(move |s, e| {
            e.will_enter_battlefield.add_ability(s, EnumSet::all(), move |g, c, _| {
                g.events.state_triggered_ability.add_state_trigger(s, move |g, _, _| {
                    g.battlefield(c.controller).none_matching(g, c.source(), predicate)
                });
            });
            e.will_leave_battlefield.add_ability(s, EnumSet::all(), move |g, _, _| {
                g.events.state_triggered_ability.remove_callbacks(s.ability_id);
            });
        })
        .effect(move |g, c| {
            mutation(g, c.source(), c.this.card_id);
        })
}
