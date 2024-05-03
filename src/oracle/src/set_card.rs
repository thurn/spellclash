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

use data::printed_cards::layout::FaceLayout;
use mtgjson::{Color, Identifiers, Side};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Represents a card printed in a set.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SetCard {
    /// The name of the artist that illustrated the card art.
    pub artist: Option<String>,

    /// The attraction lights on the card.
    pub attraction_lights: Option<Vec<u8>>,

    /// The related parts of the card.
    pub card_parts: Option<Vec<String>>,

    /// A list of all the colors found in `mana_cost`, `color_indicator`, and
    /// `text`.
    pub color_identity: Vec<Color>,

    /// A list of all the colors in the color indicator (The symbol prefixed to
    /// a card's types).
    pub color_indicator: Option<Vec<Color>>,

    /// A list of all the colors in `mana_cost` and `color_indicator`. Some
    /// cards may not have values, such as cards with "Devoid" in its text.
    pub colors: Vec<Color>,

    /// The card rank on EDHRec.
    pub edhrec_rank: Option<u32>,

    /// The flavor name on the face of the card.
    pub face_flavor_name: Option<String>,

    /// The mana value of the face for either half or part of the card. Formally
    /// known as "converted mana cost".
    pub face_mana_value: Option<f32>,

    /// The name on the face of the card.
    pub face_name: Option<String>,

    /// The promotional card name printed above the true card name on special
    /// cards that has no game function.
    pub flavor_name: Option<String>,

    /// The italicized text found below the rules text that has no game
    /// function.
    pub flavor_text: Option<String>,

    /// If the card allows a value other than 4 copies in a deck.
    pub has_alternative_deck_limit: Option<bool>,

    /// A list of identifiers associated to a card.
    pub identifiers: Identifiers,

    /// If the card is an alternate variation to an original printing.
    pub is_alternative: Option<bool>,

    /// If the card has full artwork.
    pub is_full_art: Option<bool>,

    /// If the card is part of a funny set.
    pub is_funny: Option<bool>,

    /// If the card is only available in online game variations.
    pub is_online_only: Option<bool>,

    /// If the card is oversized.
    pub is_oversized: Option<bool>,

    /// If the card is a promotional printing.
    pub is_promo: Option<bool>,

    /// If the card is rebalanced for the Alchemy play format.
    pub is_rebalanced: Option<bool>,

    /// If the card has been reprinted.
    pub is_reprint: Option<bool>,

    /// If the card is on the Magic: The Gathering Reserved List.
    pub is_reserved: Option<bool>,

    /// If the card is found in a starter deck such as Planeswalker/Brawl decks.
    pub is_starter: Option<bool>,

    /// If the card is a Story Spotlight card.
    pub is_story_spotlight: Option<bool>,

    /// If the card does not have a text box.
    pub is_textless: Option<bool>,

    /// If the card is "timeshifted", a feature of certain sets where a card
    /// will have a different `frame_version`.
    pub is_timeshifted: Option<bool>,

    /// A list of keywords found on the card.
    pub keywords: Option<Vec<String>>,

    /// The type of card layout. For a token card, this will be "token".
    pub layout: FaceLayout,

    /// The starting life total modifier. A plus or minus character precedes an
    /// integer. Used only on cards with "Vanguard" in its types.
    pub life: Option<String>,

    /// The starting loyalty value of the card. Used only on cards with
    /// "Planeswalker" in its types.
    pub loyalty: Option<String>,

    /// The mana cost of the card wrapped in brackets for each value.
    pub mana_cost: Option<String>,

    /// The mana value of the card. Formally known as "converted mana cost".
    pub mana_value: f32,

    /// The name of the card. Cards with multiple faces, like "Split" and "Meld"
    /// cards are given a delimiter.
    pub name: String,

    /// The number of the card. Can be prefixed or suffixed with a * or other
    /// characters for promotional sets.
    pub number: String,

    /// The text on the card as originally printed.
    pub original_text: Option<String>,

    /// The type of the card as originally printed. Includes any supertypes and
    /// subtypes.
    pub original_type: Option<String>,

    /// A list of card UUID's to this card's counterparts, such as transformed
    /// or melded faces.
    pub other_face_ids: Option<Vec<Uuid>>,

    /// The power of the card.
    pub power: Option<String>,

    /// A list of set printing codes the card was printed in, formatted in
    /// uppercase.
    pub printings: Option<Vec<String>>,

    /// Rebalanced digital printings.
    pub rebalanced_printings: Option<Vec<Uuid>>,

    /// Reverse related cards.
    pub reverse_related: Option<Vec<String>>,

    /// The set printing code that the card is from.
    pub set_code: String,

    /// The identifier of the card side. Used on cards with multiple faces on
    /// the same card.
    pub side: Option<Side>,

    /// The name of the signature on the card.
    pub signature: Option<String>,

    /// The subset of the card.
    pub subset: Option<Vec<String>>,

    /// A list of card subtypes found after em-dash.
    pub subtypes: Vec<String>,

    /// A list of card supertypes found before em-dash.
    pub supertypes: Vec<String>,

    /// The rules text of the card.
    pub text: Option<String>,

    /// The toughness of the card.
    pub toughness: Option<String>,

    /// The type of the card as visible, including any supertypes and subtypes.
    #[serde(rename = "type")]
    pub card_type: String,

    /// A list of all card types of the card, including Un‑sets and gameplay
    /// variants.
    #[serde(rename = "types")]
    pub card_types: Vec<String>,

    /// The universal unique identifier (v5) generated by MTGJSON. Each entry is
    /// unique.
    pub uuid: Uuid,

    /// The name of the watermark on the card.
    pub watermark: Option<String>,
}
