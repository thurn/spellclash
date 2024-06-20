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

use data::card_definitions::card_name::CardName;
use data::card_states::card_state::CardFacing;
use data::card_states::zones::ZoneQueries;
use data::core::primitives::{PlayerName, Source, Zone};
use data::game_states::game_state::GameState;
use data::game_states::game_step::GamePhaseStep;
use data::printed_cards::printed_card::Face;
use rules::mutations::move_card;

#[derive(Debug, Clone, Default)]
pub struct TestGame {
    p1: TestPlayer,
    p2: TestPlayer,
    step: Option<GamePhaseStep>,
}

impl TestGame {
    pub fn new() -> Self {
        Self { p1: TestPlayer::default(), p2: TestPlayer::default(), step: None }
    }

    pub fn player_1(mut self, player: TestPlayer) -> Self {
        self.p1 = player;
        self
    }

    pub fn player_2(mut self, player: TestPlayer) -> Self {
        self.p2 = player;
        self
    }

    pub fn step(mut self, step: GamePhaseStep) -> Self {
        self.step = Some(step);
        self
    }

    pub fn apply_to(self, game: &mut GameState) {
        if let Some(step) = self.step {
            game.step = step;
        }
        self.p1.apply_to(game, PlayerName::One);
        self.p2.apply_to(game, PlayerName::Two);
    }
}

#[derive(Debug, Clone, Default)]
pub struct TestPlayer {
    hand: Vec<CardName>,
    battlefield: Vec<CardName>,
}

impl TestPlayer {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn in_hand(mut self, card: CardName) -> Self {
        self.hand.push(card);
        self
    }

    pub fn on_battlefield(mut self, card: CardName) -> Self {
        self.battlefield.push(card);
        self
    }

    pub fn apply_to(self, state: &mut GameState, player_name: PlayerName) {
        for card in self.hand {
            Self::move_to_zone(state, player_name, card, Zone::Hand, false);
        }
        for card in self.battlefield {
            Self::move_to_zone(state, player_name, card, Zone::Battlefield, true);
        }
    }

    fn move_to_zone(
        game: &mut GameState,
        player: PlayerName,
        name: CardName,
        zone: Zone,
        turn_primary_face_up: bool,
    ) {
        let id = *game
            .library(player)
            .iter()
            .find(|&id| game.card(id).card_name == name)
            .unwrap_or_else(|| panic!("Card {name:?} not found in library"));
        move_card::run(game, Source::Game, id, zone).unwrap();
        if turn_primary_face_up {
            game.card_mut(id).facing = CardFacing::FaceUp(Face::Primary);
        }
    }
}
