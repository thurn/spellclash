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
use data::prompts::prompt::{Prompt, PromptResponse, PromptType};
use tracing::instrument;
use utils::outcome::{Outcome, Value};
use utils::with_error::WithError;
use utils::{fail, outcome};

use crate::game_creation::initialize_game;

/// Updates the state of a game's prompt manager based on the selected option in
/// [PromptAction]. This does not validate the legality of the chosen action.
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
        game.prompts.action.as_ref().with_error(|| "Expected initial prompt action")?.clone();
    match action {
        PromptAction::PickNumber(n) => pick_number(game, initial_action, n),
        PromptAction::SelectCard { source, target } => select_card(game, *source, *target),
        PromptAction::SetSelectionOrder { source, target } => {
            set_selection_order(game, *source, *target)
        }
        PromptAction::SubmitCardSelection => submit_card_selection(game, initial_action),
    }
}

fn pick_number(
    game: &mut GameState,
    initial_action: GameAction,
    n: &u32,
) -> Value<Option<GameAction>> {
    push_response(game, initial_action, PromptResponse::PickNumber(*n))
}

fn select_card(
    game: &mut GameState,
    source: usize,
    target: Option<usize>,
) -> Value<Option<GameAction>> {
    let Some(PromptType::SelectCards(card_selection)) = game.prompts.current_prompt_type_mut()
    else {
        fail!("Expected SelectCards prompt");
    };
    Ok(None)
}

fn set_selection_order(
    game: &mut GameState,
    source: usize,
    target: usize,
) -> Value<Option<GameAction>> {
    Ok(None)
}

fn submit_card_selection(
    game: &mut GameState,
    initial_action: GameAction,
) -> Value<Option<GameAction>> {
    let Some(PromptType::SelectCards(card_selection)) = game.prompts.current_prompt_type() else {
        fail!("Expected SelectCards prompt");
    };
    push_response(
        game,
        initial_action,
        PromptResponse::SelectCards(card_selection.selected.clone()),
    )
}

fn push_response(
    game: &mut GameState,
    initial_action: GameAction,
    response: PromptResponse,
) -> Value<Option<GameAction>> {
    game.prompts.responses.push(response);
    game.prompts.current_prompt = None;
    Ok(Some(initial_action))
}
