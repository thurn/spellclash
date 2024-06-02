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

use std::collections::HashSet;

use serde::{Deserialize, Serialize};

use crate::core::primitives::CardId;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum Quantity {
    /// Player must select exactly this number of cards *or* all cards in the
    /// prompt if less than this number are available.
    Count(usize),
}

/// A prompt for a player to select one or more cards from a set of cards to
/// apply some effect to.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CardSelectionPrompt {
    /// Cards which should be displayed in the browser.
    ///
    /// For example, this would contain cards that should be kept in hand during
    /// the 'discard to hand size' flow.
    pub choices: Vec<CardId>,

    /// Cards which have been selected. This should initially be empty.
    pub selected: Vec<CardId>,

    /// If true, the player seeing this prompt can rearrange the cards within
    /// the `target` position.
    pub can_reorder: bool,

    /// Validation for the number of cards selected.
    pub quantity: Quantity,
}

impl CardSelectionPrompt {
    pub fn new(choices: Vec<CardId>) -> Self {
        Self { choices, selected: vec![], can_reorder: false, quantity: Quantity::Count(1) }
    }

    pub fn can_reorder(mut self, can_reorder: bool) -> Self {
        self.can_reorder = can_reorder;
        self
    }

    pub fn quantity(mut self, quantity: Quantity) -> Self {
        self.quantity = quantity;
        self
    }
}
