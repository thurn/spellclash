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
use tokio::sync::mpsc;
use tokio::sync::mpsc::Receiver;
use tracing::{debug_span, error, info};
use utils::outcome::Value;

use crate::server_data::{ClientData, GameResponse};
use crate::{game_action_server, leave_game_server, main_menu_server, new_game_server};

/// Connects to the current game scene.
///
/// This returns commands to load & render the current game state. It's expected
/// that this will be invoked on application start and on scene change.
pub async fn connect(database: Arc<dyn Database>, user_id: UserId) -> Value<GameResponse> {
    let user = fetch_or_create_user(database.clone(), user_id).await?;
    let result = match user.activity {
        UserActivity::Menu => main_menu_server::connect(database, &user).await,
        UserActivity::Playing(game_id) => {
            game_action_server::connect(database, &user, game_id).await
        }
    }?;

    Ok(result)
}

/// Handles a [UserAction] from the client.
///
/// The most recently-returned [ClientData] (from a call to this function or
/// [connect]) must be provided to this call. Returns a [Receiver] which will be
/// sent the results of incremental updates to the game (e.g. results of AI
/// actions) along with any errors which occur.
pub async fn handle_action(
    database: Arc<dyn Database>,
    data: ClientData,
    action: UserAction,
) -> Receiver<Value<GameResponse>> {
    let (sender, receiver) = mpsc::channel(4);

    tokio::spawn(async move {
        let _span = debug_span!("handle_action", ?data.user_id, ?data.game_id);
        let result = match action {
            UserAction::NewGameAction(action) => {
                new_game_server::create(sender.clone(), database, data, action).await
            }
            UserAction::GameAction(action) => {
                game_action_server::handle_game_action(sender.clone(), database, data, action).await
            }
            UserAction::LeaveGameAction => {
                leave_game_server::leave(sender.clone(), database, data).await
            }
        };

        if let Err(e) = result {
            // Error indicates the receiver has been closed or dropped, generally safe to
            // ignore.
            error!("Error sending game action response: {:?}", e);
        }
    });

    receiver
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
