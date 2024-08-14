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

use data::prompts::select_order_prompt::CardOrderLocation;
use primitives::game_primitives::CardId;
use serde::{Deserialize, Serialize};
use specta::Type;

use crate::core::card_view::ClientCardId;
use crate::core::game_view::DisplayPlayer;

/// Represents the position of some object in the UI
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Type)]
#[serde(rename_all = "camelCase")]
pub struct ObjectPosition {
    /// Position category
    pub position: Position,
    /// Sorting key, determines order within the position
    pub sorting_key: f64,
    /// Sub-key, used to break ties in sorting
    pub sorting_sub_key: f64,
}

impl Default for ObjectPosition {
    fn default() -> Self {
        Self { position: Position::Default, sorting_key: 0.0, sorting_sub_key: 0.0 }
    }
}

/// Sub-positions for objects within the battlefield.
#[derive(Clone, Debug, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Type)]
#[serde(rename_all = "camelCase")]
pub enum BattlefieldPosition {
    Mana,
    Permanents,
}

/// Possible types of display positions
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Type)]
#[serde(rename_all = "camelCase")]
pub enum Position {
    /// Object position used in interface elements like the deck viewer which
    /// don't rely on game positioning.
    Default,

    /// Object is not visible.
    Offscreen,

    /// Object is prominently revealed, being shown at a large size after
    /// being played.
    Played,

    /// Object is on the stack
    Stack,

    /// Object is in this player's hand
    Hand(DisplayPlayer),

    /// Object is in this player's deck
    Deck(DisplayPlayer),

    /// Object is in this player's discard pile
    DiscardPile(DisplayPlayer),

    /// Object is in this player's exile zone
    Exile(DisplayPlayer),

    /// Object is in this player's command zone
    CommandZone(DisplayPlayer),

    /// Object is controlled by this player in a given battlefield position
    Battlefield(DisplayPlayer, BattlefieldPosition),

    /// Object is in attack position for this player
    Attacking(DisplayPlayer),

    /// Object is controlled by this player and is blocking the provided set of
    /// attackers
    Blocking(DisplayPlayer, Vec<ClientCardId>),

    /// Object is being displayed in a card browser, e.g. to select from a list
    /// of cards while searching
    Browser,

    /// Object is being displayed in a list of cards available to select in a
    /// card selector.
    CardSelectionChoices,

    /// Object is being displayed in a location for picking its relative order
    CardOrderLocation(CardOrderLocation),

    /// Object has just been revealed to this viewer
    Revealed,

    /// Object is in a temporary holding space for cards in hand while resolving
    /// some other 'play card' ability.
    HandStorage,

    /// Object is not visible because it is inside the indicated card.
    InsideCard(ClientCardId),

    /// Object is attached to the indicated card.
    AttachedToCard(ClientCardId),
}
