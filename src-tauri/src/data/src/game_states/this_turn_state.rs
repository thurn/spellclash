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

use std::collections::{HashMap, HashSet};
use std::iter;

use either::Either;
use serde::{Deserialize, Serialize};
use serde_with::serde_as;

use crate::core::primitives::{AbilityId, CardId, EffectId, EntityId, PermanentId};
use crate::delegates::scope::{EffectContext, Scope};

#[derive(Debug, Clone, Serialize, Deserialize)]
struct AbilityEffectId {
    pub ability_id: AbilityId,
    pub effect_id: EffectId,
}

/// Stores a state mapping for effects that persist until the end of the current
/// turn.
///
/// All state stored here is dropped during the cleanup step of each turn.
#[serde_as]
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ThisTurnState {
    /// Map from entities to lists of effects active this turn affecting that
    /// entity.
    #[serde_as(as = "Vec<(_, _)>")]
    effects: HashMap<EntityId, Vec<AbilityEffectId>>,

    /// List of control-changing effects to automatically clean up at end of
    /// turn.
    control_changing_effects: Option<Vec<(EffectId, CardId)>>,

    /// Permanents that have lost all abilities this turn.
    lost_all_abilities: HashSet<PermanentId>,
}

impl ThisTurnState {
    /// Marks a new effect which persists until end of turn for a given
    /// [EffectContext].
    pub fn add_effect(&mut self, source: AbilityId, effect_id: EffectId, target: EntityId) {
        self.effects
            .entry(target)
            .or_default()
            .push(AbilityEffectId { ability_id: source, effect_id });
    }

    /// Returns an iterator over effects for which the [AbilityId] ability has
    /// been applied to the provided target entity this turn.
    pub fn active_effects(
        &self,
        ability_id: AbilityId,
        target: EntityId,
    ) -> impl Iterator<Item = EffectId> + '_ {
        let Some(effects) = self.effects.get(&target) else {
            return Either::Right(iter::empty());
        };

        Either::Left(
            effects.iter().filter(move |e| e.ability_id == ability_id).map(|e| e.effect_id),
        )
    }

    /// Returns & removes the list of control-changing effects to automatically
    /// clean up at end of turn
    pub fn take_control_changing_effects(&mut self) -> Vec<(EffectId, CardId)> {
        self.control_changing_effects.take().unwrap_or_default()
    }

    /// Adds a control-changing effect to automatically clean up at end of turn.
    pub fn add_control_changing_effect(&mut self, effect_id: EffectId, card_id: CardId) {
        self.control_changing_effects.get_or_insert_with(Vec::new).push((effect_id, card_id));
    }

    /// Marks a permanent as having lost all abilities this turn.
    pub fn set_lost_all_abilities(&mut self, id: PermanentId) {
        self.lost_all_abilities.insert(id);
    }

    /// Returns true if a permanent has lost all abilities this turn.
    pub fn has_lost_all_abilities(&self, permanent_id: PermanentId) -> bool {
        self.lost_all_abilities.contains(&permanent_id)
    }
}
