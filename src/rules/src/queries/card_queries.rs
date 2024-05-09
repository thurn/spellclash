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

use data::card_states::play_card_plan::CastSpellChoices;
use data::card_states::zones::ZoneQueries;
use data::core::primitives::CardId;
use data::game_states::game_state::GameState;
use data::printed_cards::card_subtypes::LandSubtype;
#[allow(unused)] // Used in docs
use data::printed_cards::mana_cost::{ManaCost, ManaCostItem};
use enumset::EnumSet;
use utils::outcome::Value;

/// Returns the land subtypes on the face-up face of a card, or the empty set if
/// the card is face-down.
pub fn land_subtypes_for_face(game: &GameState, card_id: CardId) -> EnumSet<LandSubtype> {
    game.card(card_id).printed_face().map(|face| face.subtypes.land).unwrap_or_default()
}

/// Returns the [ManaCost] that needs to be paid to cast the [CardId] card using
/// the provided [CastSpellChoices]. Cost items are sorted in [ManaCostItem]
/// order.
///
/// Returns an error if invalid choices are selected, e.g. if the selected card
/// face does not exist.
pub fn mana_cost_for_casting_card(
    game: &GameState,
    card_id: CardId,
    choices: &CastSpellChoices,
) -> Value<ManaCost> {
    let mut cost = game.card(card_id).printed().face(choices.face)?.mana_cost.clone();
    cost.items.sort();
    Ok(cost)
}
