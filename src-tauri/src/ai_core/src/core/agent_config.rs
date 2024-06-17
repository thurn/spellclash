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

use std::time::Duration;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentConfig {
    pub search_duration: Duration,

    pub agent_type: AgentType,

    pub state_predictor: StatePredictor,

    pub state_combiner: StateCombiner,

    pub evaluator: AgentEvaluator,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum StatePredictor {
    Omniscient,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum StateCombiner {
    First,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AgentType {
    FirstAvailableAction,
    TreeSearch(TreeSearchAgent),
    MonteCarlo(MonteCarloAgent),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TreeSearchAgent {
    pub max_depth: Option<usize>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MonteCarloAgent {
    pub child_score_algorithm: ChildScoreAlgorithm,
    pub max_iterations: Option<usize>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AgentEvaluator {
    CustomHeuristics,
    WinLoss,
    RandomPlayout(Box<AgentEvaluator>),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ChildScoreAlgorithm {
    Uct1,
}
