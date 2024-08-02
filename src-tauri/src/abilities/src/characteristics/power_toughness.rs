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
use data::events::event_context::EventContext;
use data::game_states::game_state::GameState;
use data::properties::duration::Duration;
use data::properties::property_value::Ints;
use primitives::game_primitives::{HasSource, PermanentId};
use utils::outcome::Outcome;

/// Adds to a card's power and toughness for the current turn
pub fn add_this_turn(
    game: &mut GameState,
    context: EventContext,
    id: PermanentId,
    power: Power,
    toughness: Toughness,
) -> Outcome {
    game.card_mut(id)?.properties.power.add_effect(
        context,
        Duration::WhileOnBattlefieldThisTurn(id, context.current_turn),
        Ints::add(power),
    );
    game.card_mut(id)?.properties.toughness.add_effect(
        context,
        Duration::WhileOnBattlefieldThisTurn(id, context.current_turn),
        Ints::add(toughness),
    )
}
