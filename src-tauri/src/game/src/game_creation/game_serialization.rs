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

use data::core::primitives::PlayerName;
use data::game_states::game_state::{GameOperationMode, GameState};
use data::game_states::serialized_game_state::{SerializedGameState, SerializedGameVersion};
use data::player_states::player_map::PlayerMap;
use data::player_states::player_state::PlayerQueries;
use database::sqlite_database::SqliteDatabase;
use rules::action_handlers::actions;
use rules::action_handlers::actions::ExecuteAction;
use rules::legality::legal_actions;

use crate::game_creation::new_game;

/// Serializes the state of a game to a [SerializedGameState].
pub fn serialize(game: &GameState) -> SerializedGameState {
    SerializedGameState {
        version: SerializedGameVersion::Version1,
        id: game.id,
        seed: game.rng_seed,
        start_player: PlayerName::One,
        decks: PlayerMap::build_from(&game.players, |players, name| players.player(name).deck_name),
        player_types: PlayerMap::build_from(&game.players, |players, name| {
            players.player(name).player_type.clone()
        }),
        player_actions: game.history.player_actions.clone(),
        prompt_responses: game.history.prompt_responses.clone(),
        debug_configuration: game.configuration.debug.clone(),
    }
}

/// Builds a new [GameState] from a [SerializedGameState] by replaying all game
/// actions.
pub fn rebuild(database: SqliteDatabase, mut serialized: SerializedGameState) -> GameState {
    let mut game = new_game::create_and_start(
        database,
        serialized.id,
        serialized.player_types.one,
        serialized.decks.one,
        serialized.player_types.two,
        serialized.decks.two,
        serialized.debug_configuration,
    );
    game.operation_mode = GameOperationMode::SerializationReplay(serialized.prompt_responses);

    while serialized.player_actions.values().any(|(player, actions)| !actions.is_empty()) {
        let player = legal_actions::next_to_act(&game, None)
            .expect("Game is over but actions are non-empty");
        let is_agent = game.player(player).player_type.is_agent();
        let taken = serialized.player_actions.get_mut(player).remove(0);
        actions::execute(&mut game, player, taken.action, ExecuteAction {
            skip_undo_tracking: true,
            validate: false,
        });
    }

    game.operation_mode = GameOperationMode::Playing;
    game
}
