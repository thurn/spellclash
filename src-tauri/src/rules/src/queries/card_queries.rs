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
use data::card_states::play_card_plan::PlayCardPlan;
use data::card_states::zones::{ToCardId, ZoneQueries};
use data::core::numerics::{Power, Toughness};
use data::game_states::game_state::GameState;
use data::printed_cards::card_subtypes::{CreatureType, LandType};
use data::printed_cards::layout::CardLayout;
#[allow(unused)] // Used in docs
use data::printed_cards::mana_cost::{ManaCost, ManaCostItem};
use data::printed_cards::printed_card::{Face, PrintedCardFace};
use data::printed_cards::printed_primitives::{PrintedPower, PrintedToughness};
use enumset::EnumSet;
use primitives::game_primitives::{CardId, CardType, Color, Source, Zone};

pub enum CharacteristicFaces<'a> {
    FaceDown,
    Face(&'a PrintedCardFace),
    MultipleFaces(Vec<&'a PrintedCardFace>),
}

/// Returns the [PrintedCardFace]s for a card which currently define its
/// characteristics. Returns None if this card no longer exists.
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
pub fn characteristic_faces(
    game: &GameState,
    _: Source,
    id: impl ToCardId,
) -> Option<CharacteristicFaces> {
    let card = game.card(id)?;
    Some(match card.zone {
        Zone::Battlefield => {
            if let Some(face) = card.face_up_printed_face() {
                CharacteristicFaces::Face(face)
            } else {
                CharacteristicFaces::FaceDown
            }
        }
        Zone::Stack => {
            let cast_as = cast_as_faces(card);
            if cast_as.len() == 1 {
                CharacteristicFaces::Face(card.printed().face(cast_as.iter().next().unwrap()))
            } else {
                CharacteristicFaces::MultipleFaces(
                    cast_as.iter().map(|face| card.printed().face(face)).collect(),
                )
            }
        }
        _ => match card.printed().layout {
            CardLayout::Split | CardLayout::Aftermath => {
                CharacteristicFaces::MultipleFaces(card.printed().all_faces().collect())
            }
            _ => CharacteristicFaces::Face(&card.printed().face),
        },
    })
}

/// Returns the set of faces used to cast the indicated card, or an empty set if
/// this card has not been cast.
pub fn cast_as_faces(card: &CardState) -> EnumSet<Face> {
    card.cast_choices.as_ref().map(|choices| choices.play_as.faces).unwrap_or_default()
}

/// Returns the set of current card types on a card's characteristic faces.
/// Returns None if this card no longer exists.
///
/// See [characteristic_faces] for more information.
pub fn card_types(
    game: &GameState,
    source: Source,
    id: impl ToCardId,
) -> Option<EnumSet<CardType>> {
    Some(match characteristic_faces(game, source, id)? {
        CharacteristicFaces::FaceDown => EnumSet::new(),
        CharacteristicFaces::Face(face) => face.card_types,
        CharacteristicFaces::MultipleFaces(faces) => {
            faces.iter().flat_map(|face| face.card_types.iter()).collect()
        }
    })
}

/// Returns the set of current land subtypes on a card's characteristic faces.
/// Returns None if this card no longer exists.
///
/// See [characteristic_faces] for more information.
pub fn land_subtypes(
    game: &GameState,
    source: Source,
    id: impl ToCardId,
) -> Option<EnumSet<LandType>> {
    let card = game.card(id)?;
    let types = match characteristic_faces(game, source, id)? {
        CharacteristicFaces::FaceDown => EnumSet::new(),
        CharacteristicFaces::Face(face) => face.subtypes.land,
        CharacteristicFaces::MultipleFaces(faces) => {
            faces.iter().flat_map(|face| face.subtypes.land.iter()).collect()
        }
    };
    Some(card.properties.land_types.query(game, source, types))
}

/// Returns the set of current creature subtypes on a card's characteristic
/// faces. Returns None if this card no longer exists.
///
/// Note that setting a creature's subtype does not remove subtypes for other
/// card types:
///
/// > 205.1a. ... Similarly, when an effect sets one or more of an object's
/// > subtypes, the new subtype(s) replaces any existing subtypes from the
/// > appropriate set  (creature types, land types, artifact types, enchantment
/// > types, planeswalker types, or spell types).
///
/// <https://yawgatog.com/resources/magic-rules/#R2051a>
///
/// See [characteristic_faces] for more information.
pub fn creature_subtypes(
    game: &GameState,
    source: Source,
    id: impl ToCardId,
) -> Option<EnumSet<CreatureType>> {
    let card = game.card(id)?;
    let types = match characteristic_faces(game, source, id)? {
        CharacteristicFaces::FaceDown => EnumSet::new(),
        CharacteristicFaces::Face(face) => face.subtypes.creature,
        CharacteristicFaces::MultipleFaces(faces) => {
            faces.iter().flat_map(|face| face.subtypes.creature.iter()).collect()
        }
    };
    Some(card.properties.creature_types.query(game, source, types))
}

/// Returns the current [ManaCost] that needs to be paid to cast the [CardId]
/// card using the provided [PlayCardPlan]. Cost items are sorted in
/// [ManaCostItem] order. Returns None if this card no longer exists.
pub fn mana_cost_for_casting_card(
    game: &GameState,
    id: CardId,
    plan: &PlayCardPlan,
) -> Option<ManaCost> {
    let mut cost =
        game.card(id)?.printed().face(plan.choices.play_as.single_face()).mana_cost.clone();
    cost.items.sort();
    Some(cost)
}

/// Computes the current power on a card's characteristic faces. Returns None if
/// this card no longer exists.
///
/// See [characteristic_faces] for more information.
pub fn power(game: &GameState, source: Source, id: impl ToCardId) -> Option<Power> {
    let card = game.card(id)?;
    let result = match characteristic_faces(game, source, card.id)? {
        CharacteristicFaces::FaceDown => {
            // > 708.2a. If a face-up permanent is turned face down by a spell or ability that
            // > doesn't list any characteristics for that object, it becomes a 2/2 face-down
            // > creature with no text, no name, no subtypes, and no mana cost.
            // <https://yawgatog.com/resources/magic-rules/#R7082a>
            2
        }
        CharacteristicFaces::Face(face) => match face.power {
            Some(PrintedPower::Number(p)) => p,
            _ => 0,
        },
        _ => panic!("Cannot compute power for card with multiple active faces"),
    };

    let base = card.properties.base_power.query(game, source, result);
    Some(card.properties.power.query(game, source, base))
}

/// Computes the current toughness on card's characteristic faces. Returns None
/// if this card no longer exists.
///
/// See [characteristic_faces] for more information.
pub fn toughness(game: &GameState, source: Source, id: impl ToCardId) -> Option<Toughness> {
    let card = game.card(id)?;
    let result = match characteristic_faces(game, source, card.id)? {
        CharacteristicFaces::FaceDown => {
            // > 708.2a. If a face-up permanent is turned face down by a spell or ability that
            // > doesn't list any characteristics for that object, it becomes a 2/2 face-down
            // > creature with no text, no name, no subtypes, and no mana cost.
            // <https://yawgatog.com/resources/magic-rules/#R7082a>
            2
        }
        CharacteristicFaces::Face(face) => match face.toughness {
            Some(PrintedToughness::Number(p)) => p,
            _ => 0,
        },
        _ => panic!("Cannot compute toughness for card with multiple active faces"),
    };

    let base = card.properties.base_toughness.query(game, source, result);
    Some(card.properties.toughness.query(game, source, base))
}

/// Returns the set of colors on a card's characteristic faces. Returns None if
/// this card no longer exists.
///
/// See [characteristic_faces] for more information.
pub fn colors(game: &GameState, source: Source, id: impl ToCardId) -> Option<EnumSet<Color>> {
    let card = game.card(id)?;
    let types = match characteristic_faces(game, source, id)? {
        CharacteristicFaces::FaceDown => EnumSet::new(),
        CharacteristicFaces::Face(face) => face.colors,
        CharacteristicFaces::MultipleFaces(faces) => {
            faces.iter().flat_map(|face| face.colors.iter()).collect()
        }
    };
    Some(card.properties.colors.query(game, source, types))
}
