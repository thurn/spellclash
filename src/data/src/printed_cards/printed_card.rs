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

use enumset::EnumSet;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::core::numerics::ManaValue;
use crate::core::primitives::{CardSupertype, CardType, Color};
use crate::printed_cards::card_subtypes::CardSubtypes;
use crate::printed_cards::layout::{CardLayout, FaceLayout};
use crate::printed_cards::mana_cost::ManaCost;
use crate::printed_cards::printed_primitives::{
    AttractionLight, PrintedLoyalty, PrintedPower, PrintedToughness,
};

#[derive(Copy, Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub enum Face {
    Primary,
    FaceB,
}

/// Represents the immutable data about a card.
///
/// This describes the physical information printed on a card. It should
/// generally correspond to the definition at <https://mtgjson.com/data-models/card/card-set/>
#[derive(Clone, Debug)]
pub struct PrintedCard {
    /// The primary face of the card. This represents:
    ///
    /// - The card information on all normal cards
    /// - The front face of double-faced cards
    /// - The left face of split cards
    /// - The larger primary face of adventure cards and aftermath cards
    /// - The primary or starting face of flip cards
    pub face: PrintedCardFace,

    /// An additional face for this card, if present. This represents:
    ///
    /// - The back of a double-faced card
    /// - The right face of a split card
    /// - The smaller secondary face of an adventure or aftermath card
    /// - The secondary or 'flipped' face of a flip card
    ///
    /// Note that for cards with the 'meld' mechanic, two different cards will
    /// share copies of the same back face data.
    pub face_b: Option<PrintedCardFace>,

    /// Describes how the multiple faces of the card are organized in relation
    /// to each other.
    ///
    /// See <https://scryfall.com/docs/api/layouts>
    pub layout: CardLayout,
}

impl PrintedCard {
    /// Returns the named face of this card
    pub fn face(&self, face: Face) -> Option<&PrintedCardFace> {
        match face {
            Face::Primary => Some(&self.face),
            Face::FaceB => self.face_b.as_ref(),
        }
    }
}

/// Represents one face of a printed card.
///
/// See the comments in [PrintedCard] for more information.
#[derive(Clone, Debug)]
pub struct PrintedCardFace {
    /// MTG JSON identifier for this face
    pub id: Uuid,
    /// The name for this face.
    pub name: String,
    /// Identifier for this face.
    pub face_identifier: Face,
    /// Different printings of this card face
    pub variants: Vec<PrintedCardFaceVariant>,
    /// The set of face supertypes
    pub supertypes: EnumSet<CardSupertype>,
    /// The set of all card types of the face
    pub card_types: EnumSet<CardType>,
    /// The set of subtypes for this face, found after em-dash.
    pub subtypes: CardSubtypes,
    /// The rules text_strings for this face.
    pub oracle_text: Option<String>,
    /// Colors of this face. This incorporates the mana cost and color
    /// indicators as well as static rules text_strings modifiers like Devoid.
    ///
    /// See <https://yawgatog.com/resources/magic-rules/#R2022>
    pub colors: EnumSet<Color>,
    /// The mana cost for this face.
    ///
    /// See <https://yawgatog.com/resources/magic-rules/#R1074>
    pub mana_cost: ManaCost,
    /// The static mana value of this face.
    ///
    /// This is the printed mana value of the face itself. Note that mana value
    /// on the stack can be different, for example in the case of cards with
    /// X in their casting cost.
    ///
    /// See <https://yawgatog.com/resources/magic-rules/#R2023>
    pub mana_value: ManaValue,
    /// The printed power of the card.
    pub power: Option<PrintedPower>,
    /// The printed toughness of the card.
    pub toughness: Option<PrintedToughness>,
    /// Starting loyalty value for a Planeswalker card
    pub loyalty: Option<PrintedLoyalty>,
    /// Face layout, describing its printed structure.
    ///
    /// See <https://scryfall.com/docs/api/layouts> for descriptions & examples of layouts.
    pub layout: FaceLayout,
    /// A list of attraction lights found on a card, available only to cards
    /// printed in certain Un-sets.
    pub attraction_lights: EnumSet<AttractionLight>,
    /// Another card which this face can be melded with. Both faces combine into
    /// their identical `face_b` back face.
    pub melds_with: Option<Uuid>,
    /// True if the card allows a value other than 4 copies in a deck.
    pub has_alternative_deck_limit: bool,
}

/// A visually unique printed version of a card face.
#[derive(Clone, Debug)]
pub struct PrintedCardFaceVariant {
    pub scryfall_id: Uuid,
}
