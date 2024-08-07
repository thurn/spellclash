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
use data::card_states::zones::{ToCardId, ZoneQueries};
use data::core::numerics::Damage;
use data::game_states::game_state::GameState;
use data::game_states::state_based_event::StateBasedEvent;
use data::printed_cards::printed_card::Face;
use primitives::game_primitives::{
    CardId, HasSource, PermanentId, Source, Zone, ALL_POSSIBLE_PLAYERS,
};
use tracing::debug;
use utils::outcome;
use utils::outcome::Outcome;

use crate::mutations::move_card;

/// Turns the [Face] face of this card up and reveals it to all players.
///
/// Returns None if this card does not exist.
pub fn turn_face_up(
    game: &mut GameState,
    _source: impl HasSource,
    id: impl ToCardId,
    face: Face,
) -> Outcome {
    let card = game.card_mut(id)?;
    card.facing = CardFacing::FaceUp(face);
    card.revealed_to = ALL_POSSIBLE_PLAYERS;
    outcome::OK
}

/// Taps a permanent.
///
/// Returns None if this card does not exist.
pub fn tap(game: &mut GameState, _source: impl HasSource, id: impl ToCardId) -> Outcome {
    let card = game.card_mut(id)?;
    card.tapped_state = TappedState::Tapped;
    outcome::OK
}

/// Untaps a permanent
///
/// Returns None if this card does not exist.
pub fn untap(game: &mut GameState, _source: impl HasSource, id: impl ToCardId) -> Outcome {
    let card = game.card_mut(id)?;
    card.tapped_state = TappedState::Untapped;
    outcome::OK
}

/// Deals damage to a permanent
///
/// Returns None if this card does not exist.
pub fn deal_damage(
    game: &mut GameState,
    _source: impl HasSource,
    id: impl ToCardId,
    damage: Damage,
) -> Outcome {
    let card = game.card_mut(id)?;
    let permanent_id = card.permanent_id()?;
    debug!("Dealing {damage:?} damage to {id:?}");
    card.damage += damage;
    game.add_state_based_event(StateBasedEvent::CreatureDamaged(permanent_id));
    outcome::OK
}

/// Sacrifices a permanent.
///
/// Returns None if this card does not exist.
pub fn sacrifice(game: &mut GameState, source: impl HasSource, id: impl ToCardId) -> Outcome {
    move_card::run(game, source.source(), id, Zone::Graveyard)
}

/// Returns a permanent to its owner's hand.
pub fn return_to_hand(game: &mut GameState, source: impl HasSource, id: impl ToCardId) -> Outcome {
    move_card::run(game, source.source(), id, Zone::Hand)
}
