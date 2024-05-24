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

use data::card_states::card_reference::CardReference;
use data::game_states::oracle::Oracle;
use data::printed_cards::printed_card_id::PrintedCardId;
use database::sqlite_database::SqliteDatabase;

#[derive(Debug, Clone)]
pub struct OracleImpl {
    database: SqliteDatabase,
}

impl OracleImpl {
    pub fn new(database: SqliteDatabase) -> Self {
        Self { database }
    }
}

impl Oracle for OracleImpl {
    fn card(&self, id: PrintedCardId) -> CardReference {
        todo!()
    }
}
