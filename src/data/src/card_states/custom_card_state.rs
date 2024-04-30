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

use crate::core::primitives::ObjectId;

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum CustomCardState {
    /// Affect some game object while it exists. The effect is assumed to end
    /// once this ObjectId expires (e.g. by the target moving to a different
    /// zone).
    TargetObject { object_id: ObjectId },
}

/// Records custom state entries for a given card.
///
/// This keeps track of miscellaneous state related to resolving a card's
/// abilities, such as targets which have been selected for this card. It is
/// designed as an "append-only" data structure, meaning that state entries are
/// never removed.
#[derive(Clone, Debug, Default, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct CustomCardStateList {
    list: Vec<CustomCardState>,
}
