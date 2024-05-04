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

use std::collections::HashMap;
use std::sync::Arc;

use data::actions::game_action::GameAction;
use data::actions::user_action::UserAction;
use dioxus::prelude::*;
use display::core::card_view::CardView;
use display::core::game_view::{DisplayPlayer, GameView, PlayerView};
use display::core::object_position::{BattlefieldPosition, Position};
use server::server_data::ClientData;
use tracing::debug;

use crate::client_actions::client_action;
use crate::game_components::button_component;
use crate::game_components::card_component::CardComponent;

pub const CARD_HEIGHT: u64 = 120;

#[component]
pub fn GameComponent(view: GameView) -> Element {
    let view = Arc::new(view);
    rsx! {
        div {
            class: "flex flex-row",
            div {
                class: "flex-col",
                Zones { view: view.clone() }
            }
            div {
                class: "flex flex-col justify-between",
                GameInfo { view: view.clone() }
            }
        }
    }
}

async fn leave_game(
    cd_signal: Signal<ClientData>,
    view_signal: Signal<Option<GameView>>,
    nav: Navigator,
) {
    debug!("Request to leave game");
    client_action::apply(cd_signal, view_signal, nav, UserAction::LeaveGameAction).await;
}

#[component]
fn GameInfo(view: Arc<GameView>) -> Element {
    let cd_signal = consume_context::<Signal<ClientData>>();
    let view_signal = consume_context::<Signal<Option<GameView>>>();
    let nav = use_navigator();

    rsx! {
        div {
            class: "flex flex-col items-center",
            button {
                class: button_component::CLASS,
                onclick: move |_| leave_game(cd_signal, view_signal, nav),
                "Leave Game",
            }
            PlayerComponent {
                player: view.opponent.clone(),
                name: "Opponent",
            }
        }
        div {
            class: "flex flex-col items-center",
            div {
                class: "mr-2 text-m",
                "{view.status_description}"
            }
        }
        div {
            class: "flex flex-col items-center",
            PlayerComponent {
                player: view.viewer.clone(),
                name: "Viewer",
            }
            if view.can_pass_priority {
                button {
                    class: button_component::CLASS,
                    onclick: move |_| client_action::apply(
                        cd_signal,
                        view_signal,
                        nav,
                        GameAction::PassPriority
                    ),
                    "Continue",
                }
            } else {
                button {
                    class: button_component::DISABLED,
                    disabled: true,
                    "Continue",
                }
            }
        }
    }
}

#[component]
fn Zones(view: Arc<GameView>) -> Element {
    let positions = card_positions(&view);
    rsx! {
        ZoneComponent {
            name: "Opponent Hand",
            cards: positions.get(&Position::Hand(
                DisplayPlayer::Opponent)).unwrap_or(&vec![]).clone(),
        }
        ZoneComponent {
            name: "Opponent Mana",
            cards: positions.get(&Position::Battlefield(
                DisplayPlayer::Opponent, BattlefieldPosition::Mana)).unwrap_or(&vec![]).clone(),
        }
        ZoneComponent {
            name: "Opponent Permanents",
            cards: positions.get(&Position::Battlefield(
                DisplayPlayer::Opponent, BattlefieldPosition::Permanents)).unwrap_or(&vec![]).clone(),
        }
        ZoneComponent {
            name: "Stack",
            cards: positions.get(&Position::Stack).unwrap_or(&vec![]).clone(),
        }
        ZoneComponent {
            name: "Viewer Permanents",
            cards: positions.get(&Position::Battlefield(
                DisplayPlayer::Viewer, BattlefieldPosition::Permanents)).unwrap_or(&vec![]).clone(),
        }
        ZoneComponent {
            name: "Viewer Mana",
            cards: positions.get(&Position::Battlefield(
                DisplayPlayer::Viewer, BattlefieldPosition::Mana)).unwrap_or(&vec![]).clone(),
        }
        ZoneComponent {
            name: "Viewer Hand",
            cards: positions.get(&Position::Hand(
                DisplayPlayer::Viewer)).unwrap_or(&vec![]).clone(),
        }
    }
}

fn card_positions(view: &GameView) -> HashMap<Position, Vec<CardView>> {
    let mut cards = HashMap::new();
    for card in &view.cards {
        let position = card.position.clone();
        let cards = cards.entry(position.position).or_insert_with(Vec::new);
        cards.push((position.sorting_key, card.clone()));
    }
    let mut result = HashMap::new();
    for (position, mut cards) in cards {
        cards.sort_by_key(|(key, _)| *key);
        result.insert(position, cards.into_iter().map(|(_, card)| card).collect());
    }
    result
}

#[component]
fn PlayerComponent(name: &'static str, player: PlayerView) -> Element {
    rsx! {
        div {
            style: "width: 100px",
            class: "m-2 text-center",
            div {
                class: "mr-2 text-xl",
                "{name}"
            }
            div {
                class: "mr-2 text-m",
                "Life: {player.life}"
            }
            div {
                class: "mr-2 text-m",
                "Can Act: {player.can_act}"
            }
        }
    }
}

#[component]
fn ZoneComponent(name: &'static str, cards: Vec<CardView>) -> Element {
    rsx! {
        div {
            class: "bg-slate-300 m-2 rounded",
            style: "height: {CARD_HEIGHT}px",
            div {
                class: "flex flex-row",
                for card in cards {
                    CardComponent { card }
                }
                div {
                    class: "ml-2",
                    "{name}"
                }
            }
        }
    }
}
