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

use data::card_states::card_state::TappedState;
use dioxus::prelude::*;
use display::core::card_view::{CardView, RevealedCardStatus, RevealedCardView};
use display::core::game_view::GameView;
use game::server_data::ClientData;

use crate::client_actions::client_action;
use crate::game_components::game_component::CARD_HEIGHT;
use crate::main_app::cli::ARGS;

#[component]
pub fn CardComponent(card: CardView) -> Element {
    let c = card.clone();
    let tapped = card.tapped_state == TappedState::Tapped;
    match card.revealed {
        Some(revealed) => rsx! {
            div {
                class: "m-1",
                class: if tapped { "rotate-90" },
                RevealedCardComponent { card: c, revealed },
            }
        },
        None => rsx! {
            div {
                class: "m-1",
                class: if tapped { "rotate-90" },
                HiddenCardComponent { card },
            }
        },
    }
}

#[component]
pub fn RevealedCardComponent(card: CardView, revealed: RevealedCardView) -> Element {
    let cd_signal = consume_context::<Signal<ClientData>>();
    let view_signal = consume_context::<Signal<Option<GameView>>>();
    let nav = use_navigator();
    let width = CARD_HEIGHT * (5.0 / 7.0);
    let (class, label) = match revealed.status {
        None => ("border-2 border-black", String::new()),
        Some(RevealedCardStatus::CanPlay) => ("border-4 border-amber-300", String::new()),
        Some(RevealedCardStatus::Attacking(label)) => ("border-4 border-teal-300", label),
        Some(RevealedCardStatus::Blocking(label)) => ("border-4 border-purple-300", label),
    };
    let text_only = ARGS.get().map_or(false, |args| args.text_only);
    let name = revealed.face.name;

    rsx! {
        if let Some(action) = revealed.click_action {
            div {
                if text_only {
                    div {
                        width: "{width}vmin",
                        height: "{CARD_HEIGHT}vmin",
                        class: class,
                        onclick: { move |_|
                            client_action::client_execute_action(
                                cd_signal,
                                view_signal,
                                nav,
                                action
                            )
                        },
                        span {
                            style: "font-size: 2vmin",
                            "{name}"
                        }
                    }
                } else {
                    img {
                        src: revealed.face.image,
                        width: "{width}vmin",
                        height: "{CARD_HEIGHT}vmin",
                        class: class,
                        onclick: move |_|
                            client_action::client_execute_action(
                                cd_signal,
                                view_signal,
                                nav,
                                action
                            )
                    }
                }
                span {
                    style: "font-size: 2vmin",
                    "{label}"
                }
            }
        } else {
            if text_only {
                div {
                    class: "border-2 border-black",
                    width: "{width}vmin",
                    height: "{CARD_HEIGHT}vmin",
                    span {
                        style: "font-size: 2vmin",
                        "{name}"
                    }
                }
            } else {
                img {
                    src: revealed.face.image,
                    width: "{width}vmin",
                    height: "{CARD_HEIGHT}vmin"
                }
            }
            span {
                style: "font-size: 2vmin",
                "{label}"
            }
        }
    }
}

#[component]
pub fn HiddenCardComponent(card: CardView) -> Element {
    let width = CARD_HEIGHT * (5.0 / 7.0);
    let text_only = ARGS.get().map_or(false, |args| args.text_only);
    rsx! {
        if text_only {
            div {
                class: "border-2 border-black",
                width: "{width}vmin",
                height: "{CARD_HEIGHT}vmin",
            }
        } else {
            img {
                src: card.card_back,
                width: "{width}vmin",
                height: "{CARD_HEIGHT}vmin",
            }
        }
    }
}
