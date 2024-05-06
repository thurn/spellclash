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

use data::actions::game_action::GameAction;
use dioxus::prelude::*;
use display::core::card_view::{CardView, RevealedCardView};
use display::core::game_view::GameView;
use game::server_data::ClientData;

use crate::client_actions::client_action;
use crate::game_components::game_component::CARD_HEIGHT;

#[component]
pub fn CardComponent(card: CardView) -> Element {
    let c = card.clone();
    match card.revealed {
        Some(revealed) => rsx! {
            div {
                RevealedCardComponent { card: c, revealed },
            }
        },
        None => rsx! {
            div {
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
    let width = (CARD_HEIGHT as f64) * (5.0 / 7.0);
    rsx! {
        if revealed.can_play {
            img {
                src: revealed.face.image,
                width: "{width}px",
                height: "{CARD_HEIGHT}px",
                class: if revealed.can_play { "border-2 border-sky-500" },
                onclick: move |_|
                    client_action::client_execute_action(
                        cd_signal,
                        view_signal,
                        nav,
                        GameAction::PlayCard(card.id)
                    )
            }
        } else {
            img {
                src: revealed.face.image,
                width: "{width}px",
                height: "{CARD_HEIGHT}px",
            }
        }

    }
}

#[component]
pub fn HiddenCardComponent(card: CardView) -> Element {
    let width = (CARD_HEIGHT as f64) * (5.0 / 7.0);
    rsx! {
        img {
            src: card.card_back,
            width: "{width}px",
            height: "{CARD_HEIGHT}px",
        }
    }
}
