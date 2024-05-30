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

use std::time::{Duration, Instant};

use clap::{Parser, ValueEnum};
use data::core::primitives::PlayerName;
use data::game_states::game_state::GameState;

use crate::core::agent::AgentConfig;
use crate::core::game_state_node::{GameStateNode, GameStatus};
use crate::game::agents;
use crate::game::agents::AgentName;

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
pub enum Verbosity {
    None,
    Matches,
    Actions,
}

#[derive(Parser)]
#[clap()]
pub struct MatchupArgs {
    #[arg(value_enum)]
    pub user: AgentName,
    #[arg(value_enum)]
    pub opponent: AgentName,
    /// Maximum time in milliseconds for each agent to use for moves.
    #[arg(long, default_value_t = 1)]
    pub move_time_ms: u64,
    /// Number of matches to run between these two named players
    #[arg(long, default_value_t = 1)]
    pub matches: u64,
    /// How much log output to produce while running
    #[arg(long, value_enum, default_value_t = Verbosity::Matches)]
    pub verbosity: Verbosity,
}

pub fn run_with_args(args: &MatchupArgs) {
    let user = agents::get_agent(args.user);
    let opponent = agents::get_agent(args.opponent);

    for i in 1..=args.matches {
        if args.verbosity >= Verbosity::Matches {
            println!(">>> Running match {} between {} and {}", i, user.name(), opponent.name());
        }
        let mut game = new_game();
        run_match(args.user, args.opponent, &mut game, args.move_time_ms, args.verbosity);
    }
}

pub fn run_match(
    user_agent: AgentName,
    opponent_agent: AgentName,
    game: &mut GameState,
    move_time_ms: u64,
    verbosity: Verbosity,
) -> AgentName {
    let user = agents::get_agent(user_agent);
    let opponent = agents::get_agent(opponent_agent);
    if verbosity > Verbosity::None {
        println!("Starting game");
    }

    loop {
        match game.status() {
            GameStatus::InProgress { current_turn } => {
                let agent = if current_turn == PlayerName::One { &user } else { &opponent };
                let config =
                    AgentConfig { deadline: Instant::now() + Duration::from_millis(move_time_ms) };
                let action = agent.pick_action(config, game);
                game.execute_action(current_turn, action.clone());
                clear_action_line(verbosity);
                if verbosity > Verbosity::None {
                    println!("{} performs action {:?}", agent.name(), action);
                }
            }
            GameStatus::Completed { winners } => {
                if verbosity >= Verbosity::Matches {
                    clear_action_line(verbosity);
                    println!("Match ended with winners {:?}", winners);
                }
                return if winners.contains(PlayerName::One) { user_agent } else { opponent_agent };
            }
        }
    }
}

fn new_game() -> GameState {
    todo!("")
}

fn clear_action_line(verbosity: Verbosity) {
    if verbosity == Verbosity::Matches {
        print!("\x1B[1F"); // Moves cursor to beginning of previous line, 1 line up
        print!("\x1B[2K"); // Erase the entire line
    }
}
