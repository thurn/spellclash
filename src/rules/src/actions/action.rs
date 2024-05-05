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

use data::actions::game_action::GameAction;
use data::core::primitives::PlayerName;
use data::game_states::game_state::GameState;
use tracing::{debug, instrument};
use utils::outcome::Outcome;
use utils::{outcome, verify};

use crate::queries::{legal_actions, players};
use crate::steps::step;

#[instrument(err, level = "debug", skip(game))]
pub fn execute(game: &mut GameState, player: PlayerName, action: GameAction) -> Outcome {
    match action {
        GameAction::PassPriority => handle_pass_priority(game, player),
    }
}

#[instrument(err, level = "debug", skip(game))]
fn handle_pass_priority(game: &mut GameState, player: PlayerName) -> Outcome {
    debug!(?player, "Passing priority");
    verify!(legal_actions::can_pass_priority(game, player), "Cannot pass priority for {player:?}");
    game.passed.insert(player);
    if game.passed.len() == game.configuration.all_players.len() {
        step::advance(game)
    } else {
        game.priority = players::next_player_after(game, game.priority);
        outcome::OK
    }
}
