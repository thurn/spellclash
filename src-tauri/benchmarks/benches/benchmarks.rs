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

use ai::game::agents::AgentName;
use ai::testing::run_matchup;
use ai::testing::run_matchup::{MatchupArgs, Verbosity};
use criterion::measurement::WallTime;
use criterion::{criterion_group, criterion_main, BenchmarkGroup, Criterion};

criterion_main!(benches);
criterion_group!(benches, uct1);

pub fn uct1(c: &mut Criterion) {
    let mut group = start(c, "uct1");
    group.measurement_time(Duration::from_secs(30));
    group.bench_function("UCT1", |b| {
        b.iter(|| {
            run_matchup::run_with_args(&MatchupArgs {
                user: AgentName::Uct1Iterations250,
                opponent: AgentName::Uct1Iterations250,
                move_time_ms: 1000,
                matches: 1,
                verbosity: Verbosity::None,
            })
        })
    });
}

fn start<'a>(c: &'a mut Criterion, s: &'static str) -> BenchmarkGroup<'a, WallTime> {
    c.benchmark_group(s)
}
