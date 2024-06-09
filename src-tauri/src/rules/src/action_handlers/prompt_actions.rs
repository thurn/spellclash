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
use data::core::primitives::{CardId, PlayerName};
use data::game_states::game_state::GameState;
use data::prompts::prompt::{Prompt, PromptResponse, PromptType};
use data::prompts::select_order_prompt::CardOrderLocation;
use tracing::instrument;

use crate::game_creation::initialize_game;

/// Updates the state of a [Prompt] based on a [PromptAction].
///
/// There are two types of possible actions in response to a prompt:
///
/// 1) For intermediate updates as part of selecting a response (e.g. moving
///    around or reordering cards in a card selector), this function is expected
///    to mutate the provided [Prompt] and return None
/// 2) For 'final' actions (e.g. submitting the list of selected cards in a card
///    selector), this function is expected to return a [PromptResponse] which
///    will be used to unblock the thread which requested a choice. Changes to
///    the provided [Prompt] will be ignored in this case.
#[instrument(name = "prompt_actions_execute", level = "debug")]
pub fn execute(prompt: &mut Prompt, action: PromptAction) -> Option<PromptResponse> {
    match action {
        PromptAction::PickNumber(n) => Some(PromptResponse::PickNumber(n)),
        PromptAction::SelectAndSetOrder(card_id, location, index) => {
            select_and_set_order(prompt, card_id, location, index);
            None
        }
        PromptAction::SubmitCardSelection => None,
    }
}

fn select_and_set_order(
    prompt: &mut Prompt,
    card_id: CardId,
    location: CardOrderLocation,
    index: usize,
) {
    let PromptType::SelectOrder(prompt_data) = &mut prompt.prompt_type else {
        panic!("Expected SelectOrder prompt type");
    };

    // Remove previous position
    for order_location in enum_iterator::all::<CardOrderLocation>() {
        if let Some(map) = prompt_data.cards.get_mut(&order_location) {
            map.retain(|&id| id != card_id);
        }
    }

    prompt_data.cards.entry(location).or_default().insert(index, card_id);
}
