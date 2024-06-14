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

use std::time::Instant;

use utils::command_line;
use utils::command_line::TracingStyle;

use crate::core::agent::AgentConfig;
use crate::core::game_state_node::GameStateNode;
use crate::core::selection_algorithm::SelectionAlgorithm;
use crate::core::state_evaluator::StateEvaluator;
use crate::tree_search::alpha_beta;

/// Implements a search algorithm which repeatedly applies alpha-beta search at
/// increasing depths until its deadline is exceeded
pub struct IterativeDeepeningSearch;

impl SelectionAlgorithm for IterativeDeepeningSearch {
    fn pick_action<N, E>(
        &self,
        config: AgentConfig,
        node: &N,
        evaluator: &E,
        player: N::PlayerName,
    ) -> N::Action
    where
        N: GameStateNode,
        E: StateEvaluator<N>,
    {
        let mut depth = 1;
        let mut best_action = None;

        while config.deadline > Instant::now() {
            if command_line::flags().tracing_style == TracingStyle::AggregateTime {
                println!(">>> Searching at depth {}", depth);
            }

            let result = alpha_beta::run_internal(
                config,
                node,
                evaluator,
                depth,
                player,
                i32::MIN,
                i32::MAX,
                true, // is_top_level
            );
            match result {
                Ok(a) => {
                    best_action = Some(a.action());
                }
                Err(_) => {
                    break;
                }
            }

            depth += 1;
        }

        best_action.expect("No action found")
    }
}
