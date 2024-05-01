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

use color_eyre::Result;
use data::users::user_state::UserState;
use database::database::Database;
use display::commands::scene_name::SceneName;
use tracing::info;

use crate::requests;
use crate::server_data::{ClientData, GameResponse};

/// Connect to the main menu scene
pub async fn connect(_: &impl Database, user: &UserState) -> Result<GameResponse> {
    info!(?user.id, "Connected");
    let commands = vec![requests::load_scene(SceneName::MainMenu)];
    let client_data = ClientData { user_id: user.id, game_id: None };
    Ok(GameResponse::new(client_data).commands(commands))
}
