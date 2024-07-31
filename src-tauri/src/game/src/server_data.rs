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

use display::commands::command::Command;
use display::commands::scene_identifier::SceneIdentifier;
use display::panels::modal_panel::{ModalPanel, PanelData};
use primitives::game_primitives::{GameId, UserId};
use serde::{Deserialize, Serialize};
use specta::Type;
use tokio::sync::mpsc::UnboundedSender;
use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct Client {
    pub data: ClientData,
    pub channel: UnboundedSender<GameResponse>,
}

impl Client {
    pub fn send(&self, command: impl Into<Command>) {
        let mut client_data = self.data.clone();
        client_data.id = Uuid::new_v4();
        self.channel
            .send(GameResponse { client_data, command: command.into() })
            .expect("Failed to send command, receiver has dropped");
    }

    pub fn send_all(&self, commands: Vec<Command>) {
        for command in commands {
            self.send(command);
        }
    }
}

/// A response to a user request.
#[derive(Debug, Clone, Serialize, Deserialize, Type)]
#[serde(rename_all = "camelCase")]
pub struct GameResponse {
    /// Current context, must be returned to server with all future requests
    pub client_data: ClientData,

    /// Update to visual game state
    pub command: Command,
}

/// Standard parameters for a client request & response
#[derive(Debug, Clone, Serialize, Deserialize, Type)]
#[serde(rename_all = "camelCase")]
pub struct ClientData {
    /// Unique identifier for a given client data instance.
    ///
    /// Each request and response should be tagged with its own ID.
    pub id: Uuid,

    /// User who is currently connected
    pub user_id: UserId,

    /// Currently-displayed top level screen
    pub scene: SceneIdentifier,
}

impl ClientData {
    pub fn game_id(&self) -> GameId {
        match self.scene {
            SceneIdentifier::Game(id) => id,
            _ => panic!("No GameId provided"),
        }
    }
}
