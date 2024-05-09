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

use data::card_states::card_kind::CardKind;
use data::card_states::card_state::CardState;
use data::card_states::zones::ZoneQueries;
use data::core::primitives::{CardId, CardType, PlayerName};
use data::game_states::game_state::GameState;
use data::printed_cards::layout::CardLayout;
use data::printed_cards::printed_card::{Face, PrintedCardFace};

use crate::queries::players;

/// Describes how a face of card can be played.
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum CanPlayAs {
    Land(Face),
    Instant(Face),
    Sorcery(Face),
}

/// Returns true if the [PlayerName] player can currently legally play any face
/// of the indicated [CardId].
pub fn any_face(game: &GameState, player: PlayerName, card: CardId) -> bool {
    play_as(game, player, card).next().is_some()
}

/// Returns an iterator over currently-legal options for playing a face of the
/// card with the indicated [CardId].
///
/// Tokens, emblems, abilities, and copies of cards on the stack cannot be
/// played.
pub fn play_as(
    game: &GameState,
    player: PlayerName,
    card: CardId,
) -> impl Iterator<Item = CanPlayAs> {
    let card = game.card(card);

    let face_a = if card.kind == CardKind::Normal {
        can_play_as(game, player, card, &card.printed().face)
    } else {
        None
    };

    let face_b = match (card.printed().layout, &card.printed().face_b) {
        // > 709.3. A player chooses which half of a split card they are casting before putting it
        // > onto the stack.
        // <https://yawgatog.com/resources/magic-rules/#R7093>
        //
        // > 712.11b. A player casting a modal double-faced card or a copy of a modal double-faced
        // > card as a spell chooses which face they are casting before putting it onto the stack.
        // https://yawgatog.com/resources/magic-rules/#R71211b
        //
        // > 715.3. As a player casts an adventurer card, the player chooses whether they cast the
        // > card normally or as an Adventure.
        // https://yawgatog.com/resources/magic-rules/#R7153
        (CardLayout::Split, Some(face_b))
        | (CardLayout::ModalDfc, Some(face_b))
        | (CardLayout::Adventure, Some(face_b)) => can_play_as(game, player, card, face_b),
        _ => None,
    };

    face_a.into_iter().chain(face_b)
}

/// Returns a [CanPlayAs] indicating whether a [PlayerName] can play a given
/// [PrintedCardFace] of a [CardState] in the current [GameState].
fn can_play_as(
    game: &GameState,
    player: PlayerName,
    card: &CardState,
    face: &PrintedCardFace,
) -> Option<CanPlayAs> {
    if player != card.controller {
        return None;
    }

    let result = can_play_as_for_types(face);
    match result {
        CanPlayAs::Land(_) => {
            if in_main_phase_with_stack_empty(game, player)
                && players::land_plays_remaining(game, player) > 0
            {
                return Some(result);
            }
        }
        CanPlayAs::Instant(_) => {
            if game.priority == player {
                return Some(result);
            }
        }
        CanPlayAs::Sorcery(_) => {
            if in_main_phase_with_stack_empty(game, player) {
                return Some(result);
            }
        }
    }

    None
}

/// Returns true if the indicated player is currently the active player, in
/// their main phase, with the stack empty, while they have priority.
fn in_main_phase_with_stack_empty(game: &GameState, player: PlayerName) -> bool {
    game.stack().is_empty()
        && game.step.is_main_phase()
        && game.turn.active_player == player
        && game.priority == player
}

/// Returns a [CanPlayAs] for a card solely based on its card types.
fn can_play_as_for_types(face: &PrintedCardFace) -> CanPlayAs {
    if face.card_types.contains(CardType::Instant) {
        CanPlayAs::Instant(face.face_identifier)
    } else if face.card_types.contains(CardType::Land) {
        CanPlayAs::Land(face.face_identifier)
    } else {
        CanPlayAs::Sorcery(face.face_identifier)
    }
}
