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

use std::collections::HashMap;

use ai::game::agents::AgentName;
use ai::testing::run_matchup;
use ai::testing::run_matchup::Verbosity;
use clap::Parser;
use itertools::Itertools;

#[derive(Parser)]
#[clap()]
pub struct TournamentArgs {
    #[arg(value_enum)]
    pub players: Vec<AgentName>,
    /// Maximum time in seconds for each agent to use for moves.
    #[arg(long, default_value_t = 1)]
    pub move_time: u64,
}

pub fn main() {
    let args = TournamentArgs::parse();
    println!("Running Tournament: {:?}", args.players);
    let mut scores = HashMap::new();
    for combination in args.players.iter().combinations(2) {
        println!("Running 10-game match between {:?} and {:?}", combination[0], combination[1]);
        run_tournament_match(&args, &mut scores, *combination[0], *combination[1]);
        println!("\nUpdated Scores:");
        print_scores(&scores);
    }

    println!("\nTournament Results:");
    print_scores(&scores);
}

fn run_tournament_match(
    args: &TournamentArgs,
    scores: &mut HashMap<AgentName, i32>,
    p1: AgentName,
    p2: AgentName,
) {
    for _ in 0..5 {
        // Players play the game from both sides in order to ensure fairness
        let mut game1 = new_round::create_play_phase(&mut rand::thread_rng(), 3);
        let mut game2 = game1.clone();
        add_winner(
            scores,
            run_matchup::run_match(p1, p2, &mut game1, args.move_time, Verbosity::Matches, false),
        );
        add_winner(
            scores,
            run_matchup::run_match(p2, p1, &mut game2, args.move_time, Verbosity::Matches, false),
        );
    }
}

fn add_winner(current: &mut HashMap<AgentName, i32>, update: HashMap<AgentName, i32>) {
    for (agent, score) in update {
        current.entry(agent).and_modify(|s| *s += score).or_insert(score);
    }
}

fn print_scores(scores: &HashMap<AgentName, i32>) {
    let mut result = scores.iter().collect::<Vec<_>>();
    result.sort_by_key(|(_, score)| *score);
    result.reverse();
    for (agent, score) in result {
        println!("{:?} scored {:?}", agent, score);
    }
}
