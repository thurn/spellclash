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

use tokio::sync::{mpsc, oneshot};

use crate::core::primitives::{CardId, ObjectId, PlayerName};
use crate::prompts::card_selection_prompt::CardSelectionPrompt;
use crate::prompts::choice_prompt::{Choice, ChoicePrompt};
use crate::prompts::prompt::{Prompt, PromptResponse, PromptType};
use crate::text_strings::Text;

#[derive(Debug, Clone, Default)]
pub struct PromptManager {
    pub sender: Option<mpsc::UnboundedSender<(oneshot::Sender<PromptResponse>, Prompt)>>,
}

impl PromptManager {
    pub fn choose_object(
        &self,
        player: PlayerName,
        description: Text,
        choices: Vec<Choice<ObjectId>>,
    ) -> ObjectId {
        let response = self.send(Prompt {
            player,
            label: Some(description),
            prompt_type: PromptType::ObjectChoice(ChoicePrompt { optional: false, choices }),
        });

        let PromptResponse::ObjectChoice(object_id) = response else {
            panic!("Unexpected prompt response!");
        };

        object_id
    }

    pub fn select_cards(
        &self,
        player: PlayerName,
        description: Text,
        prompt: CardSelectionPrompt,
    ) -> Vec<CardId> {
        let response = self.send(Prompt {
            player,
            label: Some(description),
            prompt_type: PromptType::SelectCards(prompt),
        });

        let PromptResponse::SelectCards(ids) = response else {
            panic!("Unexpected prompt response!");
        };

        ids
    }

    fn send(&self, prompt: Prompt) -> PromptResponse {
        let sender = &self.sender.as_ref().expect("No prompt sender");
        let (respond, receive) = oneshot::channel();
        sender.send((respond, prompt)).expect("Error sending message");
        receive.blocking_recv().expect("Error receiving prompt response")
    }
}
