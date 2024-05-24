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

use data::actions::game_action::GameAction;
use data::core::panel_address::PanelAddress;
use database::sqlite_database::SqliteDatabase;
use display::commands::scene_identifier::SceneIdentifier;
use display::panels::panel;
use tracing::instrument;
use utils::outcome::Value;
use utils::with_error::WithError;

use crate::requests;
use crate::server_data::{ClientData, GameResponse};

#[instrument(level = "debug", skip(database))]
pub async fn handle_open_panel(
    database: Arc<SqliteDatabase>,
    mut data: ClientData,
    panel: PanelAddress,
) -> Value<GameResponse> {
    match panel {
        PanelAddress::GamePanel(game_panel) => {
            let game_id = data.game_id().with_error(|| "Expected current game ID")?;
            let game = requests::fetch_game(database, game_id).await?;
            let player_name = game.find_player_name(data.user_id)?;
            let panel = panel::build_game_panel(&game, player_name, game_panel);
            data.modal_panel = Some(panel);
            Ok(GameResponse::new(data))
        }
        PanelAddress::UserPanel(player_panel) => match player_panel {},
    }
}

pub fn handle_close_panel(mut data: ClientData) -> Value<GameResponse> {
    data.modal_panel = None;
    Ok(GameResponse::new(data))
}
