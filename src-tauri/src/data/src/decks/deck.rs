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

use std::collections::HashMap;

use enumset::EnumSet;
use serde::{Deserialize, Serialize};
use serde_with::serde_as;

use crate::card_definitions::card_name::CardName;
use crate::printed_cards::printed_card_id::PrintedCardId;

/// Data for a deck
#[serde_as]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Deck {
    /// Quantities of cards in this deck
    #[serde_as(as = "Vec<(_, _)>")]
    pub cards: HashMap<PrintedCardId, u64>,
}
