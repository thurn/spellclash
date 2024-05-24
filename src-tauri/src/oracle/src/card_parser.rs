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

use data::printed_cards::database_card::DatabaseCardFace;
use data::printed_cards::printed_card::PrintedCard;
use utils::outcome::Value;

/// Turns a [DatabaseCardFace] into a [PrintedCard].
///
/// This translates the raw data coming from MTGJSON into a more useful rust
/// data structure.
///
/// Returns an error if the card doesn't follow the expected format.
pub fn parse(faces: Vec<DatabaseCardFace>) -> Value<PrintedCard> {
    todo!("Implement {:?}", faces[0].name)
}
