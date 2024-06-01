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

use data::card_definitions::card_name;
use data::core::primitives::GameId;
use data::decks::deck_name;
use data::decks::deck_name::DeckName;
use data::game_states::game_state::{DebugConfiguration, GameState, GameStatus};
use data::game_states::game_step::GamePhaseStep;
use database::sqlite_database::SqliteDatabase;
use rules::game_creation::new_game;
use utils::command_line::CommandLine;
use utils::{command_line, paths};
use uuid::Uuid;

use crate::testing::test_game_builder::{TestGame, TestPlayer};

/// Create a new [GameState] for use in benchmarking & AI testing
pub fn create(deck_name: DeckName) -> GameState {
    command_line::FLAGS.set(CommandLine::default()).expect("Error setting command line flags");
    let database = SqliteDatabase::new(paths::get_data_dir()).unwrap();
    let mut game = new_game::create(
        database,
        GameId(Uuid::new_v4()),
        None,
        deck_name,
        None,
        deck_name,
        DebugConfiguration::default(),
    )
    .expect("Error creating game");
    game.status = GameStatus::Playing;
    game
}

pub fn vanilla_game_scenario() -> GameState {
    let mut game = create(deck_name::GREEN_VANILLA);
    let player = TestPlayer::new()
        .on_battlefield(card_name::FOREST)
        .on_battlefield(card_name::FOREST)
        .on_battlefield(card_name::FOREST)
        .on_battlefield(card_name::FOREST)
        .on_battlefield(card_name::GRIZZLY_BEARS)
        .on_battlefield(card_name::LEATHERBACK_BALOTH)
        .on_battlefield(card_name::ALPINE_GRIZZLY)
        .in_hand(card_name::FOREST)
        .in_hand(card_name::FOREST)
        .in_hand(card_name::FOREST)
        .in_hand(card_name::KALONIAN_TUSKER)
        .in_hand(card_name::GOLDEN_BEAR)
        .in_hand(card_name::TERRAIN_ELEMENTAL);

    TestGame::new()
        .step(GamePhaseStep::PreCombatMain)
        .player_1(player.clone())
        .player_2(player)
        .apply_to(&mut game);

    game
}
