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
use data::core::primitives::{GameId, PlayerName};
use data::users::user_state::UserState;
use database::database::Database;
use display::commands::scene_name::SceneName;
use display::rendering::render;
use tracing::info;

use crate::requests;
use crate::server_data::{ClientData, GameResponse};

/// Connects to an ongoing game scene, returning a [GameResponse] which renders
/// its current visual state.
pub async fn connect(
    database: &impl Database,
    user: &UserState,
    game_id: GameId,
    player_name: PlayerName,
) -> Result<GameResponse> {
    let game = requests::fetch_game(database, game_id).await?;
    info!(?user.id, ?game.id, "Connected to game");
    let mut commands = vec![requests::load_scene(SceneName::Game)];
    commands.append(&mut render::connect(&game, player_name));
    let client_data = ClientData { user_id: user.id, game_id: Some(game.id) };
    Ok(GameResponse::new(client_data).commands(commands))
}
