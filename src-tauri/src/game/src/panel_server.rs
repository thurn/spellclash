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
use data::actions::user_action::PanelTransition;
use data::core::panel_address::PanelAddress;
use database::sqlite_database::SqliteDatabase;
use display::commands::command::Command;
use display::commands::scene_identifier::SceneIdentifier;
use display::panels::modal_panel::ModalPanel;
use display::panels::panel;
use tokio::sync::mpsc::UnboundedSender;
use tracing::instrument;

use crate::server_data::{Client, ClientData, GameResponse};
use crate::{game_action_server, requests};

#[instrument(level = "debug", skip_all)]
pub fn handle_open_panel(database: SqliteDatabase, client: &mut Client, panel: PanelAddress) {
    client.send(Command::SetModalPanel(Some(open_panel(database, &client.data, panel))))
}

#[instrument(level = "debug", skip_all)]
pub fn handle_close_panel(client: &mut Client) {
    client.send(Command::SetModalPanel(None))
}

#[instrument(level = "debug", skip_all)]
pub async fn handle_panel_transition(
    database: SqliteDatabase,
    client: &mut Client,
    transition: PanelTransition,
) {
    client.send(Command::SetModalPanel(None));
    if let Some(action) = transition.action {
        game_action_server::handle_game_action(database.clone(), client, action).await;
    }

    if let Some(next_panel) = transition.open {
        client.send(Command::SetModalPanel(Some(open_panel(database, &client.data, next_panel))))
    }
}

fn open_panel(database: SqliteDatabase, data: &ClientData, panel: PanelAddress) -> ModalPanel {
    match panel {
        PanelAddress::GamePanel(game_panel) => {
            let game_id = data.game_id();
            let game = requests::fetch_game(database, game_id);
            let player_name = game.find_player_name(data.user_id);
            panel::build_game_panel(&game, player_name, game_panel)
        }
        PanelAddress::UserPanel(player_panel) => match player_panel {},
    }
}
