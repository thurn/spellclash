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
use database::sled_database::SledDatabase;
use dioxus::prelude::{Navigator, Signal, Writable};
use display::commands::command::Command;
use display::commands::scene_name::SceneName;
use display::core::game_view::GameView;
use game::server;
use game::server_data::{ClientData, GameResponse};
use once_cell::sync::Lazy;
use tracing::{debug, error, info_span};

use crate::game_client::Route;
use crate::initialize;

static DATABASE: Lazy<Arc<SledDatabase>> =
    Lazy::new(|| Arc::new(SledDatabase::new(initialize::get_data_dir())));

pub async fn connect(
    cd_signal: Signal<ClientData>,
    view_signal: Signal<Option<GameView>>,
    nav: Navigator,
) {
    let _span = info_span!("client_action::connect");
    let client_data = cd_signal();
    debug!("Connecting");
    let result = server::connect(DATABASE.clone(), client_data.user_id).await;
    match &result {
        Ok(response) => {
            debug!("Got connection response");
            handle_commands(response.clone(), nav, view_signal, cd_signal);
        }
        Err(err) => {
            error!("Error on connect: {:?}", err);
        }
    }
}

pub async fn apply(
    cd_signal: Signal<ClientData>,
    view_signal: Signal<Option<GameView>>,
    nav: Navigator,
    action: impl Into<UserAction>,
) {
    let user_action = action.into();
    let _span = info_span!("client_action::apply", ?user_action);
    let client_data = cd_signal();
    debug!("Applying action");
    let mut receiver = server::handle_action(DATABASE.clone(), client_data, user_action).await;
    loop {
        let result = receiver.recv().await;
        match &result {
            Some(Ok(response)) => {
                debug!("Got action response");
                handle_commands(response.clone(), nav, view_signal, cd_signal);
            }
            Some(Err(err)) => {
                error!("Error on action: {:?}, {:?}", err, user_action);
            }
            None => {
                debug!("Done receiving messages");
                break;
            }
        }
    }
}

fn handle_commands(
    response: GameResponse,
    navigator: Navigator,
    mut view_signal: Signal<Option<GameView>>,
    mut cd_signal: Signal<ClientData>,
) {
    *cd_signal.write() = response.client_data.clone();
    for command in response.commands {
        match command {
            Command::LoadScene { name, .. } => match name {
                SceneName::MainMenu => {
                    navigator.replace(Route::MainMenu {});
                }
                SceneName::Game(id) => {
                    navigator.replace(Route::Game { id });
                }
            },
            Command::UpdateGameView { view, .. } => {
                *view_signal.write() = Some(view);
            }
        }
    }
}
