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

use data::game_states::game_state::GameState;
use data::game_states::history_data::TakenGameAction;
use data::player_states::player_map::PlayerMap;

/// Returns true if the given game state has any 'undo'-able actions.
pub fn can_undo(game: &GameState) -> bool {
    let c = undoable_action_count(&game.history.player_actions);
    eprintln!("Count: {c}");
    c > 0
}

/// Returns the number of 'undo'-able actions in the given player map.
pub fn undoable_action_count(actions: &PlayerMap<Vec<TakenGameAction>>) -> usize {
    actions.values().map(|(_, a)| a.iter().filter(|a| a.track_for_undo).count()).sum::<usize>()
}
