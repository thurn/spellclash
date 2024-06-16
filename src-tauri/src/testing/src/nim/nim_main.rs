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

use std::io;

use ai::core::agent::{Agent, AgentConfig, AgentData};
use ai::core::game_state_node::{GameStateNode, GameStatus};
use ai::tree_search::single_level::SingleLevel;
use clap::{Parser, ValueEnum};
use testing::nim::nim_agents::{
    NIM_ALPHA_BETA_AGENT, NIM_MINIMAX_AGENT, NIM_PERFECT_AGENT, NIM_UCT1_AGENT,
};
use testing::nim::nim_game::{
    nim_sum, NimAction, NimPerfectEvaluator, NimPile, NimPlayer, NimState,
};

#[derive(Parser)]
#[clap()]
pub struct Args {
    #[arg(value_enum)]
    pub player_one: NimAgentName,
    #[arg(value_enum)]
    pub player_two: NimAgentName,
    #[arg(long, default_value_t = 5)]
    pub stack_size: u32,
    #[arg(long, default_value_t = 5)]
    pub move_time: u64,
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
pub enum NimAgentName {
    Human,
    Perfect,
    Minimax,
    AlphaBeta,
    UCT1,
}

pub fn main() {
    let args = Args::parse();
    println!("Welcome to the Game of Nim");
    let nim = NimState::new(args.stack_size);
    run_game_loop(nim, args.move_time, get_agent(args.player_one), get_agent(args.player_two))
}

fn get_agent(name: NimAgentName) -> Box<dyn Agent<NimState>> {
    match name {
        NimAgentName::Human => Box::new(NimHumanAgent {}),
        NimAgentName::Perfect => Box::new(NIM_PERFECT_AGENT),
        NimAgentName::Minimax => Box::new(NIM_MINIMAX_AGENT),
        NimAgentName::AlphaBeta => Box::new(NIM_ALPHA_BETA_AGENT),
        NimAgentName::UCT1 => Box::new(NIM_UCT1_AGENT),
    }
}

fn run_game_loop(
    mut nim: NimState,
    move_time: u64,
    mut player_one: Box<dyn Agent<NimState>>,
    mut player_two: Box<dyn Agent<NimState>>,
) {
    loop {
        print_optimal_action(&nim, player_one.name());
        println!("{}", nim);
        let p1_action = player_one.pick_action(AgentConfig::with_deadline(move_time), &nim);
        println!("<<{}>> takes {} from {}", player_one.name(), p1_action.amount, p1_action.pile);
        nim.execute_action(NimPlayer::One, p1_action);
        check_game_over(&nim);

        print_optimal_action(&nim, player_two.name());
        println!("{}", nim);

        let p2_action = player_two.pick_action(AgentConfig::with_deadline(move_time), &nim);
        println!("<<{}>> takes {} from {}", player_two.name(), p2_action.amount, p2_action.pile);
        nim.execute_action(NimPlayer::Two, p2_action);
        check_game_over(&nim);
    }
}

fn print_optimal_action(state: &NimState, player_name: &str) {
    if nim_sum(state) == 0 {
        println!("  (Game is unwinnable for {} with optimal play)", player_name);
    } else {
        let mut perfect = AgentData::omniscient("PERFECT", SingleLevel {}, NimPerfectEvaluator {});
        let action = perfect.pick_action(AgentConfig::with_deadline(5), state);
        println!("  (Optimal play for {} is {} take {})", player_name, action.pile, action.amount);
    }
}

fn check_game_over(game: &NimState) {
    if let GameStatus::Completed { winners } = game.status() {
        println!("Game Over. Winner: {:?}", winners);
        std::process::exit(0)
    }
}

struct NimHumanAgent;

impl Agent<NimState> for NimHumanAgent {
    fn name(&self) -> &'static str {
        "HUMAN"
    }

    fn pick_action(&mut self, _: AgentConfig, state: &NimState) -> NimAction {
        println!("\n>>> Input your action, e.g. 'a2' or 'b3'");

        let mut input_text = String::new();
        io::stdin().read_line(&mut input_text).expect("Error reading line");

        let trimmed = input_text.trim();
        assert_eq!(trimmed.len(), 2);
        let characters = trimmed.chars().collect::<Vec<_>>();
        let pile = match characters[0] {
            'a' => NimPile::PileA,
            'b' => NimPile::PileB,
            'c' => NimPile::PileC,
            _ => panic!("Input must be a, b, or c"),
        };
        let amount = characters[1].to_digit(10).expect("Input must be 1-9");
        assert!(amount > 0);
        assert!(amount <= state.piles[&pile]);

        NimAction { pile, amount }
    }
}
