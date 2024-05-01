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

/// A response to a user request.
#[derive(Debug, Clone)]
pub struct GameResponse {
    _context: ClientData,
    commands: Vec<Command>,
    opponent_response: Option<(UserId, Vec<Command>)>,
}

impl GameResponse {
    pub fn new(_context: ClientData) -> Self {
        Self { _context, commands: vec![], opponent_response: None }
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

    pub fn opponent_response(mut self, opponent_id: UserId, commands: Vec<Command>) -> Self {
        self.opponent_response = Some((opponent_id, commands));
        self
    }
}

/// Standard parameters for a client response
#[derive(Debug, Clone, Copy)]
pub struct ClientData {
    pub user_id: UserId,
    pub game_id: Option<GameId>,
}
