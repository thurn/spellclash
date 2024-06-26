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
use data::core::primitives::{CardType, PermanentId};
use data::delegates::scope::Scope;
use data::game_states::game_state::GameState;
use data::printed_cards::card_subtypes::LandSubtype;

use crate::queries::card_queries;

pub fn creature(game: &GameState, id: impl ToCardId) -> Option<bool> {
    Some(card_queries::card_types(game, id)?.contains(CardType::Creature))
}

pub fn planeswalker(game: &GameState, id: impl ToCardId) -> Option<bool> {
    Some(card_queries::card_types(game, id)?.contains(CardType::Planeswalker))
}

pub fn battle(game: &GameState, id: impl ToCardId) -> Option<bool> {
    Some(card_queries::card_types(game, id)?.contains(CardType::Battle))
}

pub fn island(game: &GameState, id: impl ToCardId) -> Option<bool> {
    Some(card_queries::land_subtypes(game, id)?.contains(LandSubtype::Island))
}
