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

use data::card_states::card_state::{CardFacing, CardState, TappedState};
use data::card_states::stack_ability_state::StackAbilityState;
use data::printed_cards::layout::{CardLayout, FaceLayout};
use data::printed_cards::printed_card::Face;

use crate::core::card_view::{CardView, ClientCardId, RevealedCardFace, RevealedCardView};
use crate::core::object_position::Position;
use crate::core::response_builder::ResponseBuilder;
use crate::rendering::card_view_context::CardViewContext;
use crate::rendering::{card_sync, positions};

pub fn stack_ability_view(
    builder: &ResponseBuilder,
    parent: &CardState,
    ability: &StackAbilityState,
) -> CardView {
    CardView {
        id: ClientCardId::new_for_stack_ability(ability.id),
        position: positions::for_stack_ability(ability, Position::Stack),
        card_back: "".to_string(),
        revealed: Some(RevealedCardView {
            image: card_sync::card_image(parent.printed_card_id, Face::Primary),
            face: RevealedCardFace {
                name: parent.displayed_name().to_string(),
                layout: FaceLayout::Normal,
                rules_text: Some("Hello".to_string()),
            },
            status: None,
            is_ability: true,
            click_action: None,
            can_drag: false,
            face_b: None,
            layout: CardLayout::Normal,
        }),
        revealed_to_opponents: true,
        card_facing: CardFacing::FaceUp(Face::Primary),
        tapped_state: TappedState::Untapped,
        damage: 0.0,
        create_position: None,
        destroy_position: None,
    }
}
