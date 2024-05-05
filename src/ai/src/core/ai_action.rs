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

use std::time::Duration;

use data::actions::game_action::GameAction;
use data::core::primitives::PlayerName;
use data::game_states::game_state::GameState;
use rand::prelude::IteratorRandom;
use rules::queries::legal_actions;
use tokio::time;
use utils::outcome::Value;
use utils::with_error::WithError;

/// Select a game action for the [PlayerName] in the given [GameState].
pub async fn select(game: &GameState, player: PlayerName) -> Value<GameAction> {
    time::sleep(Duration::from_millis(300)).await;
    legal_actions::compute(game, player)
        .choose(&mut rand::thread_rng())
        .with_error(|| "No legal actions available")
}
