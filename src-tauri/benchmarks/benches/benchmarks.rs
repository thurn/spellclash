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

use ai::testing::test_games;
use criterion::{criterion_group, criterion_main, Criterion};
use data::actions::game_action::GameAction;
use data::core::primitives::PlayerName;
use data::game_states::game_state::GameState;
use rules::legality::legal_actions;
use rules::legality::legal_actions::LegalActions;

criterion_main!(benches);
criterion_group!(benches, legal_actions);

pub fn legal_actions(c: &mut Criterion) {
    fn benchmark(game: &GameState) -> Vec<GameAction> {
        legal_actions::compute(&game, PlayerName::One, LegalActions {
            include_interface_actions: false,
        })
    }

    let mut group = c.benchmark_group("legal_actions");
    let game = test_games::vanilla_game_scenario();
    assert_eq!(benchmark(&game).len(), 4);
    group.bench_function("legal_actions", |b| {
        b.iter(|| {
            benchmark(&game);
        })
    });
}
