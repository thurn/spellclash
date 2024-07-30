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
use data::card_states::play_card_plan::{PlayAs, PlayCardPlan, PlayCardTiming};
use data::card_states::zones::ZoneQueries;
use data::game_states::game_state::GameState;
use data::printed_cards::layout::CardLayout;
use data::printed_cards::printed_card::PrintedCardFace;
use enumset::EnumSet;
use primitives::game_primitives::{CardId, CardType, HasController, PlayerName, Source};

use crate::queries::player_queries;

/// Returns a list of [PlayCardPlan] options describing valid choices of faces
/// and timings to play of the indicated card.
///
/// The returned faces are selected for validity based solely on their timing
/// restrictions and the intrinsic properties of the card layout.
pub fn play_as(
    game: &GameState,
    player: PlayerName,
    source: Source,
    card_id: CardId,
) -> Vec<PlayCardPlan> {
    let mut valid_faces = vec![];
    let Some(card) = game.card(card_id) else {
        return vec![];
    };

    if let Some(play) = can_play_as(game, card, &card.printed().face) {
        valid_faces.push(play);
    }

    if let (CardLayout::Split, Some(face_b))
    | (CardLayout::ModalDfc, Some(face_b))
    | (CardLayout::Adventure, Some(face_b)) = (card.printed().layout, &card.printed().face_b)
    {
        if let Some(play) = can_play_as(game, card, face_b) {
            valid_faces.push(play);
        }
    };

    valid_faces.into_iter().map(|play_as| PlayCardPlan::new(player, play_as)).collect()
}

/// Returns a [CanPlayAs] indicating whether a [PlayerName] can play a given
/// [PrintedCardFace] of a [CardState] in the current [GameState].
fn can_play_as(game: &GameState, card: &CardState, face: &PrintedCardFace) -> Option<PlayAs> {
    let player = card.controller();
    let result = can_play_as_for_types(face);
    match result.timing {
        PlayCardTiming::Land => {
            if in_main_phase_with_stack_empty(game, player)
                && player_queries::land_plays_remaining(game, player) > 0
            {
                return Some(result);
            }
        }
        PlayCardTiming::Instant => {
            if game.priority == player {
                return Some(result);
            }
        }
        PlayCardTiming::Sorcery => {
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
fn can_play_as_for_types(face: &PrintedCardFace) -> PlayAs {
    if face.card_types.contains(CardType::Instant) {
        PlayAs { faces: EnumSet::only(face.face_identifier), timing: PlayCardTiming::Instant }
    } else if face.card_types.contains(CardType::Land) {
        PlayAs { faces: EnumSet::only(face.face_identifier), timing: PlayCardTiming::Land }
    } else {
        PlayAs { faces: EnumSet::only(face.face_identifier), timing: PlayCardTiming::Sorcery }
    }
}
