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

use std::marker::PhantomData;

use clap::ValueEnum;
use data::game_states::game_state::GameState;

use crate::core::agent::{Agent, AgentData};
use crate::core::first_available_action::FirstAvailableActionAlgorithm;
use crate::core::win_loss_evaluator::WinLossEvaluator;
use crate::game::evaluators::CustomHeuristicEvaluator;
use crate::monte_carlo::monte_carlo_search::{MonteCarloAlgorithm, RandomPlayoutEvaluator};
use crate::monte_carlo::uct1::Uct1;
use crate::tree_search::alpha_beta::AlphaBetaAlgorithm;
use crate::tree_search::iterative_deepening_search::IterativeDeepeningSearch;

#[derive(ValueEnum, Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub enum AgentName {
    AlphaBetaDepth5,
    AlphaBetaDepth25,
    IterativeDeepening,
    Uct1,
    Uct1Iterations1,
    Uct1Iterations250,
    Uct1Iterations10_000,
    FirstAvailableAction,
}

pub fn get_agent(name: AgentName) -> Box<dyn Agent<GameState>> {
    match name {
        AgentName::AlphaBetaDepth5 => Box::new(AgentData::omniscient(
            "ALPHA_BETA_5",
            AlphaBetaAlgorithm { search_depth: 5 },
            CustomHeuristicEvaluator,
        )),
        AgentName::AlphaBetaDepth25 => Box::new(AgentData::omniscient(
            "ALPHA_BETA_25",
            AlphaBetaAlgorithm { search_depth: 25 },
            CustomHeuristicEvaluator,
        )),
        AgentName::IterativeDeepening => Box::new(AgentData::omniscient(
            "ITERATIVE_DEEPENING",
            IterativeDeepeningSearch,
            CustomHeuristicEvaluator,
        )),
        AgentName::Uct1 => Box::new(AgentData::omniscient(
            "UCT1",
            MonteCarloAlgorithm {
                child_score_algorithm: Uct1 {},
                max_iterations: None,
                phantom_data: PhantomData,
            },
            RandomPlayoutEvaluator { evaluator: WinLossEvaluator, phantom_data: PhantomData },
        )),
        AgentName::Uct1Iterations1 => Box::new(AgentData::omniscient(
            "UCT1_1",
            MonteCarloAlgorithm {
                child_score_algorithm: Uct1 {},
                max_iterations: Some(1),
                phantom_data: PhantomData,
            },
            RandomPlayoutEvaluator { evaluator: WinLossEvaluator, phantom_data: PhantomData },
        )),
        AgentName::Uct1Iterations250 => Box::new(AgentData::omniscient(
            "UCT1_250",
            MonteCarloAlgorithm {
                child_score_algorithm: Uct1 {},
                max_iterations: Some(250),
                phantom_data: PhantomData,
            },
            RandomPlayoutEvaluator { evaluator: WinLossEvaluator, phantom_data: PhantomData },
        )),
        AgentName::Uct1Iterations10_000 => Box::new(AgentData::omniscient(
            "UCT1_10_000",
            MonteCarloAlgorithm {
                child_score_algorithm: Uct1 {},
                max_iterations: Some(10_000),
                phantom_data: PhantomData,
            },
            RandomPlayoutEvaluator { evaluator: WinLossEvaluator, phantom_data: PhantomData },
        )),
        AgentName::FirstAvailableAction => Box::new(AgentData::omniscient(
            "FIRST_AVAILABLE_ACTION",
            FirstAvailableActionAlgorithm,
            WinLossEvaluator,
        )),
    }
}
