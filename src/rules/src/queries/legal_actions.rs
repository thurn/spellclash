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

use std::iter;

use data::actions::game_action::GameAction;
use data::core::primitives::PlayerName;
use data::game_states::game_state::GameState;

/// Iterator over all legal actions the named player can take in the current
/// game state.
pub fn compute(game: &GameState, player: PlayerName) -> impl Iterator<Item = GameAction> {
    let mut pass_priority = None;
    if next_to_act(game) == player {
        pass_priority = Some(iter::once(GameAction::PassPriority))
    }
    pass_priority.into_iter().flatten()
}

/// Returns the name of the player who is currently allowed to take an action.
pub fn next_to_act(game: &GameState) -> PlayerName {
    game.priority
}

/// Returns true if the named player can currently take the action to pass
/// priority.
pub fn can_pass_priority(game: &GameState, player: PlayerName) -> bool {
    game.priority == player
}
