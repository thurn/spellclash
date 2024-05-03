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

#[component]
pub fn CardComponent(card: CardView) -> Element {
    let c = card.clone();
    match card.revealed {
        Some(revealed) => rsx! {
            div {
                RevealedCardComponent { card: c, revealed: revealed },
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
    rsx! {
        "Card: {revealed.face.name} in position {card.position:?}"
    }
}

#[component]
pub fn HiddenCardComponent(card: CardView) -> Element {
    rsx! {
        "Hidden Card in position {card.position:?}"
    }
}
