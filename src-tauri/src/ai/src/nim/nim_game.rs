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
use std::fmt;
use std::fmt::{Display, Formatter};

use enumset::{EnumSet, EnumSetType};

use crate::core::agent::{Agent, AgentConfig};
use crate::core::game_state_node::{GameStateNode, GameStatus};
use crate::core::state_evaluator::StateEvaluator;

/// Asserts that a given `agent` picks an optimal game action for the provided
/// game state.
pub fn assert_perfect(state: &NimState, agent: &impl Agent<NimState>) {
    assert_perfect_in_seconds(state, agent, 60)
}

/// Equivalent to [assert_perfect] with a short timeout in seconds.
pub fn assert_perfect_short(state: &NimState, agent: &impl Agent<NimState>) {
    assert_perfect_in_seconds(state, agent, 1)
}

/// Equivalent to [assert_perfect] with a manually-specified deadline in
/// seconds.
pub fn assert_perfect_in_seconds(state: &NimState, agent: &impl Agent<NimState>, seconds: u64) {
    let current = state.current_turn();
    let result = agent.pick_action(AgentConfig::with_deadline(seconds), state);
    let mut copy = state.make_copy();
    copy.execute_action(current, result);
    assert_eq!(1, NimPerfectEvaluator {}.evaluate(&copy, current));
}

/// Evaluator which returns 1 if the current game state is a winning state the
/// player and -1 otherwise.
pub struct NimPerfectEvaluator {}

impl StateEvaluator<NimState> for NimPerfectEvaluator {
    fn evaluate(&self, state: &NimState, player: NimPlayer) -> i32 {
        let count = nim_sum(state);
        if player == state.turn {
            if count == 0 {
                -1
            } else {
                1
            }
        } else if count == 0 {
            1
        } else {
            -1
        }
    }
}

#[derive(Hash, Ord, PartialOrd, Debug, EnumSetType)]
pub enum NimPlayer {
    One,
    Two,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub enum NimPile {
    PileA,
    PileB,
    PileC,
}

impl Display for NimPile {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", match self {
            Self::PileA => "Pile A",
            Self::PileB => "Pile B",
            Self::PileC => "Pile C",
        })
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub struct NimAction {
    pub pile: NimPile,
    pub amount: u32,
}

#[derive(Debug, Clone)]
pub struct NimState {
    pub piles: HashMap<NimPile, u32>,
    pub turn: NimPlayer,
}

impl NimState {
    pub fn new(pile_size: u32) -> Self {
        Self::new_with_piles(pile_size, pile_size, pile_size)
    }

    pub fn new_with_piles(a: u32, b: u32, c: u32) -> Self {
        let mut piles = HashMap::new();
        piles.insert(NimPile::PileA, a);
        piles.insert(NimPile::PileB, b);
        piles.insert(NimPile::PileC, c);
        Self { piles, turn: NimPlayer::One }
    }
}

impl Display for NimState {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Piles: A[{}] B[{}] C[{}]",
            self.piles[&NimPile::PileA],
            self.piles[&NimPile::PileB],
            self.piles[&NimPile::PileC]
        )
    }
}

fn all_piles() -> Vec<NimPile> {
    vec![NimPile::PileA, NimPile::PileB, NimPile::PileC]
}

pub fn nim_sum(state: &NimState) -> u32 {
    state.piles[&NimPile::PileA] ^ state.piles[&NimPile::PileB] ^ state.piles[&NimPile::PileC]
}

impl GameStateNode for NimState {
    type Action = NimAction;
    type PlayerName = NimPlayer;

    fn make_copy(&self) -> Self {
        self.clone()
    }

    fn status(&self) -> GameStatus<NimPlayer> {
        if all_piles().iter().all(|pile| self.piles[pile] == 0) {
            GameStatus::Completed {
                winners: match self.turn {
                    NimPlayer::One => EnumSet::only(NimPlayer::Two),
                    NimPlayer::Two => EnumSet::only(NimPlayer::One),
                },
            }
        } else {
            GameStatus::InProgress { current_turn: self.turn }
        }
    }

    fn legal_actions<'a>(&'a self, _: NimPlayer) -> Box<dyn Iterator<Item = NimAction> + 'a> {
        Box::new(
            all_piles().into_iter().flat_map(|pile| {
                (1..=self.piles[&pile]).map(move |amount| NimAction { pile, amount })
            }),
        )
    }

    fn execute_action(&mut self, player: NimPlayer, action: NimAction) {
        assert_eq!(self.status(), GameStatus::InProgress { current_turn: player });
        assert!(action.amount <= self.piles[&action.pile]);
        self.piles.entry(action.pile).and_modify(|amount| *amount -= action.amount);
        self.turn = match player {
            NimPlayer::One => NimPlayer::Two,
            NimPlayer::Two => NimPlayer::One,
        };
    }
}
