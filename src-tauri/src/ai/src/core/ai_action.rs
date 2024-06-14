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

use ai_core::agent::AgentConfig;
use data::actions::game_action::GameAction;
use data::core::primitives::PlayerName;
use data::game_states::game_state::GameState;
use rules::legality::legal_actions;
use rules::legality::legal_actions::LegalActions;
use tracing::{instrument, subscriber, Level};
use utils::command_line;
use utils::command_line::TracingStyle;

use crate::game::agents;
use crate::game::agents::AgentName;
use crate::game::ai_definitions::AgentGameState;

/// Select a game action for the [PlayerName] in the given [GameState].
#[instrument(level = "debug", skip_all)]
pub fn select(input_game: &GameState, player: PlayerName) -> GameAction {
    assert_eq!(legal_actions::next_to_act(input_game, None), player, "Not {:?}'s turn", player);
    let legal =
        legal_actions::compute(input_game, player, LegalActions { for_human_player: false });
    assert!(!legal.is_empty(), "No legal actions available");
    if legal.len() == 1 {
        return legal[0];
    }

    let game = AgentGameState(input_game.shallow_clone());
    let agent = agents::get_agent(AgentName::Uct1Iterations10_000);
    let deadline = Duration::from_secs(100);
    match command_line::flags().tracing_style {
        TracingStyle::AggregateTime | TracingStyle::None => {
            agent.pick_action(AgentConfig { deadline: Instant::now() + deadline }, &game)
        }
        TracingStyle::Forest => {
            let info_subscriber = tracing_subscriber::fmt().with_max_level(Level::INFO).finish();
            subscriber::with_default(info_subscriber, || {
                agent.pick_action(AgentConfig { deadline: Instant::now() + deadline }, &game)
            })
        }
    }
}
