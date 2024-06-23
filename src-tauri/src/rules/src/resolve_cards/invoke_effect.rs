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

use data::card_definitions::ability_choices::CardOrPlayer;
use data::card_definitions::ability_definition::EffectFn;
use data::card_states::zones::ZoneQueries;
use data::core::primitives::{
    AbilityId, CardId, EntityId, HasController, PlayerName, StackAbilityId, StackItemId,
};
use data::delegates::has_delegates::HasDelegates;
use data::delegates::scope::EffectScope;
use data::game_states::game_state::GameState;
use utils::outcome;
use utils::outcome::Outcome;

pub fn run(
    game: &mut GameState,
    ability_id: AbilityId,
    stack_ability_id: Option<StackAbilityId>,
    effect: &Option<EffectFn>,
) -> Outcome {
    if let Some(function) = effect {
        let controller = match stack_ability_id {
            Some(stack_ability_id) => game.stack_ability(stack_ability_id).controller,
            _ => game.card(ability_id)?.controller(),
        };
        let scope =
            EffectScope { controller, ability_id, effect_id: game.ability_state.new_effect_id() };
        function(game, scope);
        outcome::OK
    } else {
        outcome::SKIPPED
    }
}
