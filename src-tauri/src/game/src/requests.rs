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

use std::sync::Arc;

use data::core::primitives::{GameId, UserId};
use data::game_states::animation_tracker::{AnimationState, AnimationTracker};
use data::game_states::game_state::GameState;
use data::users::user_state::UserState;
use database::sqlite_database::SqliteDatabase;
use oracle::card_database;
use utils::outcome::{Outcome, Value};
use utils::with_error::WithError;

/// Looks up a user by ID in the database.
pub async fn fetch_user(database: Arc<SqliteDatabase>, user_id: UserId) -> Value<UserState> {
    database.fetch_user(user_id).await?.with_error(|| format!("User not found {user_id:?}"))
}

/// Looks up a game by ID in the database.
pub async fn fetch_game(database: Arc<SqliteDatabase>, game_id: GameId) -> Value<GameState> {
    let mut game =
        database.fetch_game(game_id).await?.with_error(|| format!("Game not found {game_id:?}"))?;
    initialize_game(database, &mut game)?;
    Ok(game)
}

pub fn initialize_game(database: Arc<SqliteDatabase>, game: &mut GameState) -> Outcome {
    for previous in game.undo_tracker.undo.iter_mut() {
        initialize_game(database.clone(), previous.as_mut())?;
    }

    game.animations = AnimationTracker::new(if game.configuration.simulation {
        AnimationState::Ignore
    } else {
        AnimationState::Track
    });
    card_database::populate(database, game)
}
