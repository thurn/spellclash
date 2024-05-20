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

use data::actions::user_action::UserAction;
use data::core::primitives::UserId;
use data::users::user_state::{UserActivity, UserState};
use database::database::Database;
use tracing::{debug_span, info, Instrument};
use utils::outcome::Value;

use crate::server_data::{ClientData, GameResponse};
use crate::{
    game_action_server, leave_game_server, main_menu_server, new_game_server, panel_server,
};

/// Connects to the current game scene.
///
/// This returns commands to load & render the current game state. It's expected
/// that this will be invoked on application start and on scene change.
pub async fn connect(database: Arc<dyn Database>, user_id: UserId) -> Value<GameResponse> {
    let user = fetch_or_create_user(database.clone(), user_id).await?;
    let span = debug_span!("connect", ?user_id);
    let result = match user.activity {
        UserActivity::Menu => main_menu_server::connect(database, &user).instrument(span).await,
        UserActivity::Playing(game_id) => {
            game_action_server::connect(database, &user, game_id).instrument(span).await
        }
    }?;

    Ok(result)
}

/// Handles a [UserAction] from the client and returns a [GameResponse].
///
/// The most recently-returned [ClientData] (from a call to this function or
/// [connect]) must be provided to this call.
pub async fn handle_action(
    database: Arc<dyn Database>,
    data: ClientData,
    action: UserAction,
) -> Value<GameResponse> {
    let span = debug_span!("handle_action", ?data.user_id, ?data.game_id);
    match action {
        UserAction::NewGameAction(action) => {
            new_game_server::create(database, data, action).instrument(span).await
        }
        UserAction::GameAction(action) => {
            game_action_server::handle_game_action(database, data, action).instrument(span).await
        }
        UserAction::LeaveGameAction => {
            leave_game_server::leave(database, data).instrument(span).await
        }
        UserAction::QuitGameAction => {
            std::process::exit(0);
        }
        UserAction::OpenPanel(panel_address) => {
            panel_server::handle_open_panel(database, data, panel_address).instrument(span).await
        }
        UserAction::ClosePanel => panel_server::handle_close_panel(data),
    }
}

async fn fetch_or_create_user(database: Arc<dyn Database>, user_id: UserId) -> Value<UserState> {
    Ok(if let Some(player) = database.fetch_user(user_id).await? {
        player
    } else {
        let user = UserState { id: user_id, activity: UserActivity::Menu };
        database.write_user(&user).await?;
        info!(?user_id, "Created new user");
        user
    })
}
