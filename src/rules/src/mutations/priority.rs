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
use data::core::primitives::PlayerName;
use data::game_states::game_state::GameState;
use tracing::{debug, instrument};
use utils::outcome::Outcome;
use utils::{outcome, verify};

use crate::queries::players;
use crate::resolve_cards::resolve;
use crate::steps::step;

/// Passes priority for the [PlayerName] player.
///
/// Returns an error if this player does not have priority.
#[instrument(err, level = "debug", skip(game))]
pub fn pass(game: &mut GameState, player: PlayerName) -> Outcome {
    verify!(game.priority == player, "Player {player:?} does not have priority");
    debug!(?player, "Passing priority");
    game.passed.insert(player);
    if game.passed.len() == game.configuration.all_players.len() {
        game.clear_passed();
        if game.stack().is_empty() {
            step::advance(game)
        } else {
            resolve::resolve_top_of_stack(game)
        }
    } else {
        game.priority = players::next_player_after(game, game.priority);
        outcome::OK
    }
}
