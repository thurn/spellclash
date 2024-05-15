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

use ai::core::ai_action;
use data::actions::game_action::GameAction;
use data::core::primitives::{GameId, PlayerName};
use data::game_states::game_state::GameState;
use data::player_states::player_state::PlayerQueries;
use data::users::user_state::UserState;
use database::database::Database;
use display::commands::display_preferences::DisplayPreferences;
use display::commands::scene_name::SceneName;
use display::rendering::render;
use rules::action_handlers::actions;
use rules::action_handlers::actions::PlayerType;
use rules::legality::legal_actions;
use tracing::{debug, error, info, instrument};
use utils::outcome::Value;
use utils::with_error::WithError;

use crate::requests;
use crate::server_data::{ClientData, GameResponse};

/// Connects to an ongoing game scene, returning a [GameResponse] which renders
/// its current visual state.
#[instrument(level = "debug", skip(database))]
pub async fn connect(
    database: Arc<dyn Database>,
    user: &UserState,
    game_id: GameId,
) -> Value<GameResponse> {
    let game = requests::fetch_game(database, game_id).await?;
    let player_name = game.find_player_name(user.id)?;
    let mut opponent_ids = vec![];
    for name in enum_iterator::all::<PlayerName>() {
        match game.players.player(name).user_id {
            Some(id) if id != user.id => {
                opponent_ids.push(id);
            }
            _ => {}
        }
    }

    info!(?user.id, ?game.id, "Connected to game");
    let commands = render::connect(&game, player_name, DisplayPreferences::default());
    let client_data = ClientData {
        user_id: user.id,
        game_id: Some(game.id),
        display_preferences: DisplayPreferences::default(),
        opponent_ids,
    };
    Ok(GameResponse::new(SceneName::Game, client_data).commands(commands))
}

#[instrument(level = "debug", skip(database))]
pub async fn handle_game_action(
    database: Arc<dyn Database>,
    data: ClientData,
    action: GameAction,
) -> Value<GameResponse> {
    let mut game = requests::fetch_game(
        database.clone(),
        data.game_id.with_error(|| "Expected current game ID")?,
    )
    .await?;
    let result = handle_game_action_internal(database, &data, action, &mut game).await;
    if let Err(err) = &result {
        error!(?action, "Error running game action loop: {:?}", err);
    }
    result
}

async fn handle_game_action_internal(
    database: Arc<dyn Database>,
    data: &ClientData,
    action: GameAction,
    game: &mut GameState,
) -> Value<GameResponse> {
    let user_player_name = game.find_player_name(data.user_id)?;
    let mut current_player = user_player_name;
    let mut current_action = action;
    let mut result = GameResponse::new(SceneName::Game, data.clone());

    loop {
        actions::execute(
            game,
            current_player,
            current_action,
            if current_player == user_player_name { PlayerType::User } else { PlayerType::Agent },
        )?;
        let user_result = render::render_updates(game, user_player_name, data.display_preferences);

        let mut opponent_responses = vec![];
        for &opponent_id in &data.opponent_ids {
            let opponent_name = game.find_player_name(opponent_id)?;
            opponent_responses.push((
                opponent_id,
                render::render_updates(game, opponent_name, DisplayPreferences::default()),
            ))
        }
        result = result.commands(user_result);
        result = result.opponent_responses(opponent_responses);

        let next_player = legal_actions::next_to_act(game);
        if game.player(next_player).user_id.is_some() {
            database.write_game(game).await?;
            return Ok(result);
        } else {
            debug!(?next_player, "Searching for AI action");
            current_player = next_player;
            current_action = ai_action::select(game, next_player).await?;
            debug!(?next_player, ?current_action, "AI action selected");
        }
    }
}
