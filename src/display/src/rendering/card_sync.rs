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

use data::card_states::card_state::{CardFacing, TappedState};
use data::core::primitives::Source;
use data::printed_cards::printed_card::{Face, PrintedCardFace};
use rules::play_cards::play_card;

use crate::core::card_view::{CardView, RevealedCardFace, RevealedCardView};
use crate::core::object_position::ObjectPosition;
use crate::core::response_builder::ResponseBuilder;
use crate::rendering::card_view_context::CardViewContext;
use crate::rendering::positions;

/// Builds a display representation of the state of a single card or card-like
/// object
pub fn card_view(builder: &ResponseBuilder, context: &CardViewContext) -> CardView {
    let revealed = context.query_or(true, |_, card| card.revealed_to.contains(builder.player));
    CardView {
        id: context.card_id(),
        position: context.query_or(ObjectPosition::default(), |game, card| {
            positions::calculate(builder, game, card)
        }),
        card_back: "https://static.wikia.nocookie.net/mtgsalvation_gamepedia/images/f/f8/Magic_card_back.jpg/revision/latest?cb=20140813141013".to_string(),
        revealed: revealed.then(|| RevealedCardView {
            face: card_face(&context.printed().face),
            can_play: context.query_or(false, |game, card| {
                play_card::can_play_card(game, builder.player, Source::Game, card.id)
            }),
            face_b: context.printed().face_b.as_ref().map(card_face),
            layout: context.printed().layout,
        }),
        revealed_to_opponents: context
            .query_or(false, |_, card| !card.zone.is_public() && card.revealed_to.len() > 1),
        card_facing: context.query_or(CardFacing::FaceUp(Face::Primary), |_, card| card.facing),
        tapped_state: context.query_or(TappedState::Untapped, |_, card| card.tapped_state),
        damage: Default::default(),
        create_position: if builder.state.animate {
            context.query_or_none(|_, card| {
                positions::for_card(card, positions::deck(builder, card.owner))
            })
        } else {
            None
        },
        destroy_position: context.query_or_none(|_, card| {
            positions::for_card(card, positions::deck(builder, card.owner))
        }),
    }
}

fn card_face(printed: &PrintedCardFace) -> RevealedCardFace {
    RevealedCardFace {
        name: printed.name.clone(),
        image: card_image(printed),
        layout: printed.layout,
        rules_text: printed.oracle_text.clone(),
    }
}

fn card_image(printed: &PrintedCardFace) -> String {
    let id = printed.variants[0].scryfall_id.to_string();
    let dir1 = id.chars().next().unwrap();
    let dir2 = id.chars().nth(1).unwrap();
    format!("https://cards.scryfall.io/large/front/{dir1}/{dir2}/{id}.jpg")
}
