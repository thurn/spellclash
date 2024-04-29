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
use mtgjson::Layout;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::core::card_subtypes::CardSubtypes;
use crate::core::primitives::{
    AttractionLight, CardSupertype, CardType, Color, ManaValue, PrintedLoyalty, PrintedPower,
    PrintedToughness,
};

/// Represents the immutable data about a card printing. This should generally
/// correspond to the definition at <https://mtgjson.com/data-models/card/card-set/>
#[derive(Clone, Debug, Serialize, Deserialize)]
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
    /// Note that for some cards with the 'meld' mechanic, two different cards
    /// will share copies of the same back face data.
    pub face_b: Option<PrintedCardFace>,
}

/// Represents one face of a printed card.
///
/// See the comments in [PrintedCard] for more information.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PrintedCardFace {
    /// MTG JSON identifier for this face
    pub id: Uuid,
    /// The name for this face.
    pub name: String,
    /// The set of face supertypes
    pub supertypes: EnumSet<CardSupertype>,
    /// The set of all card types of the face
    pub card_types: EnumSet<CardType>,
    /// The set of subtypes for this face, found after em-dash.
    pub subtypes: CardSubtypes,
    /// The rules text for this face.
    pub oracle_text: Option<String>,
    /// Colors of this face. This incorporates the mana cost and color
    /// indicators as well as static rules text modifiers like Devoid.
    ///
    /// See <https://yawgatog.com/resources/magic-rules/#R2022>
    pub colors: EnumSet<Color>,
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
    /// Card layout, describing its printed structure. For tokens this will be
    /// "Token".
    pub layout: Layout,
    /// A list of attraction lights found on a card, available only to cards
    /// printed in certain Un-sets.
    pub attraction_lights: EnumSet<AttractionLight>,
    /// Another card which this face can be melded with. Both faces combine into
    /// their shared back face.
    pub melds_with: Option<Uuid>,
    /// True if the card allows a value other than 4 copies in a deck.
    pub has_alternative_deck_limit: bool,
}
