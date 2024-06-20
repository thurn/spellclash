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
use std::time::{Duration, Instant};

use ai::core::state_evaluator::StateEvaluator;
use ai::core::win_loss_evaluator::WinLossEvaluator;
use ai::game::agents;
use ai::game::agents::AgentName;
use ai::monte_carlo::monte_carlo_search::RandomPlayoutEvaluator;
use criterion::{criterion_group, criterion_main, BatchSize, Criterion};
use data::actions::game_action::GameAction;
use data::card_definitions::card_name;
use data::card_states::zones::ZoneQueries;
use data::core::primitives::PlayerName;
use data::decks::deck_name;
use rules::action_handlers::actions;
use rules::action_handlers::actions::ExecuteAction;
use rules::legality::legal_actions;
use rules::legality::legal_actions::LegalActions;
use testing::ai_testing::test_games;
use tracing::{subscriber, Level};
use utils::command_line;
use utils::command_line::CommandLine;

criterion_main!(benches);
criterion_group!(benches, green_vanilla, uct1, alpha_beta, random_playout_evaluator);

pub fn green_vanilla(c: &mut Criterion) {
    let mut group = c.benchmark_group("green_vanilla");
    command_line::FLAGS.set(CommandLine::default()).ok();

    let game = test_games::vanilla_game_scenario();
    group.bench_function("legal_actions", |b| {
        b.iter(|| {
            legal_actions::compute(&game, PlayerName::One, LegalActions { for_human_player: false })
        })
    });

    group.bench_function("play_creature", |b| {
        b.iter_batched(
            || {
                let game = test_games::vanilla_game_scenario();
                let creature_id = *game
                    .hand(PlayerName::One)
                    .iter()
                    .find(|&card_id| game.card(*card_id).card_name == card_name::KALONIAN_TUSKER)
                    .expect("Creature not found");
                (game, creature_id)
            },
            |(mut game, creature_id)| {
                actions::execute(
                    &mut game,
                    PlayerName::One,
                    GameAction::ProposePlayingCard(creature_id),
                    ExecuteAction { skip_undo_tracking: true, validate: false },
                )
            },
            BatchSize::LargeInput,
        )
    });

    group.bench_function("play_land", |b| {
        b.iter_batched(
            || {
                let game = test_games::vanilla_game_scenario();
                let creature_id = *game
                    .hand(PlayerName::One)
                    .iter()
                    .find(|&card_id| game.card(*card_id).card_name == card_name::FOREST)
                    .expect("Land not found");
                (game, creature_id)
            },
            |(mut game, creature_id)| {
                actions::execute(
                    &mut game,
                    PlayerName::One,
                    GameAction::ProposePlayingCard(creature_id),
                    ExecuteAction { skip_undo_tracking: false, validate: false },
                )
            },
            BatchSize::LargeInput,
        )
    });
}

pub fn uct1(c: &mut Criterion) {
    command_line::FLAGS.set(CommandLine::default()).ok();
    let mut group = c.benchmark_group("uct1");
    let game = test_games::vanilla_game_scenario();

    let error_subscriber = tracing_subscriber::fmt().with_max_level(Level::ERROR).finish();
    subscriber::with_default(error_subscriber, || {
        group.bench_function("uct1", |b| {
            b.iter(|| {
                let agent = agents::get_agent(AgentName::Uct1Iterations1);
                agent.pick_action(Instant::now() + Duration::from_secs(100), &game)
            })
        });
    });
}

pub fn random_playout_evaluator(c: &mut Criterion) {
    command_line::FLAGS.set(CommandLine::default()).ok();
    let mut group = c.benchmark_group("random_playout_evaluator");
    group.confidence_level(0.99);
    let game = test_games::vanilla_game_scenario();
    let evaluator =
        RandomPlayoutEvaluator { evaluator: WinLossEvaluator, phantom_data: PhantomData };
    let error_subscriber = tracing_subscriber::fmt().with_max_level(Level::ERROR).finish();
    subscriber::with_default(error_subscriber, || {
        group.bench_function("green_vanilla", |b| {
            b.iter(|| evaluator.evaluate(&game, PlayerName::One))
        });
    });

    let game = test_games::create(deck_name::ALL_DANDANS);
    let error_subscriber = tracing_subscriber::fmt().with_max_level(Level::ERROR).finish();
    subscriber::with_default(error_subscriber, || {
        group.bench_function("all_dandans", |b| {
            b.iter(|| evaluator.evaluate(&game, PlayerName::One));
        });
    });
}

pub fn alpha_beta(c: &mut Criterion) {
    command_line::FLAGS.set(CommandLine::default()).ok();
    let mut group = c.benchmark_group("alpha_beta");
    let game = test_games::vanilla_game_scenario();

    group.bench_function("alpha_beta", |b| {
        b.iter(|| {
            let agent = agents::get_agent(AgentName::AlphaBetaDepth5);
            agent.pick_action(Instant::now() + Duration::from_secs(100), &game)
        })
    });
}
