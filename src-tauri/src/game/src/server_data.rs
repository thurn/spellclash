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

use data::core::primitives::{GameId, UserId};
use display::commands::command::Command;
use display::commands::display_state::DisplayState;
use display::commands::scene_identifier::SceneIdentifier;
use display::panels::modal_panel::{ModalPanel, PanelData};
use serde::{Deserialize, Serialize};
use specta::Type;

/// A response to a user request.
#[derive(Debug, Clone, Serialize, Deserialize, Type)]
#[serde(rename_all = "camelCase")]
pub struct GameResponse {
    /// Current context, must be returned to server with all future requests
    pub client_data: ClientData,

    /// Animated updates to game state
    pub commands: Vec<Command>,
}

impl GameResponse {
    pub fn new(client_data: ClientData) -> Self {
        Self { client_data, commands: vec![] }
    }

    pub fn command(mut self, command: impl Into<Command>) -> Self {
        self.commands.push(command.into());
        self
    }

    pub fn insert_command(&mut self, index: usize, command: impl Into<Command>) {
        self.commands.insert(index, command.into())
    }

    pub fn push_command(&mut self, command: impl Into<Command>) {
        self.commands.push(command.into());
    }

    pub fn commands(mut self, mut commands: Vec<Command>) -> Self {
        self.commands.append(&mut commands);
        self
    }
}

/// Standard parameters for a client request & response
#[derive(Debug, Clone, Serialize, Deserialize, Type)]
#[serde(rename_all = "camelCase")]
pub struct ClientData {
    pub user_id: UserId,
    pub scene: SceneIdentifier,

    /// Optionally, a panel to display on top of the primary scene content
    pub modal_panel: Option<ModalPanel>,

    /// Options for how the game state should be visually rendered
    pub display_state: DisplayState,
}

impl ClientData {
    pub fn new(user_id: UserId, scene: SceneIdentifier) -> Self {
        Self { user_id, scene, modal_panel: None, display_state: DisplayState::default() }
    }

    pub fn game_id(&self) -> GameId {
        match self.scene {
            SceneIdentifier::Game(id) => id,
            _ => panic!("No GameId provided"),
        }
    }
}
