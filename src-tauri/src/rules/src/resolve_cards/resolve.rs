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

use data::card_definitions::definitions;
use data::card_states::card_kind::CardKind;
use data::card_states::zones::ZoneQueries;
use data::core::primitives::{Source, StackItemId, Zone};
use data::delegates::scope::Scope;
use data::game_states::game_state::GameState;
use enumset::EnumSet;
use tracing::{debug, info};

use crate::mutations::{move_card, permanents};
use crate::queries::card_queries;

/// Resolve the top item on the stack and apply its effects. Has no effect if
/// the stack is empty.
///
/// > 608.1. Each time all players pass in succession, the spell or ability on
/// > top of the stack resolves.
///
/// See <https://yawgatog.com/resources/magic-rules/#R608>
pub fn resolve_top_of_stack(game: &mut GameState) {
    let Some(StackItemId::Card(card_id)) = game.stack().last().copied() else {
        return;
    };
    debug!(?card_id, "Resolving top of stack");

    let definition = definitions::get(game.card(card_id).card_name);
    for (ability_number, ability) in definition.abilities() {
        if let Some(effect) = ability.effects {
            let scope = Scope {
                controller: game.card(card_id).controller,
                number: ability_number,
                card_id,
            };
            effect(game, scope);
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
            permanents::turn_face_up(game, Source::Game, card_id, face);
            move_card::run(game, Source::Game, card_id, Zone::Battlefield);
        } else {
            todo!("Implement targeting for permanents");
        }
    } else {
        // > 608.2m. As the final part of an instant or sorcery spell's resolution, the spell
        // is put into its owner's graveyard.
        move_card::run(game, Source::Game, card_id, Zone::Graveyard);
    }
}
