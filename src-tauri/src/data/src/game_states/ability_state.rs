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

use std::collections::{BTreeMap, BTreeSet};
use primitives::game_primitives::EventId;
use crate::game_states::effect_state::EffectState;
use crate::game_states::state_value::StateValue;
use crate::game_states::this_turn_state::ThisTurnState;

/// Stores state information associated with abilities in an ongoing
/// game.
#[derive(Debug, Default, Clone)]
pub struct AbilityState {
    /// State which persists for the duration of the current turn.
    pub this_turn: ThisTurnState,

    /// Stores arbitrary state values associated with a given [EventId].
    ///
    /// This is always manipulated via [EffectState], do not access this field
    /// directly.
    pub effect_state: BTreeMap<EventId, StateValue>,

    /// [EventId]s of one-time effects which have already fired and thus should
    /// not trigger again.
    pub fired_one_time_effects: BTreeSet<EventId>,
}
