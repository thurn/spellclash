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

use data::core::primitives::{GameId, UserId};
use database::sled_database::SledDatabase;
use dioxus::desktop::{Config, WindowBuilder};
use dioxus::prelude::*;
use display::commands::command::Command;
use display::commands::scene_name::SceneName;
use once_cell::sync::Lazy;
use server;
use server::game;
use server::server_data::ClientData;
use uuid::Uuid;

use crate::initialize;

static DATABASE: Lazy<Arc<SledDatabase>> =
    Lazy::new(|| Arc::new(SledDatabase::new(initialize::get_data_dir())));

#[derive(Routable, Clone)]
#[rustfmt::skip]
enum Route {
    #[route("/")]
    Loading {},
    
    // Routes always start with a slash
    #[route("/main")]
    MainMenu {},
    // You can have multiple segments in a route
    #[route("/game/:id")]
    Game {
        id: GameId,
    },
}

pub fn launch() {
    LaunchBuilder::desktop()
        .with_cfg(
            Config::new()
                .with_window(WindowBuilder::default().with_title("✦ Spellclash ✦"))
                .with_custom_head(
                    "<script src=\"https://cdn.tailwindcss.com\"></script>".to_string(),
                ),
        )
        .launch(App);
}

#[component]
fn App() -> Element {
    let user_id = UserId(Uuid::default());
    use_context_provider(|| Signal::new(ClientData::for_user(user_id)));
    rsx! {
        style { {include_str!("../assets/style.css")} }
        Router::<Route> {}
    }
}

#[component]
fn Loading() -> Element {
    let client_data = consume_context::<Signal<ClientData>>();
    let d = client_data();
    let connection = use_resource(move || game::connect(DATABASE.as_ref(), d.user_id));
    let nav = use_navigator();
    match &*connection.read_unchecked() {
        Some(Ok(response)) => {
            handle_commands(&d, &response.commands, nav);
            rsx! { "Got response {response:?}" }
        }
        Some(Err(err)) => rsx! { "Error: {err:?}",  },
        None => rsx! { "Loading... " },
    }

    // rsx! {
    //     style { {include_str!("../assets/style.css")} }
    //     div { id: "wrapper",
    //         div { class: "w-32 h-32 bg-lime-500 rounded"}
    //         div { style: "margin-left: 10px" }
    //         div { class: "card blue" }
    //     }
    // }
}

#[component]
fn MainMenu() -> Element {
    let mut client_data = consume_context::<Signal<ClientData>>();
    rsx! {
        div { "Main Menu" }
    }
}

#[component]
fn Game(id: GameId) -> Element {
    let mut client_data = consume_context::<Signal<ClientData>>();
    rsx! {
        div { "Game" }
    }
}

fn handle_commands(_data: &ClientData, commands: &[Command], navigator: Navigator) {
    for command in commands {
        match command {
            Command::LoadScene { name, .. } => match name {
                SceneName::MainMenu => {
                    navigator.replace(Route::MainMenu {});
                }
                SceneName::Game(id) => {
                    navigator.replace(Route::Game { id: *id });
                }
            },
            Command::UpdateGameView { .. } => {}
        }
    }
}
