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
use color_eyre::eyre::Context;
use color_eyre::Result;
use data::core::primitives::GameId;
use data::game_states::game_state::GameState;
use serde_json::{de, ser};
use sled::{Db, Tree};

use crate::database::Database;

pub struct SledDatabase {
    db: Db,
}

impl SledDatabase {
    pub fn new(path: impl Into<String>) -> Self {
        Self { db: sled::open(path.into()).expect("Unable to open database") }
    }

    fn games(&self) -> Result<Tree> {
        self.db.open_tree("games").with_context(|| "Error opening the 'games' tree")
    }
}

#[async_trait]
impl Database for SledDatabase {
    async fn fetch_game(&self, id: GameId) -> Result<Option<GameState>> {
        self.games()?
            .get(game_id_key(id))
            .with_context(|| format!("Error fetching game {id:?}"))?
            .map(|slice| {
                de::from_slice::<GameState>(&slice)
                    .with_context(|| format!("Error deserializing game {id:?}"))
            })
            .transpose()
    }

    async fn write_game(&self, game: &GameState) -> Result<()> {
        self.games()?.insert(
            game_id_key(game.id),
            ser::to_vec(game).with_context(|| format!("Error serializing game {:?}", game.id))?,
        )?;
        self.db.flush()?;
        Ok(())
    }
}

fn game_id_key(game_id: GameId) -> [u8; 16] {
    game_id.0.as_u128().to_be_bytes()
}
