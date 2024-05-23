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

use std::sync::{Mutex, MutexGuard};

use async_trait::async_trait;
use data::core::primitives::{GameId, UserId};
use data::game_states::game_state::GameState;
use data::users::user_state::UserState;
use rusqlite::{Connection, OptionalExtension};
use serde_json::{de, ser};
use utils::outcome;
use utils::outcome::{Outcome, Value};
use utils::with_error::WithError;

use crate::database::Database;

pub struct SqliteDatabase {
    connection: Mutex<Connection>,
}

impl SqliteDatabase {
    pub fn new(connection: Connection) -> Value<Self> {
        connection
            .pragma_update(None, "foreign_keys", true)
            .with_error(|| "Error setting foreign keys pragma")?;
        connection
            .execute(
                "CREATE TABLE IF NOT EXISTS games (
                   id    BLOB PRIMARY KEY,
                   data  BLOB
                ) STRICT;",
                (),
            )
            .with_error(|| "Error creating table")?;
        connection
            .execute(
                "CREATE TABLE IF NOT EXISTS users (
                   id    BLOB PRIMARY KEY,
                   data  BLOB
                ) STRICT;",
                (),
            )
            .with_error(|| "Error creating table")?;

        Ok(Self { connection: Mutex::new(connection) })
    }

    fn db(&self) -> MutexGuard<Connection> {
        self.connection.lock().expect("Error locking connection")
    }
}

#[async_trait]
impl Database for SqliteDatabase {
    async fn fetch_game(&self, id: GameId) -> Value<Option<GameState>> {
        let data = self
            .db()
            .query_row("SELECT data FROM games WHERE id = ?1", [&id.0], |row| {
                let data: Vec<u8> = row.get(0)?;
                Ok(data)
            })
            .optional()
            .with_error(|| format!("Error fetching game {id:?}"))?;

        data.map(|data| {
            de::from_slice::<GameState>(&data)
                .with_error(|| format!("Error deserializing game {id:?}"))
        })
        .transpose()
    }

    async fn write_game(&self, game: &GameState) -> Outcome {
        let data =
            ser::to_vec(game).with_error(|| format!("Error serializing game {:?}", game.id))?;
        self.db()
            .execute(
                "INSERT INTO games (id, data)
                 VALUES (?1, ?2)
                 ON CONFLICT(id) DO UPDATE SET data = ?2",
                (&game.id.0, &data),
            )
            .with_error(|| format!("Error writing game to sqlite {:?}", game.id))?;
        outcome::OK
    }

    async fn fetch_user(&self, id: UserId) -> Value<Option<UserState>> {
        let data = self
            .db()
            .query_row("SELECT data FROM users WHERE id = ?1", [&id.0], |row| {
                let data: Vec<u8> = row.get(0)?;
                Ok(data)
            })
            .optional()
            .with_error(|| format!("Error fetching user {id:?}"))?;

        data.map(|data| {
            de::from_slice::<UserState>(&data)
                .with_error(|| format!("Error deserializing user {id:?}"))
        })
        .transpose()
    }

    async fn write_user(&self, user: &UserState) -> Outcome {
        let data =
            ser::to_vec(user).with_error(|| format!("Error serializing user {:?}", user.id))?;
        self.db()
            .execute(
                "INSERT INTO users (id, data)
                VALUES (?1, ?2)
                ON CONFLICT(id) DO UPDATE SET data = ?2",
                (&user.id.0, &data),
            )
            .with_error(|| format!("Error writing user to sqlite {:?}", user.id))?;
        outcome::OK
    }
}
