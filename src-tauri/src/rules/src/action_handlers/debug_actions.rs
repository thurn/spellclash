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

use data::actions::debug_action::DebugGameAction;
use data::actions::game_action::GameAction;
use data::core::primitives::PlayerName;
use data::game_states::game_state::GameState;
use tracing::{debug, instrument};
use utils::outcome::Outcome;
use utils::{fail, outcome};

#[instrument(err, level = "debug", skip(game))]
pub fn execute(game: &mut GameState, player: PlayerName, action: DebugGameAction) -> Outcome {
    match action {
        DebugGameAction::Undo => {
            debug!(?player, "Undoing last action");
            let mut undo_list = game.undo_tracker.undo.clone();
            let Some(mut previous) = undo_list.pop() else {
                fail!("No undo state available");
            };
            previous.undo_tracker.enabled = true;
            previous.undo_tracker.undo = undo_list;
            *game = *previous;
        }
    }
    outcome::OK
}
