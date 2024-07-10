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

use std::fmt::Debug;

use data::card_states::card_state::CardState;
use data::card_states::zones::{HasZones, ToCardId, ZoneQueries};
use data::core::function_types::CardPredicate;
use data::core::primitives::{CardId, HasPlayerName, PlayerName, Source, Zone};
use data::game_states::game_state::GameState;
use data::player_states::player_state::{PlayerQueries, PlayerState};

use crate::predicates::card_predicates;

pub fn capture(game: &GameState) -> Vec<String> {
    vec![
        game_string(game),
        player_string("TWO", game.player(PlayerName::Two)),
        library("LIB2", game, PlayerName::Two),
        in_zone("G2", game, PlayerName::Two, Zone::Graveyard, card_predicates::always_true),
        in_zone("H2", game, PlayerName::Two, Zone::Hand, card_predicates::always_true),
        in_zone("M2", game, PlayerName::Two, Zone::Battlefield, card_predicates::land),
        in_zone("B2", game, PlayerName::Two, Zone::Battlefield, card_predicates::nonland),
        in_zone("B1", game, PlayerName::One, Zone::Battlefield, card_predicates::nonland),
        in_zone("M1", game, PlayerName::One, Zone::Battlefield, card_predicates::land),
        in_zone("H1", game, PlayerName::One, Zone::Hand, card_predicates::always_true),
        in_zone("G1", game, PlayerName::One, Zone::Graveyard, card_predicates::always_true),
        library("LIB1", game, PlayerName::One),
        player_string("ONE", game.player(PlayerName::One)),
    ]
}

pub fn capture_with(
    game: &GameState,
    player: impl HasPlayerName,
    card: impl ToCardId,
    arg: impl Debug,
) -> Vec<String> {
    let mut captured = capture(game);
    captured.insert(
        0,
        format!(
            "CONTEXT [Player: {:?}, Card: {}, Arg: {:?}]",
            player.player_name(),
            card_string(game.card(card.to_card_id(game.zones())).unwrap()),
            arg
        ),
    );
    captured
}

fn in_zone(
    key: &'static str,
    game: &GameState,
    player: PlayerName,
    zone: Zone,
    predicate: impl CardPredicate<CardId>,
) -> String {
    let cards = game
        .zones
        .cards_in_zone(zone, player)
        .filter(|card_id| predicate(game, Source::Game, *card_id) == Some(true))
        .filter_map(|card_id| Some(card_string(game.card(card_id)?)))
        .collect::<Vec<_>>()
        .join(", ");
    format!("{} [{}]", key, cards)
}

fn library(key: &'static str, game: &GameState, player: PlayerName) -> String {
    let cards = game
        .zones
        .library(player)
        .iter()
        .rev()
        .take(3)
        .filter_map(|&card_id| Some(card_string(game.card(card_id)?)))
        .collect::<Vec<_>>()
        .join(", ");
    format!("{} [{}]", key, cards)
}

fn game_string(game: &GameState) -> String {
    format!(
        "GAME [Turn:{:?} Step: {:?} Active:{:?}]",
        game.turn.turn_number, game.step, game.turn.active_player
    )
}

fn player_string(key: &'static str, player: &PlayerState) -> String {
    format!("{} [Life:{}]", key, player.life)
}

fn card_string(card: &CardState) -> String {
    let id = format!("{:?}", card.id);
    format!("{}{}", card.displayed_name(), id.replace("CardId", "").replace(['(', ')'], ""))
}
