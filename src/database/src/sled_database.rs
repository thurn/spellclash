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
use serde_json::{de, ser};
use sled::{Db, Tree};
use utils::outcome::Value;
use utils::with_error::WithError;

use crate::database::Database;

pub struct SledDatabase {
    db: Db,
}

impl SledDatabase {
    pub fn new(path: impl Into<String>) -> Self {
        Self { db: sled::open(path.into()).expect("Unable to open database") }
    }

    fn games(&self) -> Value<Tree> {
        self.db.open_tree("games").with_error(|| "Error opening the 'games' tree")
    }

    fn users(&self) -> Value<Tree> {
        self.db.open_tree("users").with_error(|| "Error opening the 'users' tree")
    }
}

#[async_trait]
impl Database for SledDatabase {
    async fn fetch_game(&self, id: GameId) -> Value<Option<GameState>> {
        self.games()?
            .get(game_id_key(id))
            .with_error(|| format!("Error fetching game {id:?}"))?
            .map(|slice| {
                de::from_slice::<GameState>(&slice)
                    .with_error(|| format!("Error deserializing game {id:?}"))
            })
            .transpose()
    }

    async fn write_game(&self, game: &GameState) -> Value<()> {
        self.games()?
            .insert(
                game_id_key(game.id),
                ser::to_vec(game).with_error(|| format!("Error serializing game {:?}", game.id))?,
            )
            .with_error(|| format!("Error inserting value for game {:?}", game.id))?;
        self.db.flush().with_error(|| "Error flushing db")?;
        Ok(())
    }

    async fn fetch_user(&self, id: UserId) -> Value<Option<UserState>> {
        self.users()?
            .get(user_id_key(id))
            .with_error(|| format!("Error fetching user {id:?}"))?
            .map(|slice| {
                de::from_slice::<UserState>(&slice)
                    .with_error(|| format!("Error deserializing user {id:?}"))
            })
            .transpose()
    }

    async fn write_user(&self, user: &UserState) -> Value<()> {
        self.users()?
            .insert(
                user_id_key(user.id),
                ser::to_vec(user).with_error(|| format!("Error serializing user {:?}", user.id))?,
            )
            .with_error(|| format!("Error inserting value for user {:?}", user.id))?;
        self.db.flush().with_error(|| "Error flushing db")?;
        Ok(())
    }
}

fn game_id_key(game_id: GameId) -> [u8; 16] {
    game_id.0.as_u128().to_be_bytes()
}

fn user_id_key(user_id: UserId) -> [u8; 16] {
    user_id.0.as_u128().to_be_bytes()
}
