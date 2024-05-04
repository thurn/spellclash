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

use data::actions::new_game_action::{NewGameAction, NewGameDebugOptions};
use data::actions::user_action::UserAction;
use data::core::primitives::{GameId, UserId};
use data::decks::deck_name;
use dioxus::desktop::{Config, WindowBuilder};
use dioxus::prelude::*;
use display::core::game_view::GameView;
use game;
use game::server_data::ClientData;
use log::debug;
use uuid::Uuid;

use crate::client_actions::client_action;
use crate::game_components::button_component;
use crate::game_components::game_component::GameComponent;

#[derive(Routable, Clone)]
#[rustfmt::skip]
pub enum Route {
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
        style { {include_str!("../../assets/style.css")} }
        Router::<Route> {}
    }
}

#[component]
fn Loading() -> Element {
    let cd_signal = consume_context::<Signal<ClientData>>();
    let view_signal = consume_context::<Signal<Option<GameView>>>();
    let nav = use_navigator();
    use_future(move || client_action::connect(cd_signal, view_signal, nav));
    rsx! { "Loading... " }
}

async fn new_game(
    cd_signal: Signal<ClientData>,
    view_signal: Signal<Option<GameView>>,
    nav: Navigator,
) {
    debug!("Requesting new game");
    client_action::apply(
        cd_signal,
        view_signal,
        nav,
        UserAction::NewGameAction(NewGameAction {
            deck: deck_name::ALL_GRIZZLY_BEARS,
            opponent_deck: deck_name::ALL_GRIZZLY_BEARS,
            opponent_id: None,
            debug_options: NewGameDebugOptions::default(),
        }),
    )
    .await;
}

#[component]
fn MainMenu() -> Element {
    debug!("Rendering main menu");
    let cd_signal = consume_context::<Signal<ClientData>>();
    let view_signal = consume_context::<Signal<Option<GameView>>>();
    let nav = use_navigator();
    rsx! {
        div {
            h1 {
                class: "text-4xl font-bold m-8",
                "Main Menu"
            },
            button {
                class: button_component::CLASS,
                onclick: move |_| {
                    new_game(cd_signal, view_signal, nav)
                },
                "New Game"
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
