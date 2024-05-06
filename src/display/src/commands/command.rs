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

use crate::commands::scene_name::SceneName;
use crate::core::game_view::GameView;

/// Represents an instruction to the client to perform some visual update.
#[derive(Clone, Debug, EnumKind)]
#[enum_kind(CommandKind)]
pub enum Command {
    /// Requests to load a new top-level game scene
    LoadScene {
        /// Name of scene to load
        name: SceneName,

        /// Add this scene to the set of available scenes
        additive: bool,

        /// Loading this scene even if it is currently being displayed
        load_if_current: bool,
    },

    UpdateGameView {
        /// New visual game state
        view: GameView,

        /// Whether to animate updates to this state
        animate: bool,
    },
}

impl Command {
    pub fn kind(&self) -> CommandKind {
        CommandKind::from(self)
    }
}
