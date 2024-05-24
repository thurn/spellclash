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

use serde::{Deserialize, Serialize};
use uuid::{uuid, Uuid};

use crate::card_definitions::card_name::CardName;

/// A unique ID for a specific printing of card.
///
/// This is the identifier for a card in the scryfall database. If you click
/// "copy-pasteable JSON' on scryfall, this is the identifier in the "id" field.
///
/// See <https://scryfall.com/docs/api/cards>
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub struct PrintedCardId(pub Uuid);

pub const GRIZZLY_BEARS: PrintedCardId =
    PrintedCardId(uuid!("409f9b88-f03e-40b6-9883-68c14c37c0de"));
pub const FOREST: PrintedCardId = PrintedCardId(uuid!("baf8a774-65f3-431e-b084-328ff1000895"));
