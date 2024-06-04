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

use std::fs::File;
use std::{env, fs};

use tracing::{Event, Level};
use tracing_error::ErrorLayer;
use tracing_forest::{ForestLayer, PrettyPrinter, Tag};
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;
use tracing_subscriber::{EnvFilter, Layer};
use utils::paths;
use utils::paths::{LOG_ENV, LOG_FILE};

/// Initializes global logging behavior for the 'tracing' crate.
pub fn initialize() {
    env::set_var(
        "RUST_LOG",
        env::var("RUST_LOG")
            .or_else(|_| env::var(LOG_ENV.clone()))
            .unwrap_or_else(|_| "debug,sled=warn".to_string()),
    );

    let env_filter = if let Ok(v) = env::var("RUST_LOG") {
        EnvFilter::new(v)
    } else {
        EnvFilter::new("debug,sled=warn")
    };
    let forest_layer = ForestLayer::new(PrettyPrinter::new(), tag_parser).with_filter(env_filter);

    let directory = paths::get_data_dir();
    fs::create_dir_all(directory.clone()).expect("Error creating log dir");
    let log_path = directory.join(LOG_FILE.clone());
    let log_file = File::create(log_path).expect("Error creating log file");
    let file_subscriber = tracing_subscriber::fmt::layer()
        .with_file(true)
        .with_line_number(true)
        .with_writer(log_file)
        .with_target(false)
        .with_ansi(false)
        .with_filter(EnvFilter::default());

    tracing_subscriber::registry()
        .with(forest_layer)
        .with(file_subscriber)
        .with(ErrorLayer::default())
        .init();
}

fn tag_parser(event: &Event) -> Option<Tag> {
    let target = event.metadata().target();
    let level = *event.metadata().level();
    let icon = match target {
        _ if level == Level::ERROR => '🚨',
        _ if level == Level::WARN => '🚧',
        _ if target.contains("rules") => '🎴',
        _ if target.contains("game") => '💻',
        _ if target.contains("spellclash") => '🌐',
        _ if target.contains("ai") => '🤖',
        _ => match level {
            Level::TRACE => '📍',
            Level::DEBUG => '📝',
            _ => '💡',
        },
    };

    let mut builder = Tag::builder().level(level).icon(icon);
    builder = builder.prefix(target).suffix("rs");

    Some(builder.build())
}
