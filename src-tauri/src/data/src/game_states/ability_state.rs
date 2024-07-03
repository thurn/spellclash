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

use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use serde_with::serde_as;

use crate::core::primitives::{AbilityId, CardId, EffectId, EntityId};
use crate::game_states::effect_state::EffectState;
use crate::game_states::state_value::StateValue;
use crate::game_states::this_turn_state::ThisTurnState;

/// Stores state information associated with abilities in an ongoing
/// game.
#[serde_as]
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct AbilityState {
    /// State which persists for the duration of the current turn.
    pub this_turn: ThisTurnState,

    /// Stores a list of delayed trigger activation IDs for each ability. An
    /// ability with an entry in this map will have its delayed trigger placed
    /// on the stack when the trigger condition occurs.
    ///
    /// This is mutated via the `delayed_trigger` module, do not access this
    /// field directly.
    #[serde_as(as = "Vec<(_, _)>")]
    pub delayed_triggers: HashMap<AbilityId, Vec<EffectId>>,

    /// Stores arbitrary state values associated with a given [EffectId].
    ///
    /// This is always manipulated via [EffectState], do not access this field
    /// directly.
    #[serde_as(as = "Vec<(_, _)>")]
    pub effect_state: HashMap<EffectId, StateValue>,
}
