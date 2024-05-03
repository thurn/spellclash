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

use data::actions::new_game_action::{NewGameAction, NewGameDebugOptions};
use data::actions::user_action::UserAction;
use data::core::primitives::{GameId, UserId};
use data::decks::deck_name;
use database::sled_database::SledDatabase;
use dioxus::desktop::{Config, WindowBuilder};
use dioxus::prelude::*;
use display::commands::command::Command;
use display::commands::scene_name::SceneName;
use display::core::game_view::GameView;
use once_cell::sync::Lazy;
use server;
use server::game;
use server::server_data::{ClientData, GameResponse};
use uuid::Uuid;

use crate::game_components::game_component::GameComponent;
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
    let view_signal_data: Option<GameView> = None;
    use_context_provider(|| Signal::new(view_signal_data));
    rsx! {
        style { {include_str!("../assets/style.css")} }
        Router::<Route> {}
    }
}

#[component]
fn Loading() -> Element {
    let client_data_signal = consume_context::<Signal<ClientData>>();
    let client_data = client_data_signal();
    let view_signal = consume_context::<Signal<Option<GameView>>>();
    let connection = use_resource(move || game::connect(DATABASE.as_ref(), client_data.user_id));
    let nav = use_navigator();
    match &*connection.read_unchecked() {
        Some(Ok(response)) => {
            handle_commands(response.clone(), nav, view_signal);
            rsx! { "Got response {response:?}" }
        }
        Some(Err(err)) => rsx! { "Error: {err:?}",  },
        None => rsx! { "Loading... " },
    }
}

async fn new_game() {
    let client_data = consume_context::<Signal<ClientData>>();
    let view_signal = consume_context::<Signal<Option<GameView>>>();
    let nav = use_navigator();
    let data = game::handle_action(
        DATABASE.as_ref(),
        client_data(),
        UserAction::NewGameAction(NewGameAction {
            deck: deck_name::ALL_GRIZZLY_BEARS,
            opponent_deck: deck_name::ALL_GRIZZLY_BEARS,
            opponent_id: None,
            debug_options: NewGameDebugOptions::default(),
        }),
    )
    .await;

    handle_commands(data.expect("Error in game response"), nav, view_signal);
}

#[component]
fn MainMenu() -> Element {
    rsx! {
        div {
            h1 {
                class: "text-4xl font-bold m-8",
                "Main Menu"
            }
            button {
                class: "m-8 align-middle select-none font-sans font-bold text-center uppercase transition-all disabled:opacity-50 disabled:shadow-none disabled:pointer-events-none text-xs py-3 px-6 rounded-lg bg-gray-900 text-white shadow-md shadow-gray-900/10 hover:shadow-lg hover:shadow-gray-900/20 focus:opacity-[0.85] focus:shadow-none active:opacity-[0.85] active:shadow-none",
                onclick: move |_| new_game(),
                "New Game "
            }
        }
    }
}

#[component]
fn Game(id: GameId) -> Element {
    let view_signal = consume_context::<Signal<Option<GameView>>>();
    let view = view_signal();
    match view {
        Some(game) => rsx! {
            GameComponent { view: game }
        },
        None => rsx! {
            "No GameView"
        },
    }
}

fn handle_commands(
    response: GameResponse,
    navigator: Navigator,
    mut view_signal: Signal<Option<GameView>>,
) {
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
