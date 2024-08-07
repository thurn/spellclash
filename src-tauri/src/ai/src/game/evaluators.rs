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

use data::card_states::zones::ZoneQueries;
use data::game_states::game_state::{GameState, GameStatus};
use data::player_states::player_state::PlayerQueries;
use primitives::game_primitives;
use rules::queries::player_queries;

use crate::core::state_evaluator::StateEvaluator;

#[derive(Debug, Clone)]
pub struct CustomHeuristicEvaluator;

impl StateEvaluator<GameState> for CustomHeuristicEvaluator {
    fn evaluate(&self, game: &GameState, player: game_primitives::PlayerName) -> i32 {
        match game.status {
            GameStatus::Playing => {
                let life = (game.player(player).life
                    - game.player(player_queries::next_player_after(game, player)).life)
                    as i32;
                if life != 0 {
                    return life;
                }

                game.battlefield(player).len() as i32
                    - game.battlefield(player_queries::next_player_after(game, player)).len() as i32
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
