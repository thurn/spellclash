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

use data::core::primitives::PlayerName;
use data::game_states::game_state::GameState;

use crate::commands::command::Command;
use crate::commands::display_preferences::DisplayPreferences;

/// Returns a series of [Command]s which fully describe the current state of the
/// provided game
pub fn connect(
    _game: &GameState,
    _player: PlayerName,
    _preferences: DisplayPreferences,
) -> Vec<Command> {
    vec![]
}

/// Returns a series of commands which contain animations for recent changes to
/// game states, followed by a snapshot of the current game state as returned by
/// [connect].
pub fn render_updates(
    _game: &GameState,
    _player_name: PlayerName,
    _preferences: DisplayPreferences,
) -> Vec<Command> {
    vec![]
}
