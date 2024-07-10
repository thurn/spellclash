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

use std::collections::{BTreeMap, BTreeSet};

use enum_iterator::Sequence;
use enumset::{EnumSet, EnumSetType};
use serde::{Deserialize, Serialize};
use specta::Type;

use crate::core::primitives::CardId;

/// Possible locations in which cards can be ordered.
#[derive(Debug, Hash, Ord, PartialOrd, EnumSetType, Type, Sequence, Serialize, Deserialize)]
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

    /// Player must select order for exactly this number of cards from the
    /// [CardOrderLocation::Unordered] location *or* all cards in that location
    /// if less than this number are available.
    Ordered(usize),
}

/// A prompt for a player to select one or more cards from a set of cards to
/// reorder, used to implement scry/surveil type effects.
#[derive(Clone, Debug)]
pub struct SelectOrderPrompt {
    /// Cards to order. If a location is valid for this prompt an entry *must*
    /// be created in this map, even if it is empty.
    ///
    /// This may initially have cards in any number of locations, including the
    /// 'unselected' location if cards need to be explicitly chosen to order.
    /// Each vector indicates the order chosen for elements
    /// in that selection location.
    pub cards: BTreeMap<CardOrderLocation, Vec<CardId>>,

    /// Validation for the number of cards selected.
    pub quantity: Quantity,

    /// Cards which have been moved within the prompt at least once.
    ///
    /// In order to prevent infinite loops in AI action selection, we only allow
    /// it to move each card one time.
    pub moved: BTreeSet<CardId>,
}

impl SelectOrderPrompt {
    pub fn new(cards: BTreeMap<CardOrderLocation, Vec<CardId>>) -> Self {
        SelectOrderPrompt { cards, quantity: Quantity::AnyNumber, moved: BTreeSet::new() }
    }

    pub fn quantity(mut self, quantity: Quantity) -> Self {
        self.quantity = quantity;
        self
    }

    /// Returns the list of selected cards in a given selection location.
    pub fn cards_in_location(&self, selection_type: CardOrderLocation) -> &Vec<CardId> {
        static EMPTY: Vec<CardId> = vec![];
        self.cards.get(&selection_type).unwrap_or(&EMPTY)
    }

    /// Counts the cards stored in this prompt, skipping cards in the
    /// [CardOrderLocation::Unordered] location.
    pub fn count_ordered_cards(&self) -> usize {
        self.cards
            .iter()
            .filter(|(&location, _)| location != CardOrderLocation::Unordered)
            .map(|(_, cards)| cards.len())
            .sum()
    }

    /// Returns true if the provided [CardId] is present anywhere among the
    /// cards for this prompt.
    pub fn contains_card(&self, card_id: CardId) -> bool {
        self.cards.values().any(|cards| cards.contains(&card_id))
    }
}
