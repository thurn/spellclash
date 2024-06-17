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

use petgraph::Graph;

#[derive(Debug, Clone)]
pub struct SearchNode<TPlayerName> {
    /// Player who acted to create this node
    pub player: TPlayerName,
    /// Q(v): Total reward of all playouts that passed through this state
    pub total_reward: f64,
    /// N(v): Visit count for this node
    pub visit_count: u32,
}

#[derive(Debug, Clone)]
pub struct SearchEdge<TAction> {
    pub action: TAction,
}

pub type SearchGraph<TPlayerName, TAction> = Graph<SearchNode<TPlayerName>, SearchEdge<TAction>>;

#[derive(Debug, Clone)]
pub struct MonteCarloAgentState<TPlayerName, TAction> {
    pub graph: SearchGraph<TPlayerName, TAction>,
}
