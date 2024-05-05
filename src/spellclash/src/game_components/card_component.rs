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

use dioxus::prelude::*;
use display::core::card_view::{CardView, RevealedCardView};

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
    let width = (CARD_HEIGHT as f64) * (5.0 / 7.0);
    rsx! {
        img {
            src: revealed.face.image,
            width: "{width}px",
            height: "{CARD_HEIGHT}px",
            class: if  revealed.can_play { "border-2 border-sky-500" }
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
