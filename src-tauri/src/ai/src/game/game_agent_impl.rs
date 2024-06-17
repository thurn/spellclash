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
use data::game_states::game_state::GameState;
use data::player_states::game_agent::GameAgentImpl;
use data::prompts::prompt::Prompt;
use rules::legality::legal_actions;
use rules::legality::legal_actions::LegalActions;
use tracing::{subscriber, Level};
use utils::command_line;
use utils::command_line::TracingStyle;

use crate::core::agent::{Agent, AgentData};
use crate::core::selection_algorithm::SelectionAlgorithm;
use crate::core::state_evaluator::StateEvaluator;

impl<TSelector, TEvaluator> GameAgentImpl for AgentData<TSelector, TEvaluator, GameState>
where
    TSelector: SelectionAlgorithm<GameState, TEvaluator> + Debug + Clone,
    TEvaluator: StateEvaluator<GameState> + Debug + Clone,
{
    fn select_action(&self, game: &GameState, player: primitives::PlayerName) -> GameAction {
        assert_eq!(legal_actions::next_to_act(game, None), player, "Not {:?}'s turn", player);
        let legal = legal_actions::compute(game, player, LegalActions { for_human_player: false });
        assert!(!legal.is_empty(), "No legal actions available");
        if legal.len() == 1 {
            return legal[0];
        }

        let copy = game.shallow_clone();
        let deadline = Duration::from_secs(100);
        match command_line::flags().tracing_style {
            TracingStyle::AggregateTime | TracingStyle::None => {
                self.pick_action(Instant::now() + deadline, &copy)
            }
            TracingStyle::Forest => {
                let info_subscriber =
                    tracing_subscriber::fmt().with_max_level(Level::INFO).finish();
                subscriber::with_default(info_subscriber, || {
                    self.pick_action(Instant::now() + deadline, &copy)
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
