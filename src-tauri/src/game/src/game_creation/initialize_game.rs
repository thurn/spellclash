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
use ai::core::first_available_action::FirstAvailableActionAlgorithm;
use ai::core::win_loss_evaluator::WinLossEvaluator;
use ai::game::evaluators::CustomHeuristicEvaluator;
use ai::monte_carlo::monte_carlo_search::{MonteCarloAlgorithm, RandomPlayoutEvaluator};
use ai::monte_carlo::uct1::Uct1;
use ai::tree_search::iterative_deepening_search::IterativeDeepeningSearch;
use data::card_definitions::definitions;
use data::card_states::zones::ZoneQueries;
use data::delegates::scope::AbilityScope;
use data::game_states::game_state::GameState;
use data::player_states::game_agent::{AgentType, GameAgent};
use data::player_states::player_state::{PlayerQueries, PlayerType};
use database::sqlite_database::SqliteDatabase;
use oracle::card_database;
use primitives::game_primitives::{AbilityId, PlayerName};
use utils::outcome;

pub fn run(database: SqliteDatabase, game: &mut GameState) {
    assert!(!game.initialized, "Game already initialized");
    game.initialized = true;
    card_database::populate(database, game);

    for player in enum_iterator::all::<PlayerName>() {
        if let PlayerType::Agent(agent) = &mut game.player_mut(player).player_type {
            initialize_agent(agent);
        }
    }

    let registry = definitions::registry();
    let all_card_ids = game.zones.all_cards().map(|card| card.id).collect::<Vec<_>>();
    for card_id in all_card_ids {
        outcome::execute(|| {
            let name = game.card(card_id)?.card_name;
            for (number, ability) in definitions::get(name).iterate_abilities() {
                let ability_id = AbilityId { card_id, number };
                let ability_scope = AbilityScope { ability_id };
                let card = game.card_mut(card_id)?;
                ability.add_properties(ability_scope, card);
                ability.add_card_events(ability_scope, &mut card.events);
                ability.add_global_events(ability_scope, &mut game.events);
            }
            outcome::OK
        });
    }
}

fn initialize_agent(agent: &mut GameAgent) {
    match agent.agent_type {
        AgentType::FirstAvailableAction => {
            agent.game_agent_reference = Some(Box::new(AgentData::omniscient(
                "FIRST_AVAILABLE_ACTION",
                FirstAvailableActionAlgorithm,
                WinLossEvaluator,
            )));
            agent.prompt_agent_reference = Some(Box::new(AgentData::omniscient(
                "FIRST_AVAILABLE_ACTION",
                FirstAvailableActionAlgorithm,
                WinLossEvaluator,
            )));
        }
        AgentType::TreeSearch(_) => {
            agent.game_agent_reference = Some(Box::new(AgentData::omniscient(
                "ITERATIVE_DEEPENING",
                IterativeDeepeningSearch,
                CustomHeuristicEvaluator,
            )));
            agent.prompt_agent_reference = Some(Box::new(AgentData::omniscient(
                "ITERATIVE_DEEPENING",
                IterativeDeepeningSearch,
                WinLossEvaluator,
            )));
        }
        AgentType::MonteCarlo(_) => {
            agent.game_agent_reference = Some(Box::new(AgentData::omniscient(
                "UCT1_10_000",
                MonteCarloAlgorithm {
                    child_score_algorithm: Uct1 {},
                    max_iterations: Some(10_000),
                    phantom_data: PhantomData,
                },
                RandomPlayoutEvaluator { evaluator: WinLossEvaluator, phantom_data: PhantomData },
            )));
            agent.prompt_agent_reference = Some(Box::new(AgentData::omniscient(
                "UCT1_10_000",
                MonteCarloAlgorithm {
                    child_score_algorithm: Uct1 {},
                    max_iterations: Some(10_000),
                    phantom_data: PhantomData,
                },
                RandomPlayoutEvaluator { evaluator: WinLossEvaluator, phantom_data: PhantomData },
            )));
        }
    }
}
