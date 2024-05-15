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
use tracing::info;
use utils::outcome;
use utils::outcome::Outcome;
use utils::with_error::WithError;

use crate::cli::{Cli, ARGS};

mod cli;
mod initialize;
mod logging;

#[tauri::command]
fn greet(name: String) -> String {
    info!(?name, "Got greet request");
    format!("Hello, {}! You've been greeted from Rust!", name)
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

    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![greet])
        .run(tauri::generate_context!())
        .with_error(|| "Failed to start tauri")?;

    outcome::OK
}
