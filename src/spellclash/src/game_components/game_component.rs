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

use dioxus::prelude::*;
use display::core::card_view::CardView;
use display::core::game_view::{DisplayPlayer, GameView};
use display::core::object_position::{BattlefieldPosition, Position};

use crate::game_components::card_component::CardComponent;

pub const CARD_HEIGHT: u64 = 120;

#[component]
pub fn GameComponent(view: GameView) -> Element {
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
