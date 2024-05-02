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

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

/// Describes how the multiple faces of a card are organized in relation to each
/// other.
///
/// See <https://scryfall.com/docs/api/layouts>
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, JsonSchema)]
pub enum CardLayout {
    Adventure,
    Aftermath,
    Flip,
    Meld,
    ModalDfc,
    Normal,
    Split,
    Transform,
}

/// Describes how a single face of a card is laid out.
///
/// See <https://scryfall.com/docs/api/layouts>
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, JsonSchema)]
pub enum FaceLayout {
    Adventure,
    Aftermath,
    ArtSeries,
    Augment,
    Class,
    DoubleFacedToken,
    Emblem,
    Flip,
    Host,
    Leveler,
    Meld,
    ModalDfc,
    Normal,
    Planar,
    ReversibleCard,
    Saga,
    Scheme,
    Split,
    Token,
    Transform,
    Vanguard,
}
