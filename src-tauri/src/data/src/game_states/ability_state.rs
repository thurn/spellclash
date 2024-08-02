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

use primitives::game_primitives::{CardId, EventId, PermanentId, Timestamp};

use crate::game_states::effect_state::EffectState;
use crate::game_states::state_value::StateValue;
use crate::properties::duration::Duration;

/// Stores state information associated with abilities in an ongoing
/// game.
#[derive(Default, Clone)]
pub struct AbilityState {
    /// Stores arbitrary state values associated with a given [EventId].
    ///
    /// This is always manipulated via [EffectState], do not access this field
    /// directly.
    pub effect_state: BTreeMap<EventId, StateValue>,

    /// [EventId]s of one-time effects which have already fired and thus should
    /// not trigger again.
    pub fired_one_time_effects: BTreeSet<EventId>,

    /// List of control-changing effects to automatically clean up at end of
    /// turn.
    pub change_control_this_turn: Option<Vec<(EventId, CardId)>>,
}

impl AbilityState {
    /// Returns & removes the list of control-changing effects to automatically
    /// clean up at end of turn
    pub fn remove_control_changing_effects(&mut self) -> Vec<(EventId, CardId)> {
        self.change_control_this_turn.take().unwrap_or_default()
    }

    /// Adds a control-changing effect to automatically clean up at end of turn.
    pub fn add_control_changing_effect(&mut self, event_id: EventId, card_id: CardId) {
        self.change_control_this_turn.get_or_insert_with(Vec::new).push((event_id, card_id));
    }
}
