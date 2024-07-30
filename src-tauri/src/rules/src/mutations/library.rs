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

use data::card_states::zones::{ToCardId, ZoneQueries};
use data::game_states::game_state::GameState;
use data::game_states::state_based_event::StateBasedEvent;
use primitives::game_primitives::{CardId, HasPlayerName, HasSource, PlayerName, Zone};
use utils::outcome;
use utils::outcome::Outcome;

use crate::mutations::move_card;

/// Draws a card from the top of the `player`'s library.
///
/// Marks the card as revealed to its owner.
///
/// Attempting to draw from an empty library will add a [StateBasedEvent] marker
/// which will cause the player to lose the game the next time state-based
/// actions are checked.
pub fn draw(game: &mut GameState, source: impl HasSource, player: impl HasPlayerName) -> Outcome {
    let player = player.player_name();
    let Some(&id) = game.library(player).back() else {
        game.add_state_based_event(StateBasedEvent::DrawFromEmptyLibrary(player));
        return outcome::OK;
    };
    move_card::run(game, source, id, Zone::Hand)
}

/// Draws `count` cards in sequence from the top of the `player`'s library.
///
/// Events are fired one at a time for each individual draw.
pub fn draw_cards(
    game: &mut GameState,
    source: impl HasSource,
    player: impl HasPlayerName,
    count: usize,
) -> Outcome {
    let player = player.player_name();
    let source = source.source();
    for _ in 0..count {
        draw(game, source, player)?;
    }
    outcome::OK
}

/// Move a card to the top of its owner's library.
pub fn move_to_top(
    game: &mut GameState,
    source: impl HasSource,
    card_id: impl ToCardId,
) -> Outcome {
    move_card::run(game, source, card_id, Zone::Library)
}

/// Moves all provided cards to the top of their owner's library in the given
/// order.
///
/// Cards in the list which no longer exist will be ignored.
pub fn move_all_to_top<'a>(
    game: &mut GameState,
    source: impl HasSource,
    cards: impl IntoIterator<Item = &'a CardId>,
) {
    let source = source.source();
    for card_id in cards {
        move_to_top(game, source, *card_id);
    }
}
