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

use std::collections::HashSet;
use std::fmt::Debug;
use std::time::{Duration, Instant};

use data::actions::agent_action::AgentAction;
use data::actions::game_action::GameAction;
use data::actions::prompt_action::PromptAction;
use data::core::primitives;
use data::game_states::game_state::GameState;
use data::player_states::game_agent::{GameAgentImpl, PromptAgentImpl};
use data::prompts::prompt::Prompt;
use rules::legality::legal_actions::LegalActions;
use rules::legality::{legal_actions, legal_prompt_actions};
use tracing::{subscriber, Level};
use utils::command_line;
use utils::command_line::TracingStyle;

use crate::core::agent::{Agent, AgentData};
use crate::core::game_state_node::GameStateNode;
use crate::core::selection_algorithm::SelectionAlgorithm;
use crate::core::state_evaluator::StateEvaluator;
use crate::game::prompt_state_node_impl::PromptStateNode;

impl<TSelector, TEvaluator> GameAgentImpl for AgentData<TSelector, TEvaluator, GameState>
where
    TSelector: SelectionAlgorithm<GameState, TEvaluator> + Debug + Clone,
    TEvaluator: StateEvaluator<GameState> + Debug + Clone,
{
    fn select_action(&self, game: &GameState, player: primitives::PlayerName) -> GameAction {
        let mut copy = game.shallow_clone();
        *copy.current_search_agent_mut() = Some(player);
        select_action_impl(self, copy, player).as_game_action()
    }

    fn incremental_prompt_action(
        &self,
        game: &mut GameState,
        prompt: &Prompt,
        player: primitives::PlayerName,
    ) -> PromptAction {
        let legal =
            legal_prompt_actions::compute(prompt, player, LegalActions { for_human_player: false })
                .into_iter()
                .map(AgentAction::PromptAction)
                .collect::<HashSet<_>>();
        assert!(!legal.is_empty(), "No legal prompt actions available");
        self.selector.pick_prompt_action(game, player, legal).as_prompt_action()
    }
}

impl<TSelector, TEvaluator> PromptAgentImpl for AgentData<TSelector, TEvaluator, PromptStateNode>
where
    TSelector: SelectionAlgorithm<PromptStateNode, TEvaluator> + Debug + Clone,
    TEvaluator: StateEvaluator<PromptStateNode> + Debug + Clone,
{
    fn top_level_prompt_action(
        &self,
        game: &GameState,
        prompt: &Prompt,
        player: primitives::PlayerName,
    ) -> PromptAction {
        let mut copy = game.shallow_clone();
        *copy.current_search_agent_mut() = Some(player);
        let state = PromptStateNode { game: copy, prompt: Some(prompt.clone()) };
        select_action_impl(self, state, player).as_prompt_action()
    }
}

fn select_action_impl<TState, TSelector, TEvaluator>(
    agent: &AgentData<TSelector, TEvaluator, TState>,
    state: TState,
    player: TState::PlayerName,
) -> TState::Action
where
    TState: GameStateNode + Clone + Debug,
    TSelector: SelectionAlgorithm<TState, TEvaluator> + Debug + Clone,
    TEvaluator: StateEvaluator<TState> + Debug + Clone,
{
    assert_eq!(state.current_turn(), player, "Not {:?}'s turn", player);
    let legal = state.legal_actions(player).collect::<Vec<_>>();
    assert!(!legal.is_empty(), "No legal actions available");
    if legal.len() == 1 {
        return legal[0];
    }

    let deadline = Duration::from_secs(10);
    match command_line::flags().tracing_style {
        TracingStyle::AggregateTime | TracingStyle::None => {
            agent.pick_action(Instant::now() + deadline, &state)
        }
        TracingStyle::Forest => {
            let info_subscriber = tracing_subscriber::fmt().with_max_level(Level::INFO).finish();
            subscriber::with_default(info_subscriber, || {
                agent.pick_action(Instant::now() + deadline, &state)
            })
        }
    }
}
