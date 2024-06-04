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

use std::iter;

use data::card_states::play_card_plan::CastSpellChoices;
use data::card_states::zones::ZoneQueries;
use data::core::numerics::{Power, Toughness};
use data::core::primitives::{CardId, CardType, Zone};
use data::game_states::game_state::GameState;
use data::printed_cards::card_subtypes::LandSubtype;
use data::printed_cards::layout::CardLayout;
#[allow(unused)] // Used in docs
use data::printed_cards::mana_cost::{ManaCost, ManaCostItem};
use data::printed_cards::printed_card::{Face, PrintedCardFace};
use data::printed_cards::printed_primitives::{PrintedPower, PrintedToughness};
use either::Either;
use enumset::EnumSet;

/// Returns the list of [PrintedCardFace]s for a card which currently define its
/// characteristics.
///
/// - If this card is on the battlefield, this is the face-up face of the card.
///
/// - If this card is on the stack, this is the face or faces that were cast.
///
/// - If this is a split card on the stack cast with the Fuse ability, this will
///   be a list of both faces:
///
///   > 702.102b. A fused split spell has the combined characteristics of its
///   > two halves. (See rule 709.4.)
///
///   <https://yawgatog.com/resources/magic-rules/#R702102b>
///
/// - If this is a split card which is not on the stack, this will be a list of
///   both faces:
///
///   > 709.4. In every zone except the stack, the characteristics of a split
///   > card are those of its two halves combined.
///
///   <https://yawgatog.com/resources/magic-rules/#R7094>
///
/// - If this is a double-faced card which is not on the stack or on the
///   battlefield, this will be the front face:
///
///   > 712.8a. While a double-faced card is outside the game or in a zone other
///   > than the battlefield or stack, it has only the characteristics of its
///   > front face.
///
///   <https://yawgatog.com/resources/magic-rules/#R7128a>
///
/// - If this is an Adventurer card which is not on the stack this will be the
///   primary face:
///
///   > 715.4. In every zone except the stack, and while on the stack not as an
///   > Adventure, an adventurer card has only its normal characteristics.
///
///   <https://yawgatog.com/resources/magic-rules/#R7154>
///
/// - If this is a card with Aftermath that is not on the stack, this will be a
///   list of both faces combined:
///
///   > 702.127a. Aftermath is an ability found on some split cards (see rule
///   > 709, "Split Cards"). It represents three static abilities. "Aftermath"
///   > means "You may cast this half of this split card from your graveyard,"
///   > "This half of this split card can't be cast from any zone other than a
///   > graveyard," and "If this spell was cast from a graveyard, exile it
///   > instead of putting it anywhere else any time it would leave the stack."
///
///   <https://yawgatog.com/resources/magic-rules/#R702127a>
///
/// - If this is a face-down card on the battlefield, this will be an empty
///   list:
///
///   > 708.2. Face-down spells and face-down permanents have no characteristics
///   > other than those listed by the ability or rules that allowed the spell
///   > or permanent to be face down.
///
///   <https://yawgatog.com/resources/magic-rules/#R7082>
pub fn characteristic_faces(game: &GameState, card_id: CardId) -> Vec<&PrintedCardFace> {
    let card = game.card(card_id);
    match card.zone {
        Zone::Battlefield => card.face_up_printed_face().map_or_else(Vec::new, |face| vec![face]),
        Zone::Stack => card.cast_as.iter().map(|face| card.printed().face(face)).collect(),
        _ => match card.printed().layout {
            CardLayout::Split | CardLayout::Aftermath => card.printed().all_faces().collect(),
            _ => vec![&card.printed().face],
        },
    }
}

/// Returns the set of card types on a card's characteristic faces.
///
/// See [characteristic_faces] for more information.
pub fn card_types(game: &GameState, card_id: CardId) -> EnumSet<CardType> {
    characteristic_faces(game, card_id).iter().flat_map(|face| face.card_types.iter()).collect()
}

/// Returns the set of land subtypes on a card's characteristic faces.
///
/// See [characteristic_faces] for more information.
pub fn land_subtypes(game: &GameState, card_id: CardId) -> EnumSet<LandSubtype> {
    characteristic_faces(game, card_id).iter().flat_map(|face| face.subtypes.land.iter()).collect()
}

/// Returns the [ManaCost] that needs to be paid to cast the [CardId] card using
/// the provided [CastSpellChoices]. Cost items are sorted in [ManaCostItem]
/// order.
///
/// Panics if invalid choices are selected, e.g. if the selected card
/// face does not exist.
pub fn mana_cost_for_casting_card(
    game: &GameState,
    card_id: CardId,
    choices: &CastSpellChoices,
) -> ManaCost {
    let mut cost =
        game.card(card_id).printed().face(choices.play_as.single_face()).mana_cost.clone();
    cost.items.sort();
    cost
}

/// Computes the power on card's characteristic faces.
///
/// See [characteristic_faces] for more information.
pub fn power(game: &GameState, card_id: CardId) -> Power {
    let characteristic = characteristic_faces(game, card_id);
    match characteristic[..] {
        [] => {
            // > 708.2a. If a face-up permanent is turned face down by a spell or ability that
            // > doesn't list any characteristics for that object, it becomes a 2/2 face-down
            // > creature with no text, no name, no subtypes, and no mana cost.
            // <https://yawgatog.com/resources/magic-rules/#R7082a>
            2
        }
        [face] => match face.power {
            Some(PrintedPower::Number(p)) => p,
            _ => todo!("Implement support for non-numeric power"),
        },
        _ => panic!("Cannot compute power for card with multiple active faces"),
    }
}

/// Computes the toughness on card's characteristic faces.
///
/// See [characteristic_faces] for more information.
pub fn toughness(game: &GameState, card_id: CardId) -> Toughness {
    let characteristic = characteristic_faces(game, card_id);
    match characteristic[..] {
        [] => {
            // > 708.2a. If a face-up permanent is turned face down by a spell or ability that
            // > doesn't list any characteristics for that object, it becomes a 2/2 face-down
            // > creature with no text, no name, no subtypes, and no mana cost.
            // <https://yawgatog.com/resources/magic-rules/#R7082a>
            2
        }
        [face] => match face.toughness {
            Some(PrintedToughness::Number(t)) => t,
            _ => todo!("Implement support for non-numeric toughness"),
        },
        _ => panic!("Cannot compute toughness for card with multiple active faces"),
    }
}
