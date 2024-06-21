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

/// Unique identifier for a deck.
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub struct DeckName(pub Uuid);

pub const GREEN_VANILLA: DeckName = DeckName(uuid!("9eefebd4-25c0-4f46-91ab-6b3efff302e4"));
pub const ALL_DANDANS: DeckName = DeckName(uuid!("73af6e3b-822e-4294-affc-d2e6a96e9c54"));
pub const GRIZZLY_BEAR_GIANT_GROWTH: DeckName =
    DeckName(uuid!("9420275f-d9aa-4447-999e-205c013efcc5"));
pub const DANDAN: DeckName = DeckName(uuid!("c66ab7d9-5016-4d27-b1a1-6af8336af986"));
