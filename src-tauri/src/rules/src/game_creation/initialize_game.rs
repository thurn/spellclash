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

use data::game_states::animation_tracker::{AnimationState, AnimationTracker};
use data::game_states::game_state::GameState;
use database::sqlite_database::SqliteDatabase;
use oracle::card_database;

pub fn run(database: SqliteDatabase, game: &mut GameState) {
    for previous in game.undo_tracker.undo.iter_mut() {
        run(database.clone(), previous.as_mut());
    }

    game.animations = AnimationTracker::new(if game.configuration.simulation {
        AnimationState::Ignore
    } else {
        AnimationState::Track
    });
    card_database::populate(database, game)
}
