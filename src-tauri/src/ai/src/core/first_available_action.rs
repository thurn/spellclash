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

use crate::core::agent::AgentConfig;
use crate::core::game_state_node::GameStateNode;
use crate::core::selection_algorithm::SelectionAlgorithm;
use crate::core::state_evaluator::StateEvaluator;

#[derive(Debug, Clone)]
pub struct FirstAvailableActionAlgorithm;

impl<TStateNode, TEvaluator> SelectionAlgorithm<TStateNode, TEvaluator>
    for FirstAvailableActionAlgorithm
where
    TStateNode: GameStateNode,
    TEvaluator: StateEvaluator<TStateNode>,
{
    fn pick_action(
        &mut self,
        _config: AgentConfig,
        node: &TStateNode,
        _evaluator: &TEvaluator,
        player: TStateNode::PlayerName,
    ) -> TStateNode::Action
    where
        TStateNode: GameStateNode,
        TEvaluator: StateEvaluator<TStateNode>,
    {
        node.legal_actions(player).next().expect("No legal actions for player")
    }
}
