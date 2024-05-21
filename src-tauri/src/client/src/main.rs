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

// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::env;
use std::sync::Arc;

use all_cards::card_list;
use clap::Parser;
use data::actions::user_action::UserAction;
use data::core::primitives::UserId;
use database::sled_database::SledDatabase;
use game::server;
use game::server_data::{ClientData, GameResponse};
use once_cell::sync::Lazy;
use tracing::{error, info};
use utils::outcome;
use utils::outcome::Outcome;
use utils::with_error::WithError;
use uuid::Uuid;

use crate::cli::{Cli, ARGS};

mod cli;
mod initialize;
mod logging;

static DATABASE: Lazy<Arc<SledDatabase>> =
    Lazy::new(|| Arc::new(SledDatabase::new(initialize::get_data_dir())));

#[tauri::command]
#[specta::specta]
async fn client_connect() -> Result<GameResponse, ()> {
    info!("Got connect request");
    let result = server::connect(DATABASE.clone(), UserId(Uuid::default())).await;
    match result {
        Ok(response) => Ok(response),
        Err(err) => {
            error!("Error on connect: {:?}", err);
            Err(())
        }
    }
}

#[tauri::command]
#[specta::specta]
async fn client_handle_action(
    client_data: ClientData,
    action: UserAction,
) -> Result<GameResponse, ()> {
    info!(?action, ?client_data, "Got handle_action request");
    let result = server::handle_action(DATABASE.clone(), client_data, action).await;
    match result {
        Ok(response) => Ok(response),
        Err(err) => {
            error!("Error on handle_action: {:?}", err);
            Err(())
        }
    }
}

fn main() -> Outcome {
    logging::initialize()?;
    if env::var("DISABLE_PANIC_HANDLER").is_err() {
        initialize::initialize_panic_handler()?;
    }
    let args = Cli::parse();
    ARGS.set(args).expect("Args should not be set multiple times");
    card_list::initialize();

    let commit = env!("VERGEN_GIT_SHA");
    info!(commit, "Starting game");

    let invoke_handler = {
        let builder = tauri_specta::ts::builder()
            .commands(tauri_specta::collect_commands![client_connect, client_handle_action]);

        #[cfg(debug_assertions)] // <- Only export on non-release builds
        let builder = builder.path("../../../src/generated_types.ts");

        builder.build().unwrap()
    };

    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(invoke_handler)
        .run(tauri::generate_context!())
        .with_error(|| "Failed to start tauri")?;

    outcome::OK
}
