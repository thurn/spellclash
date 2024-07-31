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

use std::path::PathBuf;
use std::sync::{Arc, Mutex, MutexGuard};

use data::game_states::serialized_game_state::SerializedGameState;
use data::printed_cards::database_card::DatabaseCardFace;
use data::printed_cards::printed_card_id::PrintedCardId;
use data::users::user_state::UserState;
use primitives::game_primitives::{GameId, UserId};
use rusqlite::{Connection, Error, OptionalExtension};
use serde_json::{de, ser};

/// SQLite database connection.
///
/// This struct is used to fetch data from & mutate the database. It operates as
/// a smart pointer, so calling .clone() is inexpensive and this is the expected
/// way to pass the connection between callers.
#[derive(Clone, Debug)]
pub struct SqliteDatabase {
    connection: Arc<Mutex<Connection>>,
}

impl SqliteDatabase {
    pub fn new(directory: PathBuf) -> Self {
        let connection = match Connection::open(directory.join("game.sqlite")) {
            Ok(connection) => connection,
            Err(Error::SqliteFailure(_, s)) => {
                panic!("Error opening database connection: {:?}", s);
            }
            Err(err) => {
                panic!("Error opening database connection: {:?}", err);
            }
        };

        connection
            .pragma_update(None, "foreign_keys", true)
            .expect("Error setting foreign keys pragma");
        let attach_printings = format!(
            "ATTACH '{}' as oracle;",
            directory.join("AllPrintings.sqlite").to_str().unwrap()
        );
        connection.execute(&attach_printings, ()).expect("Error attaching table");
        connection
            .execute(
                "CREATE TABLE IF NOT EXISTS games (
                   id    BLOB PRIMARY KEY,
                   data  BLOB
                ) STRICT;",
                (),
            )
            .expect("Error creating table");
        connection
            .execute(
                "CREATE TABLE IF NOT EXISTS users (
                   id    BLOB PRIMARY KEY,
                   data  BLOB
                ) STRICT;",
                (),
            )
            .expect("Error creating table");

        Self { connection: Arc::new(Mutex::new(connection)) }
    }

    pub fn fetch_game(&self, id: GameId) -> Option<SerializedGameState> {
        let data = self
            .db()
            .query_row("SELECT data FROM games WHERE id = ?1", [&id.0], |row| {
                let data: Vec<u8> = row.get(0)?;
                Ok(data)
            })
            .optional()
            .unwrap_or_else(|e| panic!("Error fetching game {id:?} {e:?}"));

        data.map(|data| {
            de::from_slice::<SerializedGameState>(&data)
                .unwrap_or_else(|e| panic!("Error deserializing game {id:?} {e:?}"))
        })
    }

    pub fn write_game(&self, game: &SerializedGameState) {
        let data = ser::to_vec(game)
            .unwrap_or_else(|e| panic!("Error serializing game {:?} {e:?}", game.id));
        self.db()
            .execute(
                "INSERT INTO games (id, data)
                 VALUES (?1, ?2)
                 ON CONFLICT(id) DO UPDATE SET data = ?2",
                (&game.id.0, &data),
            )
            .unwrap_or_else(|e| panic!("Error writing game to sqlite {:?} {e:?}", game.id));
    }

    pub fn fetch_user(&self, id: UserId) -> Option<UserState> {
        let data = self
            .db()
            .query_row("SELECT data FROM users WHERE id = ?1", [&id.0], |row| {
                let data: Vec<u8> = row.get(0)?;
                Ok(data)
            })
            .optional()
            .unwrap_or_else(|e| panic!("Error fetching user {id:?} {e:?}"));

        data.map(|data| {
            de::from_slice::<UserState>(&data)
                .unwrap_or_else(|e| panic!("Error deserializing user {id:?} {e:?}"))
        })
    }

    pub fn write_user(&self, user: &UserState) {
        let data = ser::to_vec(user)
            .unwrap_or_else(|e| panic!("Error serializing user {:?} {e:?}", user.id));
        self.db()
            .execute(
                "INSERT INTO users (id, data)
                VALUES (?1, ?2)
                ON CONFLICT(id) DO UPDATE SET data = ?2",
                (&user.id.0, &data),
            )
            .unwrap_or_else(|e| panic!("Error writing user to sqlite {:?} {e:?}", user.id));
    }

    /// Fetch the [DatabaseCardFace]s of a given [PrintedCardId].
    pub fn fetch_printed_faces(&self, id: PrintedCardId) -> Vec<DatabaseCardFace> {
        let connection = self.db();
        let mut statement = connection
            .prepare(
                "SELECT *
                 FROM oracle.cards NATURAL JOIN oracle.cardIdentifiers
                 WHERE scryfallId = ?1",
            )
            .expect("Error preparing query");

        // Note: database stores UUIDs as literal strings, not blobs.
        let rows = statement.query([id.0.to_string()]).expect("Error querying database");
        let cards = serde_rusqlite::from_rows::<DatabaseCardFace>(rows);
        cards.collect::<Result<_, _>>().expect("Error fetching card")
    }

    fn db(&self) -> MutexGuard<Connection> {
        match self.connection.lock() {
            Ok(guard) => guard,
            Err(er) => {
                panic!("Error getting database lock, did a writer panic? {:?}", er);
            }
        }
    }
}
