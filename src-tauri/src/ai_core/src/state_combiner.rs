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

use crate::game_state_node::{GameStateNode, GameStatus};
use crate::state_evaluator::StateEvaluator;
use crate::state_predictor::StatePredictor;

/// StateCombiners merge together the results of [StatePredictor]s. Given the
/// set of predicted game states, they select the best *actual* game state to
/// use as the basis for a selection algorithm.
pub type StateCombiner<TNode, TEvaluator> = fn(&TNode, StatePredictor<TNode>, &TEvaluator) -> TNode;

/// A pessimistic [StateCombiner]. Evaluates each predicted game state and picks
/// the one which appears to be the worst-case scenario (i.e. has the lowest
/// [StateEvaluator] score) for the current player.
pub fn worst_case<TEvaluator, TNode>(
    node: &TNode,
    predictor: StatePredictor<TNode>,
    evaluator: &TEvaluator,
) -> TNode
where
    TEvaluator: StateEvaluator<TNode>,
    TNode: GameStateNode,
{
    let player = match node.status() {
        GameStatus::InProgress { current_turn } => current_turn,
        _ => panic!("Game is over"),
    };
    let mut worst = i32::MAX;
    let mut worst_state: Option<TNode> = None;
    for state in predictor(node) {
        let evaluation = evaluator.evaluate(&state, player);
        if evaluation < worst {
            worst = evaluation;
            worst_state = Some(state);
        }
    }

    worst_state.expect("No state found")
}

/// A [StateCombiner] which returns the first-available state prediction.
pub fn first<TEvaluator, TNode>(
    node: &TNode,
    predictor: StatePredictor<TNode>,
    _: &TEvaluator,
) -> TNode
where
    TEvaluator: StateEvaluator<TNode>,
    TNode: GameStateNode,
{
    predictor(node).next().expect("No state found")
}
