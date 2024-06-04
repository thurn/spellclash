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
use data::game_states::animation_tracker::AnimationState;
use data::game_states::game_state::GameState;
use rules::legality::legal_actions;
use rules::legality::legal_actions::LegalActions;
use tracing::{instrument, subscriber, Level};
use utils::command_line::TracingStyle;
use utils::outcome::Value;
use utils::{command_line, verify};

use crate::core::agent::AgentConfig;
use crate::game::agents;
use crate::game::agents::AgentName;

/// Select a game action for the [PlayerName] in the given [GameState].
#[instrument(level = "debug", skip_all)]
pub fn select(input_game: &GameState, player: PlayerName) -> Value<GameAction> {
    verify!(legal_actions::next_to_act(input_game) == player, "Not {:?}'s turn", player);
    let legal = legal_actions::compute(input_game, player, LegalActions {
        include_interface_actions: false,
    });
    verify!(!legal.is_empty(), "No legal actions available");
    if legal.len() == 1 {
        return Ok(legal[0]);
    }

    let mut game = input_game.clone();
    game.undo_tracker.enabled = false;
    game.undo_tracker.undo.clear();
    game.animations.state = AnimationState::Ignore;
    game.animations.steps.clear();

    let agent = agents::get_agent(AgentName::Uct1Iterations10_000);
    let deadline = Duration::from_secs(100);
    match command_line::flags().tracing_style {
        TracingStyle::AggregateTime | TracingStyle::None => {
            Ok(agent.pick_action(AgentConfig { deadline: Instant::now() + deadline }, &game))
        }
        TracingStyle::Forest => {
            let info_subscriber = tracing_subscriber::fmt().with_max_level(Level::INFO).finish();
            subscriber::with_default(info_subscriber, || {
                Ok(agent.pick_action(AgentConfig { deadline: Instant::now() + deadline }, &game))
            })
        }
    }
}
