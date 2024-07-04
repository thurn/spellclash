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

use data::card_states::zones::{ToCardId, ZoneQueries};
use data::core::primitives::{CardType, PermanentId, Source};
use data::delegates::scope::Scope;
use data::game_states::game_state::GameState;
use data::printed_cards::card_subtypes::LandSubtype;

use crate::queries::{card_queries, text_changes};

pub fn always_true(game: &GameState, source: Source, id: impl ToCardId) -> Option<bool> {
    Some(true)
}

pub fn creature(game: &GameState, source: Source, id: impl ToCardId) -> Option<bool> {
    Some(card_queries::card_types(game, source, id)?.contains(CardType::Creature))
}

pub fn planeswalker(game: &GameState, source: Source, id: impl ToCardId) -> Option<bool> {
    Some(card_queries::card_types(game, source, id)?.contains(CardType::Planeswalker))
}

pub fn battle(game: &GameState, source: Source, id: impl ToCardId) -> Option<bool> {
    Some(card_queries::card_types(game, source, id)?.contains(CardType::Battle))
}

/// Returns true if the given card is a plains.
///
/// If a text-changing effect has been applied to the card which produced
/// `source` to replace instances of a land subtype in its rules text, this will
/// check for the new land subtype instead.
pub fn plains(game: &GameState, source: Source, id: impl ToCardId) -> Option<bool> {
    Some(card_queries::land_subtypes(game, source, id)?.contains(text_changes::land_subtype(
        game,
        source,
        LandSubtype::Plains,
    )))
}

/// Returns true if the given card is an island.
///
/// If a text-changing effect has been applied to the card which produced
/// `source` to replace instances of a land subtype in its rules text, this will
/// check for the new land subtype instead.
pub fn island(game: &GameState, source: Source, id: impl ToCardId) -> Option<bool> {
    Some(card_queries::land_subtypes(game, source, id)?.contains(text_changes::land_subtype(
        game,
        source,
        LandSubtype::Island,
    )))
}

/// Returns true if the given card is a swamp.
///
/// If a text-changing effect has been applied to the card which produced
/// `source` to replace instances of a land subtype in its rules text, this will
/// check for the new land subtype instead.
pub fn swamp(game: &GameState, source: Source, id: impl ToCardId) -> Option<bool> {
    Some(card_queries::land_subtypes(game, source, id)?.contains(text_changes::land_subtype(
        game,
        source,
        LandSubtype::Swamp,
    )))
}

/// Returns true if the given card is a mountain.
///
/// If a text-changing effect has been applied to the card which produced
/// `source` to replace instances of a land subtype in its rules text, this will
/// check for the new land subtype instead.
pub fn mountain(game: &GameState, source: Source, id: impl ToCardId) -> Option<bool> {
    Some(card_queries::land_subtypes(game, source, id)?.contains(text_changes::land_subtype(
        game,
        source,
        LandSubtype::Mountain,
    )))
}

/// Returns true if the given card is a forest.
///
/// If a text-changing effect has been applied to the card which produced
/// `source` to replace instances of a land subtype in its rules text, this will
/// check for the new land subtype instead.
pub fn forest(game: &GameState, source: Source, id: impl ToCardId) -> Option<bool> {
    Some(card_queries::land_subtypes(game, source, id)?.contains(text_changes::land_subtype(
        game,
        source,
        LandSubtype::Forest,
    )))
}
