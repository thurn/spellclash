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

use data::core::primitives::PlayerName;
use data::game_states::game_state::GameState;

/// Returns the next player in turn order after the given [PlayerName].
///
/// This may vary based on game configuration, e.g. in multiplayer games.
pub fn next_player_after(game: &GameState, player: PlayerName) -> PlayerName {
    match game.configuration.all_players.len() {
        2 => match player {
            PlayerName::One => PlayerName::Two,
            PlayerName::Two => PlayerName::One,
            _ => panic!("{player:?} is not a player in this game"),
        },
        3 => match player {
            PlayerName::One => PlayerName::Two,
            PlayerName::Two => PlayerName::Three,
            PlayerName::Three => PlayerName::One,
            _ => panic!("{player:?} is not a player in this game"),
        },
        4 => match player {
            PlayerName::One => PlayerName::Two,
            PlayerName::Two => PlayerName::Three,
            PlayerName::Three => PlayerName::Four,
            PlayerName::Four => PlayerName::One,
        },
        _ => panic!("Unsupported player count"),
    }
}

/// Returns the number of lands the indicated `player` can still play this turn.
pub fn land_plays_remaining(game: &GameState, player: PlayerName) -> usize {
    if game.turn.active_player == player {
        1
    } else {
        0
    }
}
