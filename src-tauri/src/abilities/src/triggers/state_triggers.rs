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

use data::card_definitions::ability_definition::{Ability, AbilityData, TriggeredAbility};
use data::card_states::iter_matching::IterMatching;
use data::card_states::zones::ZoneQueries;
use data::core::function_types::{CardMutation, CardPredicate};
use data::core::primitives::{AbilityId, CardId, HasSource, PermanentId, Zone};
use data::delegates::game_delegates::GameDelegates;
use data::delegates::scope::Scope;
use data::events::game_event::GameEventCallback;
use data::game_states::game_state::GameState;
use enumset::EnumSet;
use rules::mutations::trigger_extension::TriggerExt;
use utils::outcome;
use utils::outcome::Outcome;

/// If this card's controller controls no permanents matching `predicate`,
/// applies `mutation` to it.
pub fn when_controls_no(
    predicate: impl CardPredicate<PermanentId>,
    mutation: impl CardMutation<CardId>,
) -> impl Ability {
    TriggeredAbility::new()
        .events(move |s, e| {
            e.state_triggered_ability.trigger_if_not_on_stack(s, move |g, c, _| {
                g.battlefield(c.controller).none_matching(g, c.this_source(), predicate)
            });
        })
        .effect(move |g, c| {
            mutation(g, c.source(), c.ability_id().card_id);
        })
}
