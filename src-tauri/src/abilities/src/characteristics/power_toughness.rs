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

use data::card_states::zones::ZoneQueries;
use data::core::numerics::{Power, Toughness};
use data::core::primitives::{HasSource, PermanentId};
use data::delegates::delegate_type::DelegateType;
use data::delegates::game_delegates::GameDelegates;
use data::delegates::layer::Layer;
use data::delegates::query_value::{EnumSets, Ints, QueryValue};
use data::delegates::scope::EffectContext;
use data::game_states::game_state::GameState;
use data::printed_cards::card_subtypes::CreatureType;
use data::queries::card_modifier::CardModifier;
use data::queries::duration::Duration;
use enumset::EnumSet;
use rules::queries::query_extension::QueryExt;

/// Adds to a card's power and toughness for the current turn
pub fn add_this_turn(
    game: &mut GameState,
    context: EffectContext,
    id: PermanentId,
    power: Power,
    toughness: Toughness,
) {
    let turn = game.turn;
    if let Some(card) = game.card_mut(id) {
        card.queries.power.add(CardModifier {
            source: context.source(),
            duration: Duration::WhileOnBattlefieldThisTurn(id, turn),
            delegate_type: DelegateType::Effect,
            effect: Ints::add(power),
        });
        card.queries.toughness.add(CardModifier {
            source: context.source(),
            duration: Duration::WhileOnBattlefieldThisTurn(id, turn),
            delegate_type: DelegateType::Effect,
            effect: Ints::add(toughness),
        });
    }
}
