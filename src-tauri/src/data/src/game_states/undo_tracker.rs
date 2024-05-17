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

use serde::{Deserialize, Serialize};

use crate::game_states::game_state::GameState;

/// State for the undo system.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UndoTracker {
    /// Whether we are tracking Undo operations for this game
    pub enabled: bool,
    /// Previous states to jump to as a result of an 'undo' operation, if any.
    pub undo: Vec<Box<GameState>>,
}
