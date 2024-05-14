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

use dioxus::prelude::*;
use display::core::card_view::CardView;
use display::core::game_view::{DisplayPlayer, GameButton, GameView, PlayerView};
use display::core::object_position::{BattlefieldPosition, Position};
use game::server_data::ClientData;

use crate::client_actions::client_action;
use crate::game_components::button_component;
use crate::game_components::card_component::CardComponent;

pub const CARD_HEIGHT: f64 = 10.0;

#[component]
pub fn GameComponent(view: GameView) -> Element {
    let view = Arc::new(view);
    rsx! {
        div {
            class: "flex flex-row w-full",
            div {
                class: "flex-col w-5/6",
                Zones { view: view.clone() }
            }
            div {
                class: "flex flex-col justify-between w-1/6",
                GameInfo { view: view.clone() }
            }
        }
    }
}

#[component]
fn GameInfo(view: Arc<GameView>) -> Element {
    let top = view.top_buttons.clone();
    let bottom = view.bottom_buttons.clone();

    rsx! {
        GameButtons { buttons: top }
        div {
            class: "flex flex-col items-center",
            PlayerComponent {
                player: view.opponent.clone(),
                name: "Opponent",
            }
        }
        div {
            class: "flex flex-col items-center",
            div {
                style: "font-size: 1.5vmin",
                class: "mr-2",
                "{view.status_description}"
            }
        }
        div {
            class: "flex flex-col items-center",
            PlayerComponent {
                player: view.viewer.clone(),
                name: "Viewer",
            }
        }
        GameButtons { buttons: bottom }
    }
}

#[component]
fn GameButtons(buttons: Vec<GameButton>) -> Element {
    rsx! {
        div {
            class: "flex flex-row",
            for button in buttons {
                GameButton {
                    model: button
                }
            }
        }
    }
}

#[component]
fn GameButton(model: GameButton) -> Element {
    let cd_signal = consume_context::<Signal<ClientData>>();
    let view_signal = consume_context::<Signal<Option<GameView>>>();
    let nav = use_navigator();

    rsx! {
        button {
            style: "font-size: 2vmin",
            class: button_component::CLASS,
            onclick: move |_| {
                client_action::client_execute_action(cd_signal, view_signal, nav, model.action)
            },
            "{model.label}"
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
                style: "font-size: 1.5vmin",
                class: "mr-2",
                "{name}"
            }
            div {
                style: "font-size: 2vmin",
                class: "mr-2",
                "Life: {player.life}"
            }
            div {
                style: "font-size: 1.5vmin",
                class: "mr-2",
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
            style: "height: {CARD_HEIGHT}vmin",
            div {
                class: "flex flex-row",
                for card in cards {
                    CardComponent { card }
                }
                div {
                    style: "font-size: 2vmin",
                    class: "ml-2",
                    "{name}"
                }
            }
        }
    }
}
