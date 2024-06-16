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
use std::time::{Duration, Instant};

use data::actions::game_action::GameAction;
use data::actions::prompt_action::PromptAction;
use data::core::primitives;
use data::game_states::game_state;
use data::game_states::game_state::GameState;
use data::player_states::game_agent::GameAgentImpl;
use data::prompts::prompt::Prompt;
use rules::action_handlers::actions;
use rules::action_handlers::actions::ExecuteAction;
use rules::legality::legal_actions;
use rules::legality::legal_actions::LegalActions;
use tracing::{subscriber, Level};
use utils::command_line;
use utils::command_line::TracingStyle;

use crate::core::agent::{Agent, AgentConfig, AgentData};
use crate::core::game_state_node::{GameStateNode, GameStatus};
use crate::core::selection_algorithm::SelectionAlgorithm;
use crate::core::state_evaluator::StateEvaluator;

impl GameStateNode for GameState {
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
        actions::execute(self, player, action, ExecuteAction {
            skip_undo_tracking: false,
            validate: false,
        })
        .expect("Halt encountered during AI action execution");
    }
}

impl<TSelector, TEvaluator> GameAgentImpl for AgentData<TSelector, TEvaluator, GameState>
where
    TSelector: SelectionAlgorithm<GameState, TEvaluator> + Debug + Clone,
    TEvaluator: StateEvaluator<GameState> + Debug + Clone,
{
    fn select_action(&mut self, game: GameState, player: primitives::PlayerName) -> GameAction {
        assert_eq!(legal_actions::next_to_act(&game, None), player, "Not {:?}'s turn", player);
        let legal = legal_actions::compute(&game, player, LegalActions { for_human_player: false });
        assert!(!legal.is_empty(), "No legal actions available");
        if legal.len() == 1 {
            return legal[0];
        }

        let deadline = Duration::from_secs(100);
        match command_line::flags().tracing_style {
            TracingStyle::AggregateTime | TracingStyle::None => {
                self.pick_action(AgentConfig { deadline: Instant::now() + deadline }, &game)
            }
            TracingStyle::Forest => {
                let info_subscriber =
                    tracing_subscriber::fmt().with_max_level(Level::INFO).finish();
                subscriber::with_default(info_subscriber, || {
                    self.pick_action(AgentConfig { deadline: Instant::now() + deadline }, &game)
                })
            }
        }
    }

    fn select_prompt_action(
        &self,
        _game: &GameState,
        _prompt: &Prompt,
        _player: primitives::PlayerName,
    ) -> PromptAction {
        todo!()
    }
}
