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

use data::actions::prompt_action::PromptAction;
use data::prompts::prompt::{Prompt, PromptResponse, PromptType, SelectedOrder};
use data::prompts::select_order_prompt::CardOrderLocation;
use primitives::game_primitives::{CardId, PlayerName};
use tracing::instrument;

pub enum PromptExecutionResult {
    Prompt(Prompt),
    PromptResponse(PromptResponse),
}

/// Updates the state of a [Prompt] based on a [PromptAction].
///
/// There are two types of possible actions in response to a prompt:
///
/// 1) For intermediate updates as part of selecting a response (e.g. moving
///    around or reordering cards in a card selector), this function is expected
///    to return a new [Prompt] to show.
/// 2) For 'final' actions e.g. submitting the list of selected cards in a card
///    selector), this function is expected to return a [PromptResponse] which
///    will be used to unblock the thread which requested a choice.
#[instrument(name = "prompt_actions_execute", level = "debug", skip(prompt))]
pub fn execute(prompt: Prompt, action: PromptAction) -> PromptExecutionResult {
    match action {
        PromptAction::PickNumber(n) => {
            PromptExecutionResult::PromptResponse(PromptResponse::PickNumber(n))
        }
        PromptAction::SelectOrder(card_id, location, index) => {
            select_order(prompt, card_id, location, index)
        }
        PromptAction::SelectEntity(entity_id) => {
            PromptExecutionResult::PromptResponse(PromptResponse::EntityChoice(entity_id))
        }
        PromptAction::SubmitCardSelection => {
            let PromptType::SelectOrder(prompt_data) = prompt.prompt_type else {
                panic!("Expected SelectOrder prompt type");
            };
            PromptExecutionResult::PromptResponse(PromptResponse::SelectOrder(SelectedOrder::new(
                prompt_data.cards,
            )))
        }
        PromptAction::SelectChoice(index) => {
            PromptExecutionResult::PromptResponse(PromptResponse::MultipleChoice(index))
        }
    }
}

fn select_order(
    mut prompt: Prompt,
    card_id: CardId,
    location: CardOrderLocation,
    index: usize,
) -> PromptExecutionResult {
    let PromptType::SelectOrder(prompt_data) = &mut prompt.prompt_type else {
        panic!("Expected SelectOrder prompt type");
    };

    prompt_data.moved.insert(card_id);

    // Remove previous position
    for order_location in enum_iterator::all::<CardOrderLocation>() {
        if let Some(map) = prompt_data.cards.get_mut(&order_location) {
            map.retain(|&id| id != card_id);
        }
    }

    prompt_data.cards.entry(location).or_default().insert(index, card_id);
    PromptExecutionResult::Prompt(prompt)
}
