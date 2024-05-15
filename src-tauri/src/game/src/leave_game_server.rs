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
use database::database::Database;
use display::commands::scene_name::SceneName;
use utils::outcome::Value;

use crate::requests;
use crate::server_data::{ClientData, GameResponse};

pub async fn leave(database: Arc<dyn Database>, mut data: ClientData) -> Value<GameResponse> {
    let mut user = requests::fetch_user(database.clone(), data.user_id).await?;
    user.activity = UserActivity::Menu;
    database.write_user(&user).await?;
    data.game_id = None;
    Ok(GameResponse::new(SceneName::MainMenu, data))
}
