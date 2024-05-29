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

use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Represents a card face as it appears in the MTGJSON card database.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DatabaseCardFace {
    /// The name of the artist that illustrated the card art.
    pub artist: Option<String>,

    /// The attraction lights on the card.
    pub attraction_lights: Option<String>,

    /// A list of all the colors in `mana_cost` and
    /// `color_indicator`. Some cards may not have
    /// values, such as cards with "Devoid" in its text.
    pub colors: Option<String>,

    /// The name on the face of the card.
    pub face_name: Option<String>,

    /// The type of card layout. For a token card, this will be
    /// "token".
    pub layout: String,

    /// The starting loyalty value of the card. Used only on cards
    /// with "Planeswalker" in its types.
    pub loyalty: Option<String>,

    /// The mana cost of the card wrapped in brackets for each
    /// value.
    pub mana_cost: Option<String>,

    /// The mana value of the card. Formally known as "converted
    /// mana cost".
    pub mana_value: f32,

    /// The name of the card. Cards with multiple faces, like
    /// "Split" and "Meld" cards are given a
    /// delimiter.
    pub name: String,

    /// The power of the card.
    pub power: Option<String>,

    /// Identifier for this card as a named rules entity within the Scryfall
    /// database.
    pub scryfall_oracle_id: Uuid,

    /// Identifies this face within a multi-face card.
    pub side: Option<String>,

    /// A list of card subtypes found after em-dash.
    pub subtypes: Option<String>,

    /// A list of card supertypes found before em-dash.
    pub supertypes: Option<String>,

    /// The rules text of the card.
    pub text: Option<String>,

    /// The toughness of the card.
    pub toughness: Option<String>,

    /// A list of all card types of the card, including Un‑sets and
    /// gameplay variants.
    pub types: Option<String>,

    /// The universal unique identifier (v5) generated by MTGJSON.
    /// Each entry is unique.
    pub uuid: Uuid,
}