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

use data::actions::game_action::GameAction;
use data::actions::prompt_action::PromptAction;
use data::core::primitives::{CardId, PlayerName};
use data::game_states::game_state::GameState;
use data::prompts::pick_number_prompt::PickNumberPrompt;
use data::prompts::prompt::{Prompt, PromptType};
use data::prompts::select_order_prompt::{CardOrderLocation, Quantity, SelectOrderPrompt};
use tracing::instrument;

use crate::legality::legal_actions::LegalActions;

/// Returns the set of legal actions the [PlayerName] player can take in
/// response to this [Prompt].
pub fn compute(prompt: &Prompt, player: PlayerName, options: LegalActions) -> Vec<PromptAction> {
    if prompt.player != player {
        return vec![];
    }

    match &prompt.prompt_type {
        PromptType::PickNumber(PickNumberPrompt { minimum, maximum }) => {
            (*minimum..=*maximum).map(PromptAction::PickNumber).collect()
        }
        PromptType::SelectOrder(select_order) => select_order_prompt_actions(select_order, options),
        PromptType::EntityChoice(data) => {
            data.choices.iter().map(|choice| PromptAction::SelectEntity(choice.entity_id)).collect()
        }
        PromptType::PlayCards(_) => todo!("Implement this"),
        PromptType::MultipleChoice(data) => data
            .choices()
            .iter()
            .enumerate()
            .map(|(i, choice)| PromptAction::SelectChoice(i))
            .collect(),
    }
}

/// Returns true if the [PlayerName] player can currently legally take the
/// provided [PromptAction].
#[instrument(level = "trace", skip(prompt, prompt_action))]
pub fn can_take_action(prompt: &Prompt, player: PlayerName, prompt_action: PromptAction) -> bool {
    compute(prompt, player, LegalActions { for_human_player: true })
        .iter()
        .any(|&action| action == prompt_action)
}

fn select_order_prompt_actions(
    prompt: &SelectOrderPrompt,
    options: LegalActions,
) -> Vec<PromptAction> {
    let mut result = vec![];
    let at_maximum = match prompt.quantity {
        Quantity::AnyNumber => {
            result.push(PromptAction::SubmitCardSelection);
            true
        }
        Quantity::Ordered(n) => {
            if prompt.count_ordered_cards() == n
                || prompt.cards_in_location(CardOrderLocation::Unordered).is_empty()
            {
                result.push(PromptAction::SubmitCardSelection);
                true
            } else {
                false
            }
        }
    };

    if options.for_human_player || !at_maximum {
        // Only allow AI players to select cards to move if they're below the maximum
        // number of cards to select, in order to prevent searching invalid prompt
        // states.
        for (&location, card_ids) in &prompt.cards {
            for (i, &card_id) in card_ids.iter().enumerate() {
                add_select_order_actions_for_card(
                    &mut result,
                    prompt,
                    options,
                    location,
                    i,
                    card_id,
                );
            }
        }
    }

    result
}

fn add_select_order_actions_for_card(
    result: &mut Vec<PromptAction>,
    prompt: &SelectOrderPrompt,
    options: LegalActions,
    current_location: CardOrderLocation,
    current_index: usize,
    card_id: CardId,
) {
    if prompt.moved.contains(&card_id) && !options.for_human_player {
        // Don't allow moving the same card twice for AI agents
        return;
    }

    for &location in prompt.cards.keys() {
        if location == CardOrderLocation::Unordered && !options.for_human_player {
            // Don't allow moving to Unordered for AI agents
            continue;
        }

        for i in 0..=prompt.cards[&location].len() {
            if location == current_location && i == current_index {
                // Skip moving to current location
                continue;
            }
            result.push(PromptAction::SelectOrder(card_id, location, i));
        }
    }

    if !options.for_human_player && result.is_empty() {
        panic!("No legal actions found!");
    }
}
