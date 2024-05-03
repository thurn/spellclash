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

use std::env;

use all_cards::card_list;
use clap::Parser;
use color_eyre::Result;
use tracing::info;

use crate::cli::Cli;

pub mod cli;
pub mod game_client;
pub mod initialize;

fn main() -> Result<()> {
    initialize::initialize_logging()?;
    if env::var("DISABLE_PANIC_HANDLER").is_err() {
        initialize::initialize_panic_handler()?;
    }
    Cli::parse();
    card_list::initialize();

    let commit = env!("VERGEN_GIT_SHA");
    info!(commit, "Starting game");

    game_client::launch();
    Ok(())
}
