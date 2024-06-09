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

use std::sync::mpsc::Sender;
use std::sync::Arc;

use data::actions::user_action::UserAction;
use data::core::primitives::UserId;
use data::prompts::card_select_and_order_prompt::CardOrderLocation;
use data::users::user_state::{UserActivity, UserState};
use database::sqlite_database::SqliteDatabase;
use display::commands::field_state::{FieldKey, FieldValue};
use display::core::card_view::ClientCardId;
use tokio::sync::mpsc::UnboundedSender;
use tracing::{debug_span, info, Instrument};

use crate::server_data::{Client, ClientData, GameResponse};
use crate::{
    game_action_server, leave_game_server, main_menu_server, new_game_server, panel_server,
};

/// Connects to the current game scene.
///
/// This returns commands to load & render the current game state. It's expected
/// that this will be invoked on application start and on scene change.
pub fn connect(
    database: SqliteDatabase,
    response_channel: UnboundedSender<GameResponse>,
    user_id: UserId,
) {
    let user = fetch_or_create_user(database.clone(), user_id);
    let _span = debug_span!("connect", ?user_id);
    match user.activity {
        UserActivity::Menu => main_menu_server::connect(response_channel, &user),
        UserActivity::Playing(game_id) => {
            game_action_server::connect(database, response_channel, &user, game_id)
        }
    }
}

/// Handles a [UserAction] from the client. Sends incremental game state updates
/// to the provided `Sender` as [GameResponse] snapshots.
///
/// The most recently-returned [ClientData] (from a call to this function or
/// [connect]) must be provided to this call.
pub async fn handle_action(database: SqliteDatabase, client: &mut Client, action: UserAction) {
    let span = debug_span!("handle_action", ?action);
    match action {
        UserAction::NewGameAction(action) => new_game_server::create(database, client, action),
        UserAction::GameAction(action) => {
            game_action_server::handle_game_action(database, client, action).instrument(span).await;
        }
        UserAction::PromptAction(action) => {
            game_action_server::handle_prompt_action(client, action)
        }
        UserAction::LeaveGameAction => leave_game_server::leave(database, client),
        UserAction::QuitGameAction => {
            std::process::exit(0);
        }
        UserAction::OpenPanel(panel_address) => {
            panel_server::handle_open_panel(database, client, panel_address)
        }
        UserAction::ClosePanel => panel_server::handle_close_panel(client),
        UserAction::PanelTransition(transition) => {
            panel_server::handle_panel_transition(database, client, transition)
                .instrument(span)
                .await;
        }
    }
}

pub fn handle_update_field(
    database: SqliteDatabase,
    client: &mut Client,
    key: FieldKey,
    value: FieldValue,
) {
    game_action_server::handle_update_field(database, client, key, value);
}

pub fn handle_drag_card(
    database: SqliteDatabase,
    client: &mut Client,
    card_id: ClientCardId,
    location: CardOrderLocation,
    index: u32,
) {
    info!(?card_id, ?location, "handle_drag_card")
}

fn fetch_or_create_user(database: SqliteDatabase, user_id: UserId) -> UserState {
    if let Some(player) = database.fetch_user(user_id) {
        player
    } else {
        let user = UserState { id: user_id, activity: UserActivity::Menu };
        database.write_user(&user);
        info!(?user_id, "Created new user");
        user
    }
}
