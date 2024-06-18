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

use data::card_states::card_kind::CardKind;
use data::card_states::zones::Zones;
use data::core::primitives::{Color, GameId, PlayerName, Source, UserId};
use data::decks::deck::Deck;
use data::decks::deck_name;
use data::decks::deck_name::DeckName;
use data::delegates::game_delegates::GameDelegates;
use data::game_states::game_state::{
    DebugConfiguration, GameConfiguration, GameState, GameStatus, TurnData,
};
use data::game_states::game_step::GamePhaseStep;
use data::game_states::history_data::GameHistory;
use data::game_states::oracle::Oracle;
use data::game_states::undo_tracker::UndoTracker;
use data::player_states::player_state::{PlayerType, Players};
use data::printed_cards::printed_card_id;
use data::state_machines::state_machine_data::StateMachines;
use database::sqlite_database::SqliteDatabase;
use enumset::EnumSet;
use maplit::hashmap;
use oracle::oracle_impl::OracleImpl;
use rand::SeedableRng;
use rand_xoshiro::Xoshiro256StarStar;
use rules::mutations::library;
use rules::steps::step;
use tracing::info;

use crate::game_creation::initialize_game;

/// Creates a new game using the provided Game ID, User IDs and decks and draws
/// opening hands.
///
/// A [SqliteDatabase] is required in order to populate the oracle information
/// for cards in this game. Nothing is written to the database as a part of
/// executing this function.
pub fn create_and_start(
    database: SqliteDatabase,
    game_id: GameId,
    p1: PlayerType,
    p1_deck_name: DeckName,
    p2: PlayerType,
    p2_deck_name: DeckName,
    debug: DebugConfiguration,
) -> GameState {
    info!(?game_id, "Creating new game");
    let mut game = create(database, game_id, p1, p1_deck_name, p2, p2_deck_name, debug);
    let _ = library::draw_cards(&mut game, Source::Game, PlayerName::One, 7);
    let _ = library::draw_cards(&mut game, Source::Game, PlayerName::Two, 7);
    // TODO: Resolve mulligans
    game.status = GameStatus::Playing;
    let _ = step::advance(&mut game);
    game
}

/// Creates a new game using the provided Game ID, User IDs and decks but does
/// not transition the game to the 'playing' state and does not e.g. draw
/// opening hands.
pub fn create(
    database: SqliteDatabase,
    game_id: GameId,
    p1: PlayerType,
    p1_deck_name: DeckName,
    p2: PlayerType,
    p2_deck_name: DeckName,
    debug: DebugConfiguration,
) -> GameState {
    let oracle = Box::new(OracleImpl::new(database.clone()));
    let p1_deck = find_deck(p1_deck_name);
    let p2_deck = find_deck(p2_deck_name);

    let mut game = create_game(oracle, game_id, p1, p1_deck, p2, p2_deck, debug);
    initialize_game::run(database.clone(), &mut game, None);

    game.shuffle_library(PlayerName::One);
    game.shuffle_library(PlayerName::Two);
    game
}

fn create_game(
    oracle: Box<dyn Oracle>,
    game_id: GameId,
    p1: PlayerType,
    p1_deck: Deck,
    p2: PlayerType,
    p2_deck: Deck,
    debug: DebugConfiguration,
) -> GameState {
    let mut zones = Zones::default();
    let turn = TurnData { active_player: PlayerName::One, turn_number: 0 };
    create_cards_in_deck(oracle.as_ref(), &mut zones, p1_deck, PlayerName::One, turn);
    create_cards_in_deck(oracle.as_ref(), &mut zones, p2_deck, PlayerName::Two, turn);

    GameState {
        id: game_id,
        status: GameStatus::Setup,
        step: GamePhaseStep::Untap,
        turn,
        priority: PlayerName::One,
        passed: EnumSet::empty(),
        configuration: GameConfiguration::new(PlayerName::One | PlayerName::Two, debug),
        state_machines: StateMachines::default(),
        players: Players::new(p1, p2, 20),
        zones,
        updates: None,
        combat: None,
        history: GameHistory::default(),
        rng: Xoshiro256StarStar::seed_from_u64(3141592653589793),
        undo_tracker: UndoTracker { enabled: true, undo: vec![] },
        delegates: GameDelegates::default(),
        state_based_events: Some(vec![]),
        oracle_reference: Some(oracle),
        agent_state: None,
        current_agent_searcher: None,
    }
}

fn create_cards_in_deck(
    oracle: &dyn Oracle,
    zones: &mut Zones,
    deck: Deck,
    owner: PlayerName,
    turn: TurnData,
) {
    for (&id, &quantity) in &deck.cards {
        for _ in 0..quantity {
            zones.create_card_in_library(oracle.card(id), CardKind::Normal, owner, turn);
        }
    }
}

fn find_deck(name: DeckName) -> Deck {
    match name {
        deck_name::GREEN_VANILLA => Deck {
            colors: EnumSet::only(Color::Green),
            cards: hashmap! {
                printed_card_id::FOREST => 35,
                printed_card_id::GRIZZLY_BEARS => 1,
                printed_card_id::GIGANTOSAURUS => 1,
                printed_card_id::ALPINE_GRIZZLY => 1,
                printed_card_id::LEATHERBACK_BALOTH => 1,
                printed_card_id::KALONIAN_TUSKER => 1,
                printed_card_id::ANCIENT_BRONTODON => 1,
                printed_card_id::GARRUKS_GOREHORN => 1,
                printed_card_id::GOLDEN_BEAR => 1,
                printed_card_id::PRIMORDIAL_WURM => 1,
                printed_card_id::VORSTCLAW => 1,
                printed_card_id::TERRAIN_ELEMENTAL => 1,
                printed_card_id::ORAZCA_FRILLBACK => 1,
                printed_card_id::SWORDWISE_CENTAUR => 1,
                printed_card_id::QUILLED_SLAGWURM => 1,
                printed_card_id::ELVISH_WARRIOR => 1,
                printed_card_id::NYXBORN_COLOSSUS => 1,
                printed_card_id::RUMBLING_BALOTH => 1,
                printed_card_id::GRIZZLED_OUTRIDER => 1,
                printed_card_id::CENTAUR_COURSER => 1,
                printed_card_id::GORILLA_WARRIOR => 1,
                printed_card_id::SILVERBACK_APE => 1,
                printed_card_id::PANTHER_WARRIORS => 1,
                printed_card_id::FEROCIOUS_ZHENG => 1,
                printed_card_id::ELVISH_RANGER => 1,
                printed_card_id::ENORMOUS_BALOTH => 1,
                printed_card_id::CRAW_WURM => 1,
                printed_card_id::BROODHUNTER_WURM => 1,
                printed_card_id::AXEBANE_STAG => 1,
                printed_card_id::SPINED_WURM => 1,
                printed_card_id::SCALED_WURM => 1,
                printed_card_id::ALPHA_TYRRANAX => 1,
                printed_card_id::WHIPTAIL_WURM => 1,
                printed_card_id::CANOPY_GORGER => 1,
                printed_card_id::VASTWOOD_GORGER => 1,
                printed_card_id::PHERES_BAND_CENTAURS => 1
            },
        },
        deck_name::DANDAN => Deck {
            colors: EnumSet::only(Color::Blue),
            cards: hashmap! {
                printed_card_id::ISLAND => 20,
                printed_card_id::BRAINSTORM => 20,
                printed_card_id::DANDAN => 20,
            },
        },
        _ => {
            panic!("Unknown deck {name:?}");
        }
    }
}
