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
use utils::outcome;
use utils::outcome::Outcome;

/// Draws a card from the top of the `player`'s library.
///
/// Marks the card as revealed to its owner. Returns `outcome::GAME_OVER` if
/// this causes the game to end due to drawing from an empty library.
pub fn draw(game: &mut GameState, player: impl HasPlayerName, _source: impl HasSource) -> Outcome {
    let Some(&id) = game.library(player).back() else { return outcome::GAME_OVER };
    let card = game.card_mut(id);
    card.revealed_to.insert(card.owner);
    game.zones.move_card(id, Zone::Hand)
}

/// Draws `count` cards in sequence from the top of the `player`'s library.
///
/// Events are fired one at a time for each individual draw. Returns
/// `outcome::GAME_OVER` if this causes the game to end due to drawing from an
/// empty library.
pub fn draw_cards(
    game: &mut GameState,
    player: impl HasPlayerName,
    source: impl HasSource,
    count: usize,
) -> Outcome {
    let p = player.player_name();
    let s = source.source();
    for _ in 0..count {
        draw(game, p, s)?;
    }
    outcome::OK
}

pub fn move_to_top(game: &mut GameState, card_id: impl HasCardId) -> Outcome {
    game.zones.move_card(card_id.card_id(), Zone::Library)
}

pub fn move_all_to_top<'a>(
    game: &mut GameState,
    cards: impl Iterator<Item = &'a CardId>,
) -> Outcome {
    for card_id in cards {
        move_to_top(game, card_id)?;
    }
    outcome::OK
}
