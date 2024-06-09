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

use std::collections::{HashMap, HashSet};

use enum_iterator::Sequence;
use enumset::{EnumSet, EnumSetType};
use serde::{Deserialize, Serialize};
use specta::Type;

use crate::core::primitives::CardId;

/// Possible locations in which cards can be ordered.
#[derive(Debug, Hash, EnumSetType, Serialize, Deserialize, Type, Sequence)]
#[serde(rename_all = "camelCase")]
pub enum CardOrderLocation {
    /// Cards which have not yet been ordered
    Unordered,
    TopOfLibrary,
    BottomOfLibrary,
    Graveyard,
}

/// Selection restrictions.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum Quantity {
    /// No restrictions on number of cards selected.
    AnyNumber,

    /// Player must select exactly this number of cards *or* all cards in the
    /// prompt if less than this number are available.
    Count(usize),
}

/// A prompt for a player to select one or more cards from a set of cards to
/// reorder, used to implement scry/surveil type effects.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SelectOrderPrompt {
    /// Cards to order. If a location is valid for this prompt an entry *must*
    /// be created in this map, even if it is empty.
    ///
    /// This may initially have cards in any number of locations, including the
    /// 'unselected' location if cards need to be explicitly chosen to order.
    /// Each vector indicates the order chosen for elements
    /// in that selection location.
    pub cards: HashMap<CardOrderLocation, Vec<CardId>>,

    /// Validation for the number of cards selected.
    ///
    /// The counts used here are determined across all [CardOrderLocation]s
    /// *except* the 'Unordered' location. For example a 'Count' constraint
    /// will count the number of cards selected in all target locations but will
    /// not count unordered cards.
    pub quantity: Quantity,
}

impl SelectOrderPrompt {
    /// Returns the list of selected cards in a given selection location.
    pub fn in_location(&self, selection_type: CardOrderLocation) -> &Vec<CardId> {
        static EMPTY: Vec<CardId> = vec![];
        self.cards.get(&selection_type).unwrap_or(&EMPTY)
    }

    /// Returns true if the provided [CardId] is present anywhere among the
    /// cards for this prompt.
    pub fn contains_card(&self, card_id: CardId) -> bool {
        self.cards.values().any(|cards| cards.contains(&card_id))
    }
}
