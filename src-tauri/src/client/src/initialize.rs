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

use std::panic;
use std::panic::PanicInfo;

use color_eyre::config::{HookBuilder, PanicHook};
use color_eyre::eyre;
use utils::paths;
use utils::paths::LOG_FILE;

pub fn initialize_panic_handler() {
    let (panic_hook, eyre_hook) = HookBuilder::default()
        .panic_section(format!(
            "This is a bug. Consider reporting it at {}",
            env!("CARGO_PKG_REPOSITORY")
        ))
        .capture_span_trace_by_default(false)
        .display_location_section(false)
        .display_env_section(false)
        .into_hooks();

    // convert from a color_eyre EyreHook to a eyre ErrorHook
    let eyre_hook = eyre_hook.into_eyre_hook();
    eyre::set_hook(Box::new(move |error: &(dyn std::error::Error + 'static)| eyre_hook(error)))
        .expect("Error setting eyre_hook");
    panic::set_hook(Box::new(move |panic_info| {
        on_panic(&panic_hook, panic_info);
    }));
}

fn on_panic(panic_hook: &PanicHook, panic_info: &PanicInfo) {
    let msg = format!("{}", panic_hook.panic_report(panic_info));
    log::error!("Error: {}", strip_ansi_escapes::strip_str(msg));

    // human_panic stack trace for release builds
    #[cfg(not(debug_assertions))]
    {
        use human_panic::{handle_dump, print_msg, Metadata};
        let meta = Metadata {
            version: env!("CARGO_PKG_VERSION").into(),
            name: env!("CARGO_PKG_NAME").into(),
            authors: env!("CARGO_PKG_AUTHORS").replace(':', ", ").into(),
            homepage: env!("CARGO_PKG_HOMEPAGE").into(),
        };

        let file_path = handle_dump(&meta, panic_info);
        // prints human-panic message
        print_msg(file_path, &meta).expect("human-panic: printing error message to console failed");
        eprintln!("{}", panic_hook.panic_report(panic_info)); // prints color-eyre stack trace to stderr
    }

    // better_panic stack trace for debug builds
    #[cfg(debug_assertions)]
    {
        better_panic::Settings::auto()
            .most_recent_first(false)
            .lineno_suffix(true)
            .verbosity(better_panic::Verbosity::Full)
            .create_panic_handler()(panic_info);
    }

    std::process::exit(libc::EXIT_FAILURE);
}

pub fn version() -> String {
    let version = env!("CARGO_PKG_VERSION");
    let build_date = env!("VERGEN_BUILD_DATE");
    let sha = env!("VERGEN_GIT_SHA");
    let log_file = paths::get_data_dir().join(LOG_FILE.clone());
    let log_file_string = log_file.display();
    let data_dir_path = paths::get_data_dir().display().to_string();

    format!(
        "\
{version}
Built: {build_date}
Commit: {sha}
Log file: {log_file_string}
Data directory: {data_dir_path}"
    )
}
