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

use ai::core::agent::AgentData;
use ai::core::win_loss_evaluator::WinLossEvaluator;
use ai::monte_carlo::monte_carlo_search::{MonteCarloAlgorithm, RandomPlayoutEvaluator};
use ai::monte_carlo::uct1::Uct1;
use ai::tree_search::alpha_beta::AlphaBetaAlgorithm;
use ai::tree_search::minimax::MinimaxAlgorithm;
use ai::tree_search::single_level::SingleLevel;

use crate::nim::nim_game::{NimPerfectEvaluator, NimState};

/// Agent which always makes optimal moves
pub const NIM_PERFECT_AGENT: AgentData<SingleLevel, NimPerfectEvaluator, NimState> =
    AgentData::omniscient("PERFECT", SingleLevel {}, NimPerfectEvaluator {});

pub const NIM_MINIMAX_AGENT: AgentData<MinimaxAlgorithm, WinLossEvaluator, NimState> =
    AgentData::omniscient("MINIMAX", MinimaxAlgorithm { search_depth: 25 }, WinLossEvaluator {});

pub const NIM_ALPHA_BETA_AGENT: AgentData<AlphaBetaAlgorithm, WinLossEvaluator, NimState> =
    AgentData::omniscient("ALPHA_BETA", AlphaBetaAlgorithm { search_depth: 25 }, WinLossEvaluator);

pub const NIM_UCT1_AGENT: AgentData<
    MonteCarloAlgorithm<Uct1>,
    RandomPlayoutEvaluator<NimState, WinLossEvaluator>,
    NimState,
> = AgentData::omniscient(
    "UCT1",
    MonteCarloAlgorithm { child_score_algorithm: Uct1 {}, max_iterations: None },
    RandomPlayoutEvaluator { evaluator: WinLossEvaluator, phantom_data: PhantomData },
);
