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

use data::users::user_state::UserActivity;
use database::sqlite_database::SqliteDatabase;
use display::commands::command::{Command, SceneView};
use display::commands::scene_identifier::SceneIdentifier;
use tokio::sync::mpsc::UnboundedSender;

use crate::server_data::{Client, ClientData, GameResponse};
use crate::{main_menu_server, requests};

pub fn leave(database: SqliteDatabase, client: &mut Client) {
    let id = client.data.user_id;
    let mut user = requests::fetch_user(database.clone(), id);
    user.activity = UserActivity::Menu;
    database.write_user(&user);
    client.data.scene = SceneIdentifier::MainMenu;
    client.send(Command::UpdateScene(SceneView::MainMenuView(main_menu_server::main_menu_view())));
}
