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

use std::collections::BTreeSet;
use std::time::Instant;

use data::game_states::game_state::GameState;

use crate::core::game_state_node::GameStateNode;
use crate::core::selection_algorithm::SelectionAlgorithm;
use crate::core::state_evaluator::StateEvaluator;

/// An agent which does a depth 1 search of legal actions, returning the one
/// that produces the best evaluator state.
#[derive(Debug, Clone)]
pub struct SingleLevel {}

impl<N, E> SelectionAlgorithm<N, E> for SingleLevel
where
    N: GameStateNode,
    E: StateEvaluator<N>,
{
    fn pick_action(
        &self,
        _deadline: Instant,
        node: &N,
        evaluator: &E,
        player: N::PlayerName,
    ) -> N::Action {
        let mut best_score = i32::MIN;
        let mut best_action: Option<N::Action> = None;

        for action in node.legal_actions(player) {
            let mut child = node.make_copy();
            child.execute_action(player, action);
            let score = evaluator.evaluate(&child, player);
            if score > best_score {
                best_score = score;
                best_action = Some(action);
            }
        }

        best_action.expect("No legal actions found")
    }
}
