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

use data::core::primitives;
use data::game_states::game_state::{GameState, GameStatus};
use data::player_states::player_state::PlayerQueries;
use rules::queries::player_queries;

use crate::core::state_evaluator::StateEvaluator;

pub struct LifeTotalEvaluator;

impl StateEvaluator<GameState> for LifeTotalEvaluator {
    fn evaluate(&self, game: &GameState, player: primitives::PlayerName) -> i32 {
        match game.status {
            GameStatus::Playing => {
                (game.player(player).life
                    - game.player(player_queries::next_player_after(game, player)).life)
                    as i32
            }
            GameStatus::GameOver { winners } => {
                if winners.contains(player) {
                    i32::MAX
                } else {
                    i32::MIN
                }
            }
            _ => 0,
        }
    }
}
