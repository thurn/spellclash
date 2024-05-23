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

use async_trait::async_trait;
use data::core::primitives::{GameId, UserId};
use data::game_states::game_state::GameState;
use data::users::user_state::UserState;
use utils::outcome::{Outcome, Value};

/// Trait abstracting over ways of serializing game state
#[async_trait]
pub trait Database: Send + Sync {
    /// Read a game from the database
    async fn fetch_game(&self, id: GameId) -> Value<Option<GameState>>;

    /// Write a game to the database
    async fn write_game(&self, game: &GameState) -> Outcome;

    /// Read a user from the database
    async fn fetch_user(&self, id: UserId) -> Value<Option<UserState>>;

    /// Write a user to the database
    async fn write_user(&self, user: &UserState) -> Outcome;
}
