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

use std::sync::Arc;

use data::actions::new_game_action::NewGameAction;
use data::card_definitions::card_name;
use data::card_states::card_kind::CardKind;
use data::card_states::zones::Zones;
use data::core::numerics::LifeValue;
use data::core::primitives::{GameId, PlayerName, Source, UserId};
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
use data::users::user_state::UserActivity;
use database::sqlite_database::SqliteDatabase;
use display::commands::scene_identifier::SceneIdentifier;
use display::core::display_state::DisplayState;
use display::rendering::render;
use enumset::EnumSet;
use maplit::hashmap;
use oracle::oracle_impl::OracleImpl;
use rand_xoshiro::rand_core::SeedableRng;
use rand_xoshiro::Xoshiro256StarStar;
use rules::mutations::library;
use rules::steps::step;
use tracing::info;
use uuid::Uuid;

use crate::game_creation::new_game;
use crate::server_data::{Client, ClientData, GameResponse};
use crate::{game_action_server, requests};

pub fn create(database: SqliteDatabase, client: &mut Client, action: NewGameAction) {
    let mut user = requests::fetch_user(database.clone(), client.data.user_id);

    let game_id = if let Some(id) = action.debug_options.override_game_id {
        id
    } else {
        GameId(Uuid::new_v4())
    };

    // TODO: Determine start player at random
    let mut game = new_game::create_and_start(
        database.clone(),
        game_id,
        PlayerType::Human(user.id),
        action.deck,
        action.opponent,
        action.opponent_deck,
        action.debug_options.configuration,
    );
    if let Some(action) = game_action_server::auto_pass_action(&game, PlayerName::One) {
        // Pass priority until the first configured stop.
        game_action_server::handle_game_action_internal(
            database.clone(),
            client,
            action,
            &mut game,
        );
    }

    user.activity = UserActivity::Playing(game.id);
    client.data.scene = SceneIdentifier::Game(game.id);
    let state = DisplayState::default();
    let commands = render::connect(&game, game.find_player_name(user.id), &state);

    database.write_game(&game);
    database.write_user(&user);
    client.send_all(commands);
}
