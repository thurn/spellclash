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
use data::game_states::game_state::GameState;
use data::player_states::game_agent::{AgentType, GameAgent};
use data::player_states::player_state::{PlayerQueries, PlayerState, PlayerType};
use data::prompts::game_update::UpdateChannel;
use database::sqlite_database::SqliteDatabase;
use oracle::card_database;

pub fn run(database: SqliteDatabase, game: &mut GameState, update_channel: Option<UpdateChannel>) {
    for previous in game.undo_tracker.undo.iter_mut() {
        run(database.clone(), previous.as_mut(), None);
    }
    card_database::populate(database, game);

    for player in enum_iterator::all::<PlayerName>() {
        if let PlayerType::Agent(agent) = &mut game.player_mut(player).player_type {
            initialize_agent(agent);
        }
    }

    game.updates = update_channel;
}

fn initialize_agent(agent: &mut GameAgent) {
    match agent.agent_type {
        AgentType::FirstAvailableAction => {}
        AgentType::TreeSearch(_) => {}
        AgentType::MonteCarlo(_) => {}
    }
    todo!()
}
