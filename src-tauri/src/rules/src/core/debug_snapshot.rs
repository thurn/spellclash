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

use data::card_states::card_state::CardState;
use data::card_states::zones::ZoneQueries;
use data::core::function_types::CardPredicate;
use data::core::primitives::{CardId, PlayerName, Source, Zone};
use data::game_states::game_state::GameState;

use crate::predicates::card_predicates;

pub fn capture(game: &GameState) -> Vec<String> {
    let mut result = vec![];
    result.push(in_zone(
        "G2",
        game,
        PlayerName::Two,
        Zone::Graveyard,
        card_predicates::always_true,
    ));
    result.push(in_zone("H2", game, PlayerName::Two, Zone::Hand, card_predicates::always_true));
    result.push(in_zone("L2", game, PlayerName::Two, Zone::Battlefield, card_predicates::land));
    result.push(in_zone("B2", game, PlayerName::Two, Zone::Battlefield, card_predicates::nonland));
    result.push(in_zone("B1", game, PlayerName::One, Zone::Battlefield, card_predicates::nonland));
    result.push(in_zone("L1", game, PlayerName::One, Zone::Battlefield, card_predicates::land));
    result.push(in_zone("H1", game, PlayerName::One, Zone::Hand, card_predicates::always_true));
    result.push(in_zone(
        "G1",
        game,
        PlayerName::One,
        Zone::Graveyard,
        card_predicates::always_true,
    ));
    result
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

fn card_string(card: &CardState) -> String {
    let id = format!("{:?}", card.id);
    format!(
        "{}{}",
        card.displayed_name(),
        id.replace("CardId", "").replace("(", "").replace(")", "")
    )
}
