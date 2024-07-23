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
use data::game_states::game_phase_step::GamePhaseStep;
use data::game_states::game_state;
use data::game_states::game_state::GameState;
use rules::action_handlers::actions;
use rules::action_handlers::actions::ExecuteAction;
use rules::legality::legal_actions;
use rules::legality::legal_actions::LegalActions;

use crate::core::game_state_node::{GameStateNode, GameStatus};

impl GameStateNode for GameState {
    type Action = AgentAction;
    type PlayerName = primitives::PlayerName;

    fn make_copy(&self) -> Self {
        self.shallow_clone()
    }

    fn status(&self) -> GameStatus<primitives::PlayerName> {
        match self.status {
            game_state::GameStatus::GameOver { winners } => GameStatus::Completed { winners },
            _ => GameStatus::InProgress {
                current_turn: legal_actions::next_to_act(self, None).unwrap(),
            },
        }
    }

    fn legal_actions<'a>(
        &'a self,
        player: primitives::PlayerName,
    ) -> Box<dyn Iterator<Item = AgentAction> + 'a> {
        let legal = legal_actions::compute(self, player, LegalActions { for_human_player: false });
        Box::new(legal.into_iter().map(AgentAction::GameAction))
    }

    fn execute_action(&mut self, player: primitives::PlayerName, action: AgentAction) {
        actions::execute(self, player, action.as_game_action(), ExecuteAction {
            skip_undo_tracking: true,
            validate: false,
        });
    }

    fn set_agent_state(&mut self, agent_state: AgentState<Self::PlayerName, Self::Action>) {
        self.agent_state = Some(agent_state);
    }

    fn get_agent_state(&self) -> &AgentState<Self::PlayerName, Self::Action> {
        self.agent_state.as_ref().expect("Agent state not found")
    }

    fn get_agent_state_mut(&mut self) -> &mut AgentState<Self::PlayerName, Self::Action> {
        self.agent_state.as_mut().expect("Agent state not found")
    }

    fn take_agent_state(mut self) -> AgentState<Self::PlayerName, Self::Action> {
        self.agent_state.take().expect("Agent state not found")
    }
}
