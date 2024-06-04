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

use data::game_states::game_state::GameState;
use data::game_states::oracle::Oracle;
use database::sqlite_database::SqliteDatabase;

use crate::oracle_impl::OracleImpl;

/// Update the printed card references and oracle reference for this game,
/// loading oracle card definitions from the database as needed.
pub fn populate(database: SqliteDatabase, game: &mut GameState) {
    let oracle = OracleImpl::new(database.clone());
    for card in game.zones.all_cards_mut() {
        card.printed_card_reference =
            Some(oracle.card(card.printed_card_id).printed_card_reference);
    }

    game.oracle_reference = Some(Box::new(oracle));
}
