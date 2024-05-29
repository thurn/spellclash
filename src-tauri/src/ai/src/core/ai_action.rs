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

use data::actions::game_action::GameAction;
use data::core::primitives::PlayerName;
use data::game_states::game_state::GameState;
use rules::legality::legal_actions;
use tracing::{subscriber, Level};
use utils::outcome::Value;
use utils::verify;

use crate::core::agent::AgentConfig;
use crate::game::agents;
use crate::game::agents::AgentName;

/// Select a game action for the [PlayerName] in the given [GameState].
pub fn select(game: &GameState, player: PlayerName) -> Value<GameAction> {
    verify!(legal_actions::next_to_act(game) == player, "Not {:?}'s turn", player);
    let legal = legal_actions::compute(game, player);
    verify!(!legal.is_empty(), "No legal actions available");
    if legal.len() == 1 {
        return Ok(legal[0]);
    }

    let agent = agents::get_agent(AgentName::Uct1);
    let info_subscriber = tracing_subscriber::fmt().with_max_level(Level::INFO).finish();
    subscriber::with_default(info_subscriber, || {
        Ok(agent.pick_action(
            AgentConfig {
                deadline: Instant::now() + Duration::from_secs(10),
                panic_on_search_timeout: true,
            },
            game,
        ))
    })
}
