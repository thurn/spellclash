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

use data::actions::game_action::GameAction;
use data::core::primitives::{GameId, PlayerName};
use data::users::user_state::UserState;
use database::database::Database;
use display::commands::display_preferences::DisplayPreferences;
use display::commands::scene_name::SceneName;
use display::rendering::render;
use tracing::info;
use utils::outcome::Value;
use utils::with_error::WithError;

use crate::requests;
use crate::server_data::{ClientData, GameResponse};

/// Connects to an ongoing game scene, returning a [GameResponse] which renders
/// its current visual state.
pub async fn connect(
    database: &impl Database,
    user: &UserState,
    game_id: GameId,
) -> Value<GameResponse> {
    let game = requests::fetch_game(database, game_id).await?;
    let player_name = game.find_player_name(user.id)?;
    let mut opponent_ids = vec![];
    for name in enum_iterator::all::<PlayerName>() {
        match game.players.get(name).user_id {
            Some(id) if id != user.id => {
                opponent_ids.push(id);
            }
            _ => {}
        }
    }

    info!(?user.id, ?game.id, "Connected to game");
    let mut commands = vec![requests::load_scene(SceneName::Game(game_id))];
    commands.append(&mut render::connect(&game, player_name, DisplayPreferences::default()));
    let client_data = ClientData {
        user_id: user.id,
        game_id: Some(game.id),
        display_preferences: DisplayPreferences::default(),
        opponent_ids,
    };
    Ok(GameResponse::new(client_data).commands(commands))
}

pub async fn handle_game_action(
    database: &impl Database,
    data: ClientData,
    _action: GameAction,
) -> Value<GameResponse> {
    let game =
        requests::fetch_game(database, data.game_id.with_error(|| "Expected current game ID")?)
            .await?;
    let player_name = game.find_player_name(data.user_id)?;
    let user_result = render::render_updates(&game, player_name, data.display_preferences);

    let mut opponent_responses = vec![];
    for &opponent_id in &data.opponent_ids {
        let opponent_name = game.find_player_name(opponent_id)?;
        opponent_responses.push((
            opponent_id,
            render::render_updates(&game, opponent_name, DisplayPreferences::default()),
        ));
    }

    database.write_game(&game).await?;
    Ok(GameResponse::new(data).commands(user_result).opponent_responses(opponent_responses))
}
