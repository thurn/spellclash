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
use display::core::game_view::GameView;

use crate::game_components::card_component::CardComponent;

#[component]
pub fn GameComponent(view: GameView) -> Element {
    rsx! {
        "Game View"
        for card in view.cards {
            CardComponent { card }
        }
    }
}
