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

/// Identifies a specific printing of a card or token.
///
/// This is a wrapper around the MJSON ID for the card and uniquely identifies
/// printings in the MTJSON database.
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub struct CardPrintingId(pub Uuid);

pub const GRIZZLY_BEARS: CardPrintingId =
    CardPrintingId(uuid!("4b35d338-f8cd-5d65-86f8-49cc76756592"));
pub const FOREST: CardPrintingId = CardPrintingId(uuid!("2f8a3f3d-3b11-5f26-b766-0b0bbda0a5bb"));
