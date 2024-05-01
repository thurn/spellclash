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

use crate::actions::user_action::UserAction;
use crate::core::primitives::{GameId, UserId};
use crate::decks::deck_name::DeckName;

/// Debug options for a new game
#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub struct NewGameDebugOptions {
    /// Set the created game to have this ID
    pub override_game_id: Option<GameId>,
}

/// Action to create a new game
#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub struct NewGameAction {
    /// Deck to use for this game
    pub deck: DeckName,

    /// Deck for opponent to use
    pub opponent_deck: DeckName,

    /// Optionally, a [UserId] for this player
    pub opponent_id: Option<UserId>,

    /// Debug options
    pub debug_options: NewGameDebugOptions,
}

impl From<NewGameAction> for UserAction {
    fn from(value: NewGameAction) -> Self {
        UserAction::NewGameAction(value)
    }
}
