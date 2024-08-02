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

use data::game_states::game_state::GameState;
use enumset::EnumSet;
use primitives::game_primitives::{PlayerName, Source};
use rules::queries::player_queries;

/// Possible sets of players that can be targeted
#[derive(Clone, Copy, Eq, PartialEq, Debug)]
pub enum PlayerSet {
    /// Search the set of all players in the game
    AllPlayers,

    /// Search the cards controlled by this ability's controller
    You,

    /// Search the set of opponents of this ability's controller
    Opponents,
}

/// Returns the set of players represented by the provided [PlayerSet].
pub fn players_in_set(
    game: &GameState,
    controller: PlayerName,
    source: Source,
    set: PlayerSet,
) -> EnumSet<PlayerName> {
    match set {
        PlayerSet::AllPlayers => player_queries::all_players(game),
        PlayerSet::You => EnumSet::only(controller),
        PlayerSet::Opponents => player_queries::all_opponents(game, controller),
    }
}
