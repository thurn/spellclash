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

use crate::core::primitives::EffectId;
use crate::game_states::this_turn_state::ThisTurnState;

/// Stores state information associated with abilities in an ongoing
/// game.
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct AbilityState {
    /// State which persists for the duration of the current turn.
    pub this_turn: ThisTurnState,

    /// Stores the [EffectId] which will be returned next from
    /// [Self::new_effect_id].
    ///
    /// Do not access this field directly.
    pub next_effect_id: EffectId,
}

impl AbilityState {
    /// Returns a new unique [EffectId] for use in this game.
    ///
    /// Please attempt to have less than eighteen quintillion effects in a
    /// single game.
    pub fn new_effect_id(&mut self) -> EffectId {
        let result = self.next_effect_id;
        self.next_effect_id = EffectId(result.0 + 1);
        result
    }
}
