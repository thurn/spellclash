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

use crate::core::primitives::{AbilityId, CardId, EntityId, ObjectId};
use crate::delegates::scope::Scope;

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct EffectOrigin {
    pub ability_id: AbilityId,
    pub object_id: ObjectId,
}

/// Stores a state mapping for effects that persist until the end of the current
/// turn.
///
/// All state stored here is dropped during the cleanup step of each turn.
#[serde_as]
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ThisTurnState {
    /// List of effects active this turn affecting each entity.
    #[serde_as(as = "Vec<(_, _)>")]
    effects: HashMap<EntityId, Vec<EffectOrigin>>,

    /// List of control-changing effects to automatically clean up at end of
    /// turn.
    control_changing_effects: Option<Vec<(Scope, CardId)>>,
}

impl ThisTurnState {
    /// Marks a new effect which persists until end of turn for a given
    /// [EffectOrigin].
    pub fn add_effect(&mut self, origin: EffectOrigin, target: EntityId) {
        self.effects.entry(target).or_default().push(origin);
    }

    /// Returns the number of times the [AbilityId] ability has been applied to
    /// the provided target entity this turn.
    pub fn application_count(&self, ability_id: AbilityId, target: EntityId) -> usize {
        self.effects
            .get(&target)
            .map_or(0, |e| e.iter().filter(|e| e.ability_id == ability_id).count())
    }

    /// Returns & removes the list of control-changing effects to automatically
    /// clean up at end of turn
    pub fn take_control_changing_effects(&mut self) -> Vec<(Scope, CardId)> {
        self.control_changing_effects.take().unwrap_or_default()
    }

    /// Adds a control-changing effect to automatically clean up at end of turn.
    pub fn add_control_changing_effect(&mut self, scope: Scope, card_id: CardId) {
        self.control_changing_effects.get_or_insert_with(Vec::new).push((scope, card_id));
    }
}
