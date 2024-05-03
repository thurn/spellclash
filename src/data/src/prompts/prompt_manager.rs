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

use utils::fail;
use utils::outcome::{StopCondition, Value};

use crate::core::primitives::{CardId, ObjectId, PlayerName};
use crate::prompts::card_selection_prompt::CardSelectionPrompt;
use crate::prompts::choice_prompt::{Choice, ChoicePrompt};
use crate::prompts::prompt::{Prompt, PromptResponse, PromptType};
use crate::text_strings::Text;

#[derive(Debug, Clone, Default)]
pub struct PromptManager {
    pub current_prompt: Option<Prompt>,
    pub current_response: Option<PromptResponse>,
}

impl PromptManager {
    pub fn choose_object(
        &mut self,
        player: PlayerName,
        description: Text,
        choices: Vec<Choice<ObjectId>>,
    ) -> Value<ObjectId> {
        let PromptResponse::ObjectChoice(id) = self.send(Prompt {
            player,
            label: Some(description),
            prompt_type: PromptType::ObjectChoice(ChoicePrompt { optional: false, choices }),
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

    fn send(&mut self, prompt: Prompt) -> Value<&PromptResponse> {
        if let Some(response) = &self.current_response {
            Ok(response)
        } else {
            self.current_prompt = Some(prompt);
            Err(StopCondition::Prompt)
        }
    }
}
