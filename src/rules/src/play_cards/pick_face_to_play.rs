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
use data::card_states::play_card_plan::{CastSpellChoices, ManaPaymentPlan, PlayCardPlan};
use data::card_states::zones::ZoneQueries;
use data::core::primitives::{CardId, CardType, PlayerName, Source};
use data::game_states::game_state::GameState;
use data::printed_cards::layout::CardLayout;
use data::printed_cards::printed_card::{Face, PrintedCardFace};

use crate::play_cards::play_card::PlayCardStep;
use crate::play_cards::play_card_choices::{PlayCardChoice, PlayCardChoicePrompt};
use crate::queries::players;

pub fn run(
    game: &GameState,
    source: Source,
    card_id: CardId,
    plan: &PlayCardPlan,
) -> PlayCardChoice {
    let mut valid_faces = vec![];
    let card = game.card(card_id);
    if can_play_as(game, card, &card.printed().face).is_some() {
        valid_faces.push(Face::Primary);
    }

    if let (CardLayout::Split, Some(face_b))
    | (CardLayout::ModalDfc, Some(face_b))
    | (CardLayout::Adventure, Some(face_b)) = (card.printed().layout, &card.printed().face_b)
    {
        if can_play_as(game, card, face_b).is_some() {
            valid_faces.push(Face::FaceB);
        }
    };

    match valid_faces[..] {
        [] => PlayCardChoice::Invalid,
        [face] => PlayCardChoice::Continue {
            updated_plan: PlayCardPlan {
                spell_choices: CastSpellChoices { face, ..plan.spell_choices.clone() },
                mana_payment: ManaPaymentPlan::default(),
            },
        },
        _ => PlayCardChoice::Prompt {
            optional: false,
            prompt: PlayCardChoicePrompt::SelectFace { valid_faces },
        },
    }
}

/// Describes how a face of card can be played.
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum CanPlayAs {
    Land(Face),
    Instant(Face),
    Sorcery(Face),
}

/// Returns a [CanPlayAs] indicating whether a [PlayerName] can play a given
/// [PrintedCardFace] of a [CardState] in the current [GameState].
fn can_play_as(game: &GameState, card: &CardState, face: &PrintedCardFace) -> Option<CanPlayAs> {
    let player = card.controller;
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
