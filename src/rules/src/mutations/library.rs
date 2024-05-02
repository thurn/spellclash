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

use data::card_states::zones::ZoneQueries;
use data::core::primitives::{CardId, HasCardId, HasPlayerName, HasSource, Zone};
use data::game_states::game_state::GameState;

/// Draws a card from the top of the `player`'s library.
///
/// Returns the ID of the card drawn, or None if the draw was prevented or the
/// library is empty.
pub fn draw(
    game: &mut GameState,
    player: impl HasPlayerName,
    _source: impl HasSource,
) -> Option<CardId> {
    let id = *game.library(player).back()?;
    game.zones.move_card(id, Zone::Hand);
    Some(id)
}

/// Draws `count` cards in sequence from the top of the `player`'s library.
///
/// Events are fired one at a time for each individual draw. Returns the number
/// of cards actually drawn. The result will be zero if e.g. all draws are
/// prevented.
pub fn draw_cards(
    game: &mut GameState,
    player: impl HasPlayerName,
    source: impl HasSource,
    count: usize,
) -> usize {
    let p = player.player_name();
    let s = source.source();
    let mut drawn = 0;
    for _ in 0..count {
        if draw(game, p, s).is_some() {
            drawn += 1;
        }
    }

    drawn
}

pub fn move_to_top(game: &mut GameState, card_id: impl HasCardId) {
    game.zones.move_card(card_id.card_id(), Zone::Library);
}

pub fn move_all_to_top<'a>(game: &mut GameState, cards: impl Iterator<Item = &'a CardId>) {
    for card_id in cards {
        move_to_top(game, card_id);
    }
}
