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
use data::card_states::zones::ZoneQueries;
use data::core::numerics::Damage;
use data::core::primitives::{CardId, Source, Zone, ALL_POSSIBLE_PLAYERS};
use data::game_states::game_state::GameState;
use data::game_states::state_based_event::StateBasedEvent;
use data::printed_cards::printed_card::Face;
use tracing::debug;
use utils::outcome::Outcome;
use utils::{outcome, verify};

/// Turns the [Face] face of this card up and reveals it to all players.
pub fn turn_face_up(game: &mut GameState, _source: Source, card: CardId, face: Face) -> Outcome {
    let card = game.card_mut(card);
    card.facing = CardFacing::FaceUp(face);
    card.revealed_to = ALL_POSSIBLE_PLAYERS;
    outcome::OK
}

/// Taps a card.
///
/// Returns an error if this card is already tapped or this card is not on the
/// battlefield.
pub fn tap(game: &mut GameState, _source: Source, card_id: CardId) -> Outcome {
    let card = game.card_mut(card_id);
    verify!(card.zone == Zone::Battlefield, "Card {card_id:?} is not on the battlefield");
    verify!(card.tapped_state == TappedState::Untapped, "Card {card_id:?} is already tapped");
    card.tapped_state = TappedState::Tapped;
    outcome::OK
}

/// Untaps a card
///
/// Returns an error if this card is not on the battlefield.
pub fn untap(game: &mut GameState, _source: Source, card_id: CardId) -> Outcome {
    let card = game.card_mut(card_id);
    verify!(card.zone == Zone::Battlefield, "Card {card_id:?} is not on the battlefield");
    card.tapped_state = TappedState::Untapped;
    outcome::OK
}

/// Deals damage to a permanent
pub fn deal_damage(
    game: &mut GameState,
    source: Source,
    card_id: CardId,
    damage: Damage,
) -> Outcome {
    debug!("Dealing {damage:?} damage to {card_id:?}");
    game.card_mut(card_id).damage += damage;
    game.add_state_based_event(StateBasedEvent::CreatureDamaged(card_id));
    outcome::OK
}
