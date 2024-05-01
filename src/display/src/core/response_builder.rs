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

use data::core::primitives::{CardId, PlayerName};

use crate::commands::command::Command;
use crate::commands::display_preferences::DisplayPreferences;
use crate::core::game_view::{DisplayPlayer, GameView};
use crate::core::object_position::ObjectPosition;

pub struct ResponseState {
    /// Whether to play animations as part of this update
    pub animate: bool,

    /// True for a full UI update, false if we are rendering a game snapshot for
    /// animation
    pub is_final_update: bool,

    /// User configuration for how this response should be rendered.
    pub display_preferences: DisplayPreferences,
}

/// Primary builder used to render game state.
///
/// Tracks a list of [Command]s to update the game client along with things like
/// which [PlayerName] we are rendering for.
pub struct ResponseBuilder {
    /// Player for whom we are building a UI update
    pub player: PlayerName,

    /// Response configuration
    pub state: ResponseState,

    /// Commands to send to this client
    pub commands: Vec<Command>,

    /// Tracks the positions of client cards as of the most recently-seen
    /// snapshot.
    ///
    /// This is used to customize animation behavior, mostly in order to not
    /// move cards to the "display" browser when they're already in another
    /// similar card browser.
    pub last_snapshot_positions: HashMap<CardId, ObjectPosition>,
}

impl ResponseBuilder {
    pub fn new(player: PlayerName, state: ResponseState) -> Self {
        Self { player, state, commands: vec![], last_snapshot_positions: HashMap::default() }
    }

    /// Adds a new command
    pub fn push(&mut self, command: Command) {
        self.commands.push(command);
    }

    /// Adds a new command to update the [GameView]
    pub fn push_game_view(&mut self, game: GameView) {
        for card in &game.cards {
            self.last_snapshot_positions.insert(card.id, card.position.clone());
        }

        self.commands.push(Command::UpdateGameView { view: game, animate: self.state.animate });
    }

    /// Converts a [PlayerName] into a [DisplayPlayer].
    pub fn to_display_player(&self, name: PlayerName) -> DisplayPlayer {
        if name == self.player {
            DisplayPlayer::Viewer
        } else {
            DisplayPlayer::Opponent
        }
    }
}
