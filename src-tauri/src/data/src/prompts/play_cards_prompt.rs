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

use primitives::game_primitives::{CardId, Zone};
/// A prompt shown to the user to allow them to play one or more cards from a
/// set of cards.
#[derive(Clone, Debug)]
pub struct PlayCardsPrompt {
    /// Zone of origin for the cards being played.
    pub from_zone: Zone,
    /// Identifies the choices of cards that the user can possibly play.
    pub cards: Vec<CardId>,
}
