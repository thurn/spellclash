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

use data::card_definitions::ability_definition::Ability;
use data::card_states::zones::ZoneQueries;
use data::core::primitives::{
    AbilityId, CardId, EntityId, EventId, HasController, PlayerName, Source, StackAbilityId,
    StackItemId,
};
use data::game_states::game_state::GameState;
use utils::outcome;
use utils::outcome::Outcome;

use crate::dispatcher::dispatch;

pub fn run(
    game: &mut GameState,
    ability_id: AbilityId,
    stack_ability_id: Option<StackAbilityId>,
    ability: &dyn Ability,
) -> Outcome {
    let mut context = dispatch::build_event_context(game, ability_id)?;
    match stack_ability_id {
        Some(stack_ability_id) => {
            let stack_ability = game.stack_ability(stack_ability_id);
            if let Some(custom_effect) = &stack_ability.custom_effect {
                // Use the original event ID for custom effects so they can be
                // tied back to their origin.
                context.event_id = custom_effect.event_id;
                let effect = custom_effect.effect.clone();
                effect.invoke(game, context);
            } else {
                ability.invoke_effect(game, context);
            }
        }
        _ => {
            let card = game.card(ability_id)?;
            ability.invoke_effect(game, context);
        }
    };
    outcome::OK
}
