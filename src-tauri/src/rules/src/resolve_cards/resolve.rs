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

use data::card_definitions::ability_definition::AbilityType;
use data::card_definitions::definitions;
use data::card_states::card_kind::CardKind;
use data::card_states::zones::ZoneQueries;
use data::core::primitives::{AbilityId, CardId, Source, StackAbilityId, StackItemId, Zone};
use data::delegates::has_delegates::HasDelegates;
use data::delegates::scope::Scope;
use data::game_states::game_state::GameState;
use enumset::EnumSet;
use tracing::{debug, info};
use utils::outcome;
use utils::outcome::Outcome;

use crate::mutations::{move_card, permanents};
use crate::queries::card_queries;
use crate::resolve_cards::invoke_effect;

/// Resolve the top item on the stack and apply its effects. Has no effect if
/// the stack is empty.
///
/// > 608.1. Each time all players pass in succession, the spell or ability on
/// > top of the stack resolves.
///
/// See <https://yawgatog.com/resources/magic-rules/#R608>
pub fn resolve_top_of_stack(game: &mut GameState) -> Outcome {
    match game.stack().last().copied() {
        Some(StackItemId::Card(card_id)) => resolve_top_card_of_stack(game, card_id),
        Some(StackItemId::StackAbility(stack_ability_id)) => {
            resolve_top_ability_of_stack(game, stack_ability_id)
        }
        _ => outcome::OK,
    }
}

fn resolve_top_card_of_stack(game: &mut GameState, card_id: CardId) -> Outcome {
    debug!(?card_id, "Resolving top card of stack");
    let definition = definitions::get(game.card(card_id).card_name);
    for (ability_number, ability) in definition.iterate_abilities() {
        if ability.ability_type == AbilityType::Spell {
            let ability_id = AbilityId { card_id, number: ability_number };
            invoke_effect::run(
                game,
                ability_id,
                None,
                game.card(card_id).targets.clone(),
                &ability.effect,
            )?;
        }
    }

    let card = game.card(card_id);
    if card_queries::card_types(game, card_id).iter().any(|t| t.is_permanent()) {
        // > 608.3. If the object that's resolving is a permanent spell, its resolution may involve
        // > several steps. The instructions in rules 608.3a and b are always performed first. Then
        // > one of the steps in rule 608.3c-e is performed, if appropriate.
        // <https://yawgatog.com/resources/magic-rules/#R6083>

        if card.targets.is_empty() {
            // > 608.3a. If the object that's resolving has no targets, it becomes a permanent and
            // > enters the battlefield under the control of the spell's controller.
            // <https://yawgatog.com/resources/magic-rules/#R6083a>
            let face = if card.cast_as.len() == 1 {
                card.cast_as.iter().next().unwrap()
            } else {
                panic!("Expected only a single face!");
            };
            permanents::turn_face_up(game, Source::Game, card_id, face)?;
            move_card::run(game, Source::Game, card_id, Zone::Battlefield)?;
        } else {
            todo!("Implement targeting for permanents");
        }
    } else {
        // > 608.2m. As the final part of an instant or sorcery spell's resolution, the spell
        // is put into its owner's graveyard.
        move_card::run(game, Source::Game, card_id, Zone::Graveyard)?;
    }

    outcome::OK
}

fn resolve_top_ability_of_stack(game: &mut GameState, stack_ability_id: StackAbilityId) -> Outcome {
    debug!(?stack_ability_id, "Resolving top ability of stack");
    let ability_id = game.stack_ability(stack_ability_id).ability_id;
    let ability_definition =
        definitions::get(game.card(ability_id.card_id).card_name).get_ability(ability_id.number);
    invoke_effect::run(
        game,
        ability_id,
        Some(stack_ability_id),
        game.stack_ability(stack_ability_id).targets.clone(),
        &ability_definition.effect,
    )?;
    game.zones.remove_stack_ability(stack_ability_id);
    outcome::OK
}
