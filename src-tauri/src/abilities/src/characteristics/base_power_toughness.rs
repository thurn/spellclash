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
use data::delegates::layer::Layer;
use data::delegates::query_value::{EnumSets, Ints, QueryValue};
use data::events::event_context::EventContext;
use data::game_states::game_state::GameState;
use data::printed_cards::card_subtypes::CreatureType;
use data::properties::card_modifier::CardModifier;
use data::properties::duration::Duration;
use enumset::EnumSet;
use utils::outcome::Outcome;

/// Sets a card's base power and toughness for the current turn
pub fn set_this_turn(
    game: &mut GameState,
    context: EventContext,
    id: PermanentId,
    power: Power,
    toughness: Toughness,
) -> Outcome {
    let turn = game.turn;
    game.card_mut(id)?.properties.base_power.add_effect(
        context,
        Duration::WhileOnBattlefieldThisTurn(id, turn),
        Ints::set(Layer::PowerToughnessSettingEffects, context, power),
    );
    game.card_mut(id)?.properties.base_toughness.add_effect(
        context,
        Duration::WhileOnBattlefieldThisTurn(id, turn),
        Ints::set(Layer::PowerToughnessSettingEffects, context, toughness),
    )
}
