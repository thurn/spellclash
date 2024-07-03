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

use data::card_definitions::ability_definition::{Ability, EffectFn};
use data::card_states::zones::ZoneQueries;
use data::core::primitives::{
    AbilityId, CardId, EffectId, EntityId, HasController, PlayerName, StackAbilityId, StackItemId,
};
use data::delegates::scope::{EffectContext, Scope};
use data::game_states::game_state::GameState;
use utils::outcome;
use utils::outcome::Outcome;

pub fn run(
    game: &mut GameState,
    ability_id: AbilityId,
    stack_ability_id: Option<StackAbilityId>,
    ability: &dyn Ability,
) -> Outcome {
    let effect_id = new_effect_id(game);
    match stack_ability_id {
        Some(stack_ability_id) => {
            let stack_ability = game.stack_ability(stack_ability_id);
            if let Some(delayed_trigger_effect_id) = stack_ability.delayed_trigger_effect_id {
                let context = EffectContext {
                    scope: Scope {
                        controller: stack_ability.controller,
                        ability_id,
                        timestamp: delayed_trigger_effect_id.timestamp(),
                    },
                    effect_id: delayed_trigger_effect_id,
                };
                ability.invoke_delayed_trigger_effect(game, context);
            } else {
                let context = EffectContext {
                    scope: Scope {
                        controller: stack_ability.controller,
                        ability_id,
                        timestamp: effect_id.timestamp(),
                    },
                    effect_id,
                };
                ability.invoke_effect(game, context);
            }
        }
        _ => {
            let card = game.card(ability_id)?;
            let context = EffectContext {
                scope: Scope {
                    controller: card.controller(),
                    ability_id,
                    timestamp: effect_id.timestamp(),
                },
                effect_id,
            };
            ability.invoke_effect(game, context);
        }
    };
    outcome::OK
}

/// Creates a new [EffectId].
///
/// All [EffectId]s are also valid timestamps and share the same ID space.
fn new_effect_id(game: &mut GameState) -> EffectId {
    EffectId(game.zones.new_timestamp().0)
}
