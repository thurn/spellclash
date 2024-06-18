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

use ai_core::core::agent_state::AgentState;
use data::actions::agent_action::AgentAction;
use data::core::primitives;
use data::game_states::game_state;
use data::game_states::game_state::GameState;
use data::prompts::prompt::Prompt;
use rules::action_handlers::actions::ExecuteAction;
use rules::action_handlers::prompt_actions::PromptExecutionResult;
use rules::action_handlers::{actions, prompt_actions};
use rules::legality::legal_actions::LegalActions;
use rules::legality::{legal_actions, legal_prompt_actions};

use crate::core::game_state_node::{GameStateNode, GameStatus};

#[derive(Debug, Clone)]
pub struct PromptStateNode {
    pub game: GameState,
    pub prompt: Option<Prompt>,
}

impl GameStateNode for PromptStateNode {
    type Action = AgentAction;
    type PlayerName = primitives::PlayerName;

    fn make_copy(&self) -> Self {
        Self { game: self.game.shallow_clone(), prompt: self.prompt.clone() }
    }

    fn status(&self) -> GameStatus<primitives::PlayerName> {
        match self.game.status {
            game_state::GameStatus::GameOver { winners } => GameStatus::Completed { winners },
            _ => GameStatus::InProgress {
                current_turn: legal_actions::next_to_act(&self.game, self.prompt.as_ref()),
            },
        }
    }

    fn legal_actions<'a>(
        &'a self,
        player: primitives::PlayerName,
    ) -> Box<dyn Iterator<Item = AgentAction> + 'a> {
        if let Some(prompt) = &self.prompt {
            Box::new(
                legal_prompt_actions::compute(prompt, player, LegalActions {
                    for_human_player: false,
                })
                .into_iter()
                .map(AgentAction::PromptAction),
            )
        } else {
            self.game.legal_actions(player)
        }
    }

    fn execute_action(&mut self, player: primitives::PlayerName, action: AgentAction) {
        if let Some(prompt) = self.prompt.take() {
            match prompt_actions::execute(prompt, action.as_prompt_action()) {
                PromptExecutionResult::Prompt(p) => self.prompt = Some(p),
                PromptExecutionResult::PromptResponse(_) => {}
            }
        } else {
            self.game.execute_action(player, action)
        }
    }

    fn set_agent_state(&mut self, agent_state: AgentState<Self::PlayerName, Self::Action>) {
        self.game.set_agent_state(agent_state)
    }

    fn get_agent_state(&self) -> &AgentState<Self::PlayerName, Self::Action> {
        self.game.get_agent_state()
    }

    fn get_agent_state_mut(&mut self) -> &mut AgentState<Self::PlayerName, Self::Action> {
        self.game.get_agent_state_mut()
    }

    fn take_agent_state(mut self) -> AgentState<Self::PlayerName, Self::Action> {
        self.game.take_agent_state()
    }
}