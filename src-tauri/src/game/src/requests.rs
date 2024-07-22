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
use data::game_states::game_state::GameState;
use data::prompts::game_update::UpdateChannel;
use data::users::user_state::UserState;
use database::sqlite_database::SqliteDatabase;
use oracle::card_database;

use crate::game_creation::{game_serialization, initialize_game};

/// Looks up a user by ID in the database.
pub fn fetch_user(database: SqliteDatabase, user_id: UserId) -> UserState {
    database.fetch_user(user_id).unwrap_or_else(|| {
        panic!("User not found: {user_id:?}");
    })
}

/// Looks up a game by ID in the database.
///
/// An [UpdateChannel] can be provided here in order to receive incremental
/// updates and interface prompts. This should generally be included whenever
/// you are fetching a game for the purpose of mutating it, as otherwise
/// attempts to prompt the user for a choice will panic. It's safe to omit this
/// field when simply reading the state of a game.
pub fn fetch_game(
    database: SqliteDatabase,
    game_id: GameId,
    update_channel: Option<UpdateChannel>,
) -> GameState {
    let serialized =
        database.fetch_game(game_id).unwrap_or_else(|| panic!("Game not found: {game_id:?}"));
    game_serialization::rebuild(database.clone(), serialized)
}
