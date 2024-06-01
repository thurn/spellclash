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

/// Identifies a named oracle instance of a card, i.e. something a player can
/// pick when resolving a "choose a card name" effect.
///
/// This is a wrapper around the Scryfall Oracle ID:
///
/// > A unique ID for this card’s oracle identity. This value is consistent
/// > across reprinted card editions, and unique among different cards with the
/// > same name (tokens, Unstable variants, etc). Always present except for the
/// > reversible_card layout where it will be absent; oracle_id will be found on
/// > each face instead.
///
/// You can find this information on a Scryfall card page by clicking
/// "Copy-pasteable JSON" and copying the oracle_id field.
///
/// See <https://scryfall.com/docs/api/cards>
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub struct CardName(pub Uuid);

pub const BRAINSTORM: CardName = CardName(uuid!("36cd2364-d113-47d1-b2c4-b088d9eb88dd"));
pub const DANDAN: CardName = CardName(uuid!("88929373-b2c8-4a81-a809-fed87fd5b0d7"));
pub const FOREST: CardName = CardName(uuid!("b34bb2dc-c1af-4d77-b0b3-a0fb342a5fc6"));
pub const GRIZZLY_BEARS: CardName = CardName(uuid!("14c8f55d-d177-4c25-a931-ebeb9e6062a0"));
pub const ISLAND: CardName = CardName(uuid!("b2c6aa39-2d2a-459c-a555-fb48ba993373"));
pub const MOUNTAIN: CardName = CardName(uuid!("a3fb7228-e76b-4e96-a40e-20b5fed75685"));
pub const PLAINS: CardName = CardName(uuid!("bc71ebf6-2056-41f7-be35-b2e5c34afa99"));
pub const SWAMP: CardName = CardName(uuid!("56719f6a-1a6c-4c0a-8d21-18f7d7350b68"));
pub const LEATHERBACK_BALOTH: CardName = CardName(uuid!("0ee14128-3bec-4b65-8ee4-619337d4ed45"));
pub const ALPINE_GRIZZLY: CardName = CardName(uuid!("86e3c86c-d50e-4aa9-b080-f68b6afee87c"));
pub const KALONIAN_TUSKER: CardName = CardName(uuid!("df7f697e-6886-4897-a024-61ae225c1b34"));
pub const GOLDEN_BEAR: CardName = CardName(uuid!("6ba7ab42-918e-472a-b364-06073cc354d2"));
pub const TERRAIN_ELEMENTAL: CardName = CardName(uuid!("3a9b9fe0-1846-41fc-b7c3-ed9abef27a6a"));
