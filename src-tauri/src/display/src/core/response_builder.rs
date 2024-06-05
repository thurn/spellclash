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
use std::sync::Arc;

use data::core::primitives::PlayerName;
use data::game_states::game_state::{DebugActAsPlayer, GameState};
use rules::legality::legal_actions;

use crate::commands::command::{Command, SceneView};
use crate::core::card_view::ClientCardId;
use crate::core::display_state::DisplayState;
use crate::core::game_view::{DisplayPlayer, GameView};
use crate::core::object_position::ObjectPosition;

pub struct ResponseState<'a> {
    /// Whether to play animations as part of this update
    pub animate: bool,

    /// True for a full UI update, false if we are rendering a game snapshot for
    /// animation
    pub is_final_update: bool,

    /// User configuration for how this response should be rendered.
    pub display_state: &'a DisplayState,

    /// True if all cards should be revealed
    pub reveal_all_cards: bool,

    /// Allows a player to act as another player for debugging purposes
    pub act_as_player: Option<DebugActAsPlayer>,
}

/// Primary builder used to render game state.
///
/// Tracks a list of [Command]s to update the game client along with things like
/// which [PlayerName] we are rendering for.
pub struct ResponseBuilder<'a> {
    /// Player for whom we are building a UI update
    player: PlayerName,

    /// Response configuration
    pub response_state: ResponseState<'a>,

    /// Commands to send to this client
    pub commands: Vec<Command>,

    /// Tracks the positions of client cards as of the most recently-seen
    /// snapshot.
    ///
    /// This is used to customize animation behavior, mostly in order to not
    /// move cards to the "display" browser when they're already in another
    /// similar card browser.
    pub last_snapshot_positions: HashMap<ClientCardId, ObjectPosition>,
}

impl<'a> ResponseBuilder<'a> {
    pub fn new(player: PlayerName, state: ResponseState<'a>) -> Self {
        Self {
            player,
            response_state: state,
            commands: vec![],
            last_snapshot_positions: HashMap::default(),
        }
    }

    /// Adds a new command
    pub fn push(&mut self, command: Command) {
        self.commands.push(command);
    }

    /// Adds a new command to update the [GameView]
    pub fn push_game_view(&mut self, game: GameView) {
        for card in &game.cards {
            self.last_snapshot_positions.insert(card.id.clone(), card.position.clone());
        }

        self.commands.push(Command::UpdateScene(SceneView::GameView(game)));
    }

    /// Converts a [PlayerName] into a [DisplayPlayer].
    pub fn to_display_player(&self, name: PlayerName) -> DisplayPlayer {
        if name == self.player {
            DisplayPlayer::Viewer
        } else {
            DisplayPlayer::Opponent
        }
    }

    /// Returns the [PlayerName] to use for top-level display & positioning
    /// logic.
    pub fn display_as_player(&self) -> PlayerName {
        self.player
    }

    /// Returns the [PlayerName] that should be used for *actions* in the
    /// rendered UI.
    ///
    /// If the debug option is being used to allow a player to act as another,
    /// this may not match the user's [PlayerName].
    pub fn act_as_player(&self, game: &GameState) -> PlayerName {
        if let Some(act) = self.response_state.act_as_player {
            if act.name == legal_actions::next_to_act(game) {
                return act.name;
            }
        }

        self.player
    }
}
