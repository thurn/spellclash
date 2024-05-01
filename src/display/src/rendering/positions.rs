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
use data::core::primitives::{CardType, PlayerName, Zone};
use data::game_states::game_state::GameState;

use crate::core::object_position::{BattlefieldPosition, ObjectPosition, Position};
use crate::core::response_builder::ResponseBuilder;

/// Calculates the game position in which the provided card should be displayed.
pub fn calculate(builder: &ResponseBuilder, _game: &GameState, card: &CardState) -> ObjectPosition {
    let owner = builder.to_display_player(card.owner);
    let position = match card.zone {
        Zone::Hand => Position::Hand(owner),
        Zone::Graveyard => Position::DiscardPile(owner),
        Zone::Library => Position::Deck(owner),
        Zone::Battlefield => Position::Battlefield(
            builder.to_display_player(card.controller),
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
    ObjectPosition { position, sorting_key: card.object_id.as_u64(), sorting_sub_key: 0 }
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