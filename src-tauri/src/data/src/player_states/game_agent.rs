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

use dyn_clone::DynClone;
use serde::{Deserialize, Serialize};

use crate::actions::game_action::GameAction;
use crate::actions::prompt_action::PromptAction;
use crate::core::primitives::PlayerName;
use crate::game_states::game_state::GameState;
use crate::game_states::oracle::Oracle;
use crate::prompts::prompt::Prompt;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GameAgent {
    pub search_duration: Duration,

    pub agent_type: AgentType,

    pub state_predictor: StatePredictor,

    pub state_combiner: StateCombiner,

    pub evaluator: AgentEvaluator,

    #[serde(skip)]
    pub implementation_reference: Option<Box<dyn GameAgentImpl>>,
}

impl GameAgent {
    pub fn implementation(&mut self) -> &mut dyn GameAgentImpl {
        self.implementation_reference
            .as_mut()
            .expect("Implementation reference not populated")
            .as_mut()
    }
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

/// Trait representing an AI agent playing in a game.
///
/// This is very similar to the normal 'Agent' trait, but we separate it out to
/// avoid crate circular dependency problems and add a little bit of
/// game-specific context.
pub trait GameAgentImpl: Debug + DynClone + Send {
    fn select_action(&mut self, game: GameState, player: PlayerName) -> GameAction;

    fn select_prompt_action(
        &self,
        game: &GameState,
        prompt: &Prompt,
        player: PlayerName,
    ) -> PromptAction;
}

dyn_clone::clone_trait_object!(GameAgentImpl);
