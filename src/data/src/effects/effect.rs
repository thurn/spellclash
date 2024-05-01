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

#[allow(unused)] // Used in docs
use crate::core::primitives::ObjectId;

/// Represents a list of game [Effect]s to apply
pub struct EffectList {
    effects: Vec<Effect>,
}

impl EffectList {
    /// Adds a new effect to this list
    pub fn push(&mut self, effect: Effect) {
        self.effects.push(effect)
    }
}

/// Represents a mutation to the state of an ongoing game.
///
/// Effects are implicitly associated with a set of chosen [ObjectId]s which
/// they should be applied to, and must be evaluated with an appropriate object
/// set. Generally effects should use this system instead of directly storing an
/// [ObjectId] internally. Effects should function properly with any number of
/// provided objects. Each effect's documentation should describe the
/// characteristics of its expected object set and the operation it performs on
/// those objects.
///
/// Effect implementations should silently ignore [ObjectId]s which no longer
/// exist.
///
/// From the Comprehensive Rules:
///
/// > An effect is something that happens in the game as a result of a spell or
/// > ability. When a spell, activated ability, or triggered ability resolves,
/// > it may create one or more one-shot or continuous effects.
///
/// See <https://yawgatog.com/resources/magic-rules/#R6091>
#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum Effect {
    DealOpeningHand,
}
