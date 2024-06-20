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

use data::card_definitions::ability_definition::{AbilityBuilder, TriggeredAbility};
use data::card_states::zones::ZoneQueries;
use data::core::primitives::{HasCardId, HasSource, Zone};
use data::delegates::game_delegates::GameDelegates;
use utils::outcome;

use crate::core::types::{CardMutation, CardPredicate};

/// If this card's controller controls no permanents matching `predicate`,
/// applies `mutation` to it.
pub fn when_controls_no(
    predicate: impl CardPredicate,
    mutation: impl CardMutation,
) -> impl AbilityBuilder {
    TriggeredAbility::new()
        .condition(Zone::Battlefield, move |d| {
            d.state_triggered_abilities.whenever(move |g, s, _| {
                if !g.battlefield(s.controller).iter().any(|&card_id| predicate(g, s, card_id)) {
                    mutation(g, s.source(), s.card_id())?;
                }
                outcome::OK
            })
        })
        .effects(|_, _| outcome::OK)
}
