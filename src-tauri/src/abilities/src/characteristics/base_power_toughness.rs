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
use data::core::layer::Layer;
use data::core::numerics::{Power, Toughness};
use data::events::event_context::EventContext;
use data::game_states::game_state::GameState;
use data::properties::duration::Duration;
use data::properties::property_value::Ints;
use primitives::game_primitives::{HasSource, PermanentId};
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
