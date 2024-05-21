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

use enum_kinds::EnumKind;
use serde::{Deserialize, Serialize};
use specta::Type;

use crate::core::game_view::GameView;
use crate::core::main_menu_view::MainMenuView;

#[derive(Clone, Debug, Serialize, Deserialize, Type)]
pub struct UpdateGameViewCommand {
    /// New visual game state
    pub view: GameView,

    /// Whether to animate updates to this state
    pub animate: bool,
}

/// Represents an instruction to the client to perform some visual update.
#[derive(Clone, Debug, EnumKind, Serialize, Deserialize, Type)]
#[enum_kind(CommandKind)]
pub enum Command {
    UpdateGameView(UpdateGameViewCommand),

    UpdateMainMenuView(MainMenuView),
}

impl Command {
    pub fn kind(&self) -> CommandKind {
        CommandKind::from(self)
    }
}
