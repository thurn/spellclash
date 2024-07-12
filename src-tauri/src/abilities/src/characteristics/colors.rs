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

use data::card_states::zones::{ToCardId, ZoneQueries};
use data::core::primitives::{Color, HasSource, PermanentId};
use data::delegates::delegate_type::DelegateType;
use data::delegates::game_delegates::GameDelegates;
use data::delegates::layer::Layer;
use data::delegates::query_value::{EnumSets, QueryValue};
use data::delegates::scope::EffectContext;
use data::game_states::game_state::GameState;
use data::properties::card_modifier::CardModifier;
use data::properties::duration::Duration;
use enumset::EnumSet;
use utils::outcome::Outcome;

pub fn set_this_turn(
    game: &mut GameState,
    context: EffectContext,
    id: PermanentId,
    colors: impl Into<EnumSet<Color>>,
) -> Outcome {
    let turn = game.turn;
    game.card_mut(id)?.properties.colors.add_effect(
        context,
        Duration::WhileOnBattlefieldThisTurn(id, turn),
        EnumSets::set(Layer::ColorChangingEffects, context, colors.into()),
    )
}
