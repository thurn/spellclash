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

use std::cmp;
use std::collections::HashSet;
use std::time::Instant;

use data::game_states::game_state::GameState;
use tracing::debug;

use crate::core::game_state_node::{GameStateNode, GameStatus};
use crate::core::selection_algorithm::SelectionAlgorithm;
use crate::core::state_evaluator::StateEvaluator;
use crate::tree_search::scored_action::ScoredAction;

/// Implements alpha-beta pruning over minimax tree search.
///
/// This is a 'fail soft' implementation per wikipedia. I have not been able to
/// detect any performance or gameplay difference with the 'fail hard' version.
///
/// See <https://en.wikipedia.org/wiki/Alpha-beta_pruning>
#[derive(Debug, Clone)]
pub struct AlphaBetaAlgorithm {
    pub search_depth: u32,
}

#[derive(Debug)]
pub struct DeadlineExceededError;

impl<N, E> SelectionAlgorithm<N, E> for AlphaBetaAlgorithm
where
    N: GameStateNode,
    E: StateEvaluator<N>,
{
    fn pick_action(
        &self,
        deadline: Instant,
        node: &N,
        evaluator: &E,
        player: N::PlayerName,
    ) -> N::Action {
        assert!(matches!(node.status(), GameStatus::InProgress { .. }));
        run_internal(deadline, node, evaluator, self.search_depth, player, i32::MIN, i32::MAX, true)
            .expect("Deadline exceeded")
            .action()
    }
}

#[allow(clippy::too_many_arguments)]
pub fn run_internal<N, E>(
    deadline: Instant,
    node: &N,
    evaluator: &E,
    depth: u32,
    player: N::PlayerName,
    mut alpha: i32,
    mut beta: i32,
    top_level: bool,
) -> Result<ScoredAction<N::Action>, DeadlineExceededError>
where
    N: GameStateNode,
    E: StateEvaluator<N>,
{
    match node.status() {
        _ if depth == 0 => Ok(ScoredAction::new(evaluator.evaluate(node, player))),
        GameStatus::Completed { .. } => Ok(ScoredAction::new(evaluator.evaluate(node, player))),
        GameStatus::InProgress { current_turn } if current_turn == player => {
            let mut result = ScoredAction::new(i32::MIN);
            for action in node.legal_actions(current_turn) {
                if deadline_exceeded(deadline, depth) {
                    return Err(DeadlineExceededError);
                }
                let mut child = node.make_copy();
                child.execute_action(current_turn, action);
                let score = run_internal(
                    deadline,
                    &child,
                    evaluator,
                    depth - 1,
                    player,
                    alpha,
                    beta,
                    false,
                )?
                .score();
                alpha = cmp::max(alpha, score);
                result.insert_max(action, score);
                if score >= beta {
                    break; // Beta cutoff
                }
            }
            Ok(result)
        }
        GameStatus::InProgress { current_turn } => {
            let mut result = ScoredAction::new(i32::MAX);
            for action in node.legal_actions(current_turn) {
                if deadline_exceeded(deadline, depth) {
                    return Err(DeadlineExceededError);
                }
                let mut child = node.make_copy();
                child.execute_action(current_turn, action);
                let score = run_internal(
                    deadline,
                    &child,
                    evaluator,
                    depth - 1,
                    player,
                    alpha,
                    beta,
                    false,
                )?
                .score();
                if top_level {
                    debug!("Score {:?} for action {:?}", score, action);
                }
                beta = cmp::min(beta, score);
                result.insert_min(action, score);
                if score <= alpha {
                    break; // Alpha cutoff
                }
            }
            assert!(result.has_action());
            Ok(result)
        }
    }
}

/// Check whether `deadline` has been exceeded.
fn deadline_exceeded(deadline: Instant, depth: u32) -> bool {
    depth > 1 && deadline < Instant::now()
}
