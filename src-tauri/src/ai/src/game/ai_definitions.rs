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

use std::fmt::Debug;
use std::ops::{Deref, DerefMut};

use ai_core::game_state_node::{GameStateNode, GameStatus};
use data::actions::game_action::GameAction;
use data::core::primitives;
use data::game_states::game_state;
use data::game_states::game_state::GameState;
use rules::action_handlers::actions;
use rules::action_handlers::actions::ExecuteAction;
use rules::legality::legal_actions;
use rules::legality::legal_actions::LegalActions;

#[derive(Debug, Clone)]
pub struct AgentGameState(pub GameState);

impl Deref for AgentGameState {
    type Target = GameState;

    fn deref(&self) -> &GameState {
        &self.0
    }
}

impl DerefMut for AgentGameState {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl GameStateNode for AgentGameState {
    type Action = GameAction;
    type PlayerName = primitives::PlayerName;

    fn make_copy(&self) -> Self {
        self.clone()
    }

    fn status(&self) -> GameStatus<primitives::PlayerName> {
        match self.status {
            game_state::GameStatus::GameOver { winners } => GameStatus::Completed { winners },
            _ => GameStatus::InProgress { current_turn: legal_actions::next_to_act(self, None) },
        }
    }

    fn legal_actions<'a>(
        &'a self,
        player: primitives::PlayerName,
    ) -> Box<dyn Iterator<Item = GameAction> + 'a> {
        Box::new(
            legal_actions::compute(self, player, LegalActions { for_human_player: false })
                .into_iter(),
        )
    }

    fn execute_action(&mut self, player: primitives::PlayerName, action: GameAction) {
        actions::execute(self, player, action, ExecuteAction { automatic: false, validate: false });
    }
}
