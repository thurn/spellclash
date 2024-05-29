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

use data::actions::game_action::{CombatAction, GameAction};
use data::actions::prompt_action::PromptAction;
use data::core::primitives::PlayerName;
use data::game_states::game_state::GameState;
use data::prompts::prompt::PromptResponse;
use tracing::instrument;
use utils::outcome;
use utils::outcome::{Outcome, Value};
use utils::with_error::WithError;

/// Updates the state of a game's prompt manager based on the selected option in
/// [PromptAction].
///
/// If this selection completes the prompt, returns the initial [GameAction]
/// that triggered the prompt, which should now be executed again with the
/// updated prompt state. Returns None if the current prompt is not complete and
/// we should wait for further user input.
#[instrument(name = "prompt_actions_execute", level = "debug", skip(game))]
pub fn execute<'a>(
    game: &mut GameState,
    player: PlayerName,
    action: &PromptAction,
) -> Value<Option<GameAction>> {
    let initial_action =
        game.prompts.action.as_ref().with_error(|| "Expected initial prompt action")?;
    Ok(match action {
        PromptAction::PickNumber(n) => {
            game.prompts.responses.push(PromptResponse::PickNumber(*n));
            game.prompts.current_prompt = None;
            Some(initial_action.clone())
        }
    })
}
