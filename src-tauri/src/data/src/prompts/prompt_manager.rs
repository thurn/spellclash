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

use std::collections::VecDeque;

use serde::{Deserialize, Serialize};
use utils::fail;
use utils::outcome::{StopCondition, Value};

use crate::actions::game_action::GameAction;
use crate::core::primitives::{CardId, EntityId, PlayerName};
use crate::prompts::card_selection_prompt::CardSelectionPrompt;
use crate::prompts::choice_prompt::{Choice, ChoicePrompt};
use crate::prompts::pick_number_prompt::PickNumberPrompt;
use crate::prompts::prompt::{Prompt, PromptResponse, PromptType};
use crate::text_strings::Text;

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PromptManager {
    /// Most recent game action which was *not* a prompt response, i.e. the
    /// action which triggered the prompt stored here.
    pub action: Option<GameAction>,

    /// Current prompt to display to a player, if any.
    pub current_prompt: Option<Prompt>,

    /// Recorded responses in prompts displayed to the user, in order.
    ///
    /// Cleared after each new non-prompt game action.
    pub responses: Vec<PromptResponse>,

    /// Current position within the responses list.
    ///
    /// Responses are returned to the user in the order in which they're
    /// generated. If this number exceeds the size of the response list, we
    /// write the prompt to [Self::current_prompt], halt simulation, and
    /// request input from the user.
    pub response_index: usize,
}

impl PromptManager {
    /// Clears all stored prompts and sets the provided action as the
    /// currently-processing game action.
    pub fn reset_with_action(&mut self, action: &GameAction) {
        self.action = Some(action.clone());
        self.current_prompt = None;
        self.responses.clear();
        self.response_index = 0;
    }

    pub fn choose_entity(
        &mut self,
        player: PlayerName,
        description: Text,
        choices: Vec<Choice<EntityId>>,
    ) -> Value<EntityId> {
        let PromptResponse::EntityChoice(id) = self.send(Prompt {
            player,
            label: Some(description),
            prompt_type: PromptType::EntityChoice(ChoicePrompt { optional: false, choices }),
        })?
        else {
            fail!("Unexpected prompt response type!");
        };

        Ok(*id)
    }

    pub fn select_cards(
        &mut self,
        player: PlayerName,
        description: Text,
        prompt: CardSelectionPrompt,
    ) -> Value<Vec<CardId>> {
        let PromptResponse::SelectCards(ids) = self.send(Prompt {
            player,
            label: Some(description),
            prompt_type: PromptType::SelectCards(prompt),
        })?
        else {
            fail!("Unexpected prompt response type!");
        };

        Ok(ids.clone())
    }

    /// Show a [PickNumberPrompt].
    pub fn pick_number(
        &mut self,
        player: PlayerName,
        description: Text,
        prompt: PickNumberPrompt,
    ) -> Value<u32> {
        let PromptResponse::PickNumber(number) = self.send(Prompt {
            player,
            label: Some(description),
            prompt_type: PromptType::PickNumber(prompt),
        })?
        else {
            fail!("Unexpected prompt response type!");
        };

        Ok(*number)
    }

    fn send(&mut self, prompt: Prompt) -> Value<&PromptResponse> {
        let index = self.response_index;
        if let Some(response) = self.responses.get(self.response_index) {
            self.response_index += 1;
            Ok(response)
        } else {
            self.current_prompt = Some(prompt);
            Err(StopCondition::Prompt)
        }
    }
}
