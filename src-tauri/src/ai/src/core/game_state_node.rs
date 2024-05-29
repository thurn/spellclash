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

use std::fmt::Debug;
use std::hash::Hash;

use enumset::{EnumSet, EnumSetType};

#[derive(Debug, PartialEq, Eq)]
pub enum GameStatus<TPlayer: EnumSetType + Debug> {
    /// Game is still ongoing, it is TPlayer's turn to act.
    InProgress { current_turn: TPlayer },
    /// Game has ended, the indicated players have won
    Completed { winners: EnumSet<TPlayer> },
}

/// A generic game state used by an AI algorithm.
///
/// Keeping the AI search algorithm implementation generic when possible is
/// useful for nim. We use a much simpler game with a known-optimal
/// strategy (the game of Nim) to sanity-check that the AI implementations are
/// doing broadly correct things.
pub trait GameStateNode {
    /// A game action to transition the game to a new state.
    type Action: Eq + Copy + Hash + Debug;

    /// A player in the game.
    type PlayerName: EnumSetType + Debug;

    /// Create a copy of this search node to be mutated by selection algorithms.
    /// A basic implementation of this would be to simply call `.clone()`, but
    /// sometimes parts of the game state are only for display and are not
    /// relevant for selection algorithms.
    fn make_copy(&self) -> Self;

    /// Returns the status for the game, either the player whose turn it is or
    /// the player who won.
    fn status(&self) -> GameStatus<Self::PlayerName>;

    /// Returns the player whose turn it currently is, or an error if the game
    /// has ended.
    fn current_turn(&self) -> Self::PlayerName {
        match self.status() {
            GameStatus::InProgress { current_turn } => current_turn,
            GameStatus::Completed { .. } => panic!("Error: Game is over"),
        }
    }

    /// Returns an iterator over actions that the provided `player` can legally
    /// take in the current game state.
    ///
    /// Should an error if no actions are available.
    fn legal_actions<'a>(
        &'a self,
        player: Self::PlayerName,
    ) -> Box<dyn Iterator<Item = Self::Action> + 'a>;

    /// Apply the result of a given action to this game state.
    fn execute_action(&mut self, player: Self::PlayerName, action: Self::Action);
}
