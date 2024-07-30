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
use data::delegates::layer::Layer;
use data::delegates::query_value::EnumSets;
use data::events::event_context::EventContext;
use data::game_states::game_state::GameState;
use data::printed_cards::card_subtypes::CreatureType;
use data::properties::duration::Duration;
use enumset::EnumSet;
use primitives::game_primitives::{Color, HasSource, PermanentId};
use utils::outcome::Outcome;

/// Sets a card's creature subtypes for the current turn
pub fn set_this_turn(
    game: &mut GameState,
    context: EventContext,
    id: PermanentId,
    colors: impl Into<EnumSet<CreatureType>>,
) -> Outcome {
    game.card_mut(id)?.properties.creature_types.add_effect(
        context,
        Duration::WhileOnBattlefieldThisTurn(id, context.current_turn),
        EnumSets::set(Layer::TypeChangingEffects, context, colors.into()),
    )
}
