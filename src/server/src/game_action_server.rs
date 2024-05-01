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

use color_eyre::eyre::bail;
use color_eyre::Result;
use data::actions::game_action::GameAction;
use data::core::primitives::{GameId, PlayerName, UserId};
use data::game_states::game_state::GameState;
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
) -> Result<GameResponse> {
    let game = requests::fetch_game(database, game_id).await?;
    let (player_name, opponent_ids) = find_players(&game, user.id)?;
    info!(?user.id, ?game.id, "Connected to game");
    let mut commands = vec![requests::load_scene(SceneName::Game)];
    commands.append(&mut render::connect(&game, player_name));
    let client_data = ClientData { user_id: user.id, game_id: Some(game.id), opponent_ids };
    Ok(GameResponse::new(client_data).commands(commands))
}

pub async fn handle_game_action(
    _database: &impl Database,
    _data: ClientData,
    _action: GameAction,
) -> Result<GameResponse> {
    todo!("")
}

fn find_players(game: &GameState, user_id: UserId) -> Result<(PlayerName, Vec<UserId>)> {
    let mut opponent_ids = vec![];
    for name in enum_iterator::all::<PlayerName>() {
        match game.players.get(name).user_id {
            Some(id) if id != user_id => {
                opponent_ids.push(id);
            }
            _ => {}
        }
    }

    for name in enum_iterator::all::<PlayerName>() {
        if game.players.get(name).user_id == Some(user_id) {
            return Ok((name, opponent_ids));
        }
    }

    bail!("User {user_id:?} is not a player in game {:?}", game.id);
}
