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

use data::card_states::card_state::CardState;
use data::card_states::stack_ability_state::StackAbilityState;
use data::game_states::game_state::GameState;
use data::prompts::prompt::PromptType;
use data::prompts::select_order_prompt::CardOrderLocation;
use primitives::game_primitives::{CardType, EntityId, HasController, PlayerName, Zone};

use crate::core::object_position::{BattlefieldPosition, ObjectPosition, Position};
use crate::core::response_builder::ResponseBuilder;

/// Calculates the game position in which the provided card should be displayed.
pub fn calculate(builder: &ResponseBuilder, _game: &GameState, card: &CardState) -> ObjectPosition {
    if let Some(position) = position_override(builder, card) {
        return position;
    }

    let owner = builder.to_display_player(card.owner);
    let position = match card.zone {
        Zone::Hand => Position::Hand(owner),
        Zone::Graveyard => Position::DiscardPile(owner),
        Zone::Library => Position::Deck(owner),
        Zone::Battlefield => Position::Battlefield(
            builder.to_display_player(card.controller()),
            if card.printed().face.card_types.contains(CardType::Land) {
                BattlefieldPosition::Mana
            } else {
                BattlefieldPosition::Permanents
            },
        ),
        Zone::Stack => Position::Stack,
        Zone::Exiled => Position::Exile(owner),
        Zone::Command => Position::CommandZone(owner),
        Zone::OutsideTheGame => Position::Offscreen,
    };

    for_card(card, position)
}

pub fn for_card(card: &CardState, position: Position) -> ObjectPosition {
    let sorting_key = card.object_id.as_sorting_key();
    ObjectPosition { position, sorting_key, sorting_sub_key: 0.0 }
}

pub fn for_stack_ability(ability: &StackAbilityState, position: Position) -> ObjectPosition {
    let sorting_key = ability.object_id.as_sorting_key();
    ObjectPosition { position, sorting_key, sorting_sub_key: 0.0 }
}

pub fn deck(builder: &ResponseBuilder, player: PlayerName) -> Position {
    Position::Deck(builder.to_display_player(player))
}

pub fn hand(builder: &ResponseBuilder, player: PlayerName) -> Position {
    Position::Hand(builder.to_display_player(player))
}

pub fn discard(builder: &ResponseBuilder, player: PlayerName) -> Position {
    Position::DiscardPile(builder.to_display_player(player))
}

fn position_override(builder: &ResponseBuilder, card: &CardState) -> Option<ObjectPosition> {
    if let Some(prompt) = builder.current_prompt() {
        match &prompt.prompt_type {
            PromptType::EntityChoice(data) => {
                if card.zone == Zone::Battlefield || card.zone == Zone::Stack {
                    // Entities on the battlefield or stack are displayed in their normal positions
                    // during an entity choice prompt.
                    return None;
                }

                if let Some((i, _)) = data
                    .choices
                    .iter()
                    .enumerate()
                    .find(|(_, choice)| choice.entity_id.matches_card(card.id))
                {
                    // Show entities in other zones in a selector
                    return Some(ObjectPosition {
                        position: Position::CardSelectionChoices,
                        sorting_key: i as f64,
                        sorting_sub_key: 0.0,
                    });
                }
            }
            PromptType::SelectOrder(data) => {
                for location in enum_iterator::all::<CardOrderLocation>() {
                    if let Some(i) =
                        data.cards_in_location(location).iter().position(|c| c == &card.id)
                    {
                        return Some(ObjectPosition {
                            position: Position::CardOrderLocation(location),
                            sorting_key: i as f64,
                            sorting_sub_key: 0.0,
                        });
                    }
                }
            }
            _ => {}
        }
    }
    None
}
