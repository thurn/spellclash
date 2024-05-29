// Copyright © Dungeon of the Diamond Queen 2024-present
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

use std::time::{Duration, Instant};

use crate::core::game_state_node::{GameStateNode, GameStatus};
use crate::core::selection_algorithm::SelectionAlgorithm;
use crate::core::state_combiner::StateCombiner;
use crate::core::state_evaluator::StateEvaluator;
use crate::core::state_predictor::StatePredictor;
use crate::core::{state_combiner, state_predictor};

#[derive(Debug, Clone, Copy)]
pub struct AgentConfig {
    /// Time at which the agent should complete its move selection.
    pub deadline: Instant,
    /// If true, the agent should panic if it has not completed its search after
    /// the deadline is exceeded
    pub panic_on_search_timeout: bool,
}

impl AgentConfig {
    pub fn with_deadline(seconds: u64) -> Self {
        Self {
            deadline: Instant::now() + Duration::from_secs(seconds),
            panic_on_search_timeout: false,
        }
    }
}

/// An AI Agent for a given game state, any system capable of selecting valid
/// game actions for a player.
///
/// This trait is almost always used as a wrapper for [AgentData] with fewer
/// type parameters, but it can also be implemented on its own for novel agent
/// types.
pub trait Agent<TNode>: Send
where
    TNode: GameStateNode,
{
    /// Name of this agent, used for debugging.
    fn name(&self) -> &'static str;

    /// Select an action for the current player to take in the `node` game
    /// state. Should attempt to return a result before the [AgentConfig]'s
    /// `deadline`.
    fn pick_action(&self, config: AgentConfig, node: &TNode) -> TNode::Action;

    /// If true, this agent will not be queried for actions on their turn.
    fn inactive(&self) -> bool {
        false
    }
}

/// A tuple of various pieces needed to perform agent action selection.
///
/// Uses a [StatePredictor] to identify possible concrete game states, a
/// [SelectionAlgorithm] to decide on actions to take, a [StateEvaluator] to
/// score different sates, and a [StateCombiner] to incorporate state
/// predictions together. Refer to the documentation on each individual type for
/// more information about their role in the system.
pub struct AgentData<TSelector, TEvaluator, TNode>
where
    TSelector: SelectionAlgorithm,
    TEvaluator: StateEvaluator<TNode>,
    TNode: GameStateNode + 'static,
{
    /// Name of this agent, used for debugging.
    pub name: &'static str,
    /// Returns an iterator over *possible* states this game might be in, given
    /// the presence of hidden information the Agent is not privy to.
    pub predictor: StatePredictor<TNode>,
    /// Selects a game action given a specific game state.
    pub selector: TSelector,
    /// Produces a numeric score ranking the desirability of a game state.
    pub evaluator: TEvaluator,
    /// Given a set of *possible* game states from the predictor, selects an
    /// actual game state to run the selection algorithm form.
    pub combiner: StateCombiner<TNode, TEvaluator>,
}

impl<TSelector, TEvaluator, TNode> AgentData<TSelector, TEvaluator, TNode>
where
    TSelector: SelectionAlgorithm,
    TEvaluator: StateEvaluator<TNode>,
    TNode: GameStateNode + 'static,
{
    /// Creates an Agent using the [state_predictor::omniscient] state
    /// predictor, giving it perfect knowledge of all hidden game state. Uses
    /// the [state_combiner::worst_case] combiner.
    pub const fn omniscient(
        name: &'static str,
        selector: TSelector,
        evaluator: TEvaluator,
    ) -> Self {
        Self {
            name,
            predictor: state_predictor::omniscient,
            selector,
            evaluator,
            combiner: state_combiner::worst_case,
        }
    }
}

impl<TSelector, TEvaluator, TNode> Agent<TNode> for AgentData<TSelector, TEvaluator, TNode>
where
    TSelector: SelectionAlgorithm,
    TEvaluator: StateEvaluator<TNode>,
    TNode: GameStateNode,
{
    fn name(&self) -> &'static str {
        self.name
    }

    fn pick_action(&self, deadline: AgentConfig, node: &TNode) -> TNode::Action {
        let player = match node.status() {
            GameStatus::InProgress { current_turn } => current_turn,
            _ => panic!("Game is over"),
        };
        let node = (self.combiner)(node, self.predictor, &self.evaluator);
        self.selector.pick_action(deadline, &node, &self.evaluator, player)
    }
}
