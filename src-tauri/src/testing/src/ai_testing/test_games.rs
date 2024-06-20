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

use all_cards::card_list;
use data::card_definitions::card_name;
use data::core::primitives::GameId;
use data::decks::deck_name;
use data::decks::deck_name::DeckName;
use data::game_states::game_state::{DebugConfiguration, GameState, GameStatus};
use data::game_states::game_step::GamePhaseStep;
use data::player_states::player_state::PlayerType;
use database::sqlite_database::SqliteDatabase;
use game::game_creation::new_game;
use utils::paths;
use uuid::Uuid;

use crate::ai_testing::test_game_builder::{TestGame, TestPlayer};

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

/// Create a new [GameState] for use in benchmarking & AI testing
pub fn create(deck_name: DeckName) -> GameState {
    card_list::initialize();
    let database = SqliteDatabase::new(paths::get_data_dir());
    let mut game = new_game::create(
        database.clone(),
        GameId(Uuid::new_v4()),
        PlayerType::None,
        deck_name,
        PlayerType::None,
        deck_name,
        DebugConfiguration::default(),
    );
    *game.status_mut() = GameStatus::Playing;
    game.undo_tracker_mut().enabled = false;
    game.undo_tracker_mut().undo.clear();
    *game.updates_mut() = None;
    game
}
