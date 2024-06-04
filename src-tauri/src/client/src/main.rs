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

use all_cards::card_list;
use clap::Parser;
use data::actions::user_action::UserAction;
use data::core::primitives::UserId;
use database::sqlite_database::SqliteDatabase;
use display::commands::field_state::{FieldKey, FieldValue};
use game::server;
use game::server_data::{Client, ClientData, GameResponse};
use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};
use specta::Type;
use tauri::{AppHandle, EventTarget, Manager};
use tauri_specta::Event;
use tokio::sync::mpsc;
use tracing::info;
use utils::command_line::TracingStyle;
use utils::{command_line, paths};
use uuid::Uuid;

use crate::command_line_parser::CommandLineParser;

mod command_line_parser;
mod initialize;
mod logging;

static DATABASE: Lazy<SqliteDatabase> = Lazy::new(|| SqliteDatabase::new(paths::get_data_dir()));

#[derive(Debug, Clone, Serialize, Deserialize, Type, Event)]
pub struct GameResponseEvent(GameResponse);

#[tauri::command]
#[specta::specta]
async fn client_connect(app: AppHandle) {
    info!("Got connect request");
    let (sender, mut receiver) = mpsc::unbounded_channel();
    server::connect(DATABASE.clone(), sender, UserId(Uuid::default()));
    while let Some(response) = receiver.recv().await {
        app.emit_to(EventTarget::app(), "game_response", response).unwrap();
    }
}

#[tauri::command]
#[specta::specta]
async fn client_handle_action(client_data: ClientData, action: UserAction, app: AppHandle) {
    info!(?action, "Got handle_action request");
    let (sender, mut receiver) = mpsc::unbounded_channel();
    let mut client = Client { data: client_data, channel: sender };
    server::handle_action(DATABASE.clone(), &mut client, action).await;
    while let Some(response) = receiver.recv().await {
        app.emit_to(EventTarget::app(), "game_response", response).unwrap();
    }
}

#[tauri::command]
#[specta::specta]
async fn client_update_field(
    client_data: ClientData,
    key: FieldKey,
    value: FieldValue,
    app: AppHandle,
) {
    let (sender, mut receiver) = mpsc::unbounded_channel();
    let mut client = Client { data: client_data, channel: sender };
    server::handle_update_field(DATABASE.clone(), &mut client, key, value);
    while let Some(response) = receiver.recv().await {
        app.emit_to(EventTarget::app(), "game_response", response).unwrap();
    }
}

fn main() {
    let args = CommandLineParser::parse().build();
    command_line::FLAGS.set(args).expect("Flags should not be set multiple times");

    match command_line::flags().tracing_style {
        TracingStyle::AggregateTime => {
            tracing_span_tree::span_tree().aggregate(true).enable();
        }
        TracingStyle::Forest => {
            logging::initialize();
        }
        TracingStyle::None => {}
    }

    if env::var("DISABLE_PANIC_HANDLER").is_err() {
        initialize::initialize_panic_handler();
    }
    card_list::initialize();

    let commit = env!("VERGEN_GIT_SHA");
    info!(commit, "Starting game");

    let (invoke_handler, register_events) = {
        let builder = tauri_specta::ts::builder()
            .commands(tauri_specta::collect_commands![
                client_connect,
                client_handle_action,
                client_update_field
            ])
            .events(tauri_specta::collect_events![GameResponseEvent]);

        #[cfg(debug_assertions)] // <- Only export on non-release builds
        let builder = builder.path("../../../src/generated_types.ts");

        builder.build().unwrap()
    };

    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(invoke_handler)
        .setup(|app| {
            register_events(app);
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("Failed to start tauri");
}
