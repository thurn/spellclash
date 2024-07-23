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

use std::fmt::{Debug, Formatter};

use serde::{Deserialize, Serialize};

use crate::actions::user_action::UserAction;
use crate::core::primitives::{GameId, UserId};
use crate::decks::deck_name::DeckName;
use crate::game_states::game_state::DebugConfiguration;
use crate::player_states::player_state::PlayerType;

/// Debug options for a new game
#[derive(Copy, Clone, Eq, PartialEq, Debug, Default, Serialize, Deserialize)]
pub struct NewGameDebugOptions {
    /// Set the created game to have this ID
    pub override_game_id: Option<GameId>,

    /// Debug options
    pub configuration: DebugConfiguration,
}

/// Action to create a new game
#[derive(Clone, Serialize, Deserialize)]
pub struct NewGameAction {
    /// Deck to use for this game
    pub deck: DeckName,

    /// AI configuration for opponent
    pub opponent: PlayerType,

    /// Deck for opponent to use
    pub opponent_deck: DeckName,

    /// Debug options
    pub debug_options: NewGameDebugOptions,
}

impl Debug for NewGameAction {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("NewGameAction")
            .field("deck", &self.deck)
            .field("opponent_deck", &self.opponent_deck)
            .field("debug_options", &self.debug_options)
            .finish()
    }
}

impl From<NewGameAction> for UserAction {
    fn from(value: NewGameAction) -> Self {
        UserAction::NewGameAction(value)
    }
}
