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
    TopOfLibrary,
    BottomOfLibrary,
    Graveyard,
}

/// Selection restrictions.
///
/// The counts used here are determined across all [CardOrderLocation]s. For
/// example a 'Count' constraint will count the number of cards selected in all
/// valid locations.
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
pub struct CardSelectOrderPrompt {
    /// All cards which should be displayed in the browser.
    pub choices: Vec<CardId>,

    /// Locations to which the player can move cards and pick their order.
    ///
    /// For each location specified here, the player will be able to move cards
    /// to these locations and pick their relative order.
    pub locations: EnumSet<CardOrderLocation>,

    /// Cards which have been ordered.
    ///
    /// This may initially be empty (if the player must select some cards to
    /// order from a list of options, e.g. brainstorm) or may initially be
    /// populated with choices (if the cards already exist in some target zone,
    /// e.g. scry). Each vector indicates the order chosen for elements in
    /// that selection set.
    ///
    /// Only [CardOrderLocation]s which are specified in [Self::locations]
    /// are allowed as keys here.
    pub ordered: HashMap<CardOrderLocation, Vec<CardId>>,

    /// Validation for the number of cards selected.
    pub quantity: Quantity,
}

impl CardSelectOrderPrompt {
    /// Returns the list of selected cards in a given selection location.
    pub fn in_location(&self, selection_type: CardOrderLocation) -> &Vec<CardId> {
        static EMPTY: Vec<CardId> = vec![];
        self.ordered.get(&selection_type).unwrap_or(&EMPTY)
    }
}
