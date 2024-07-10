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
use std::iter;

use either::Either;
use serde::{Deserialize, Serialize};
use serde_with::serde_as;

use crate::core::primitives::{AbilityId, CardId, EffectId, EntityId, PermanentId, Timestamp};
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
    effects: BTreeMap<EntityId, Vec<AbilityEffectId>>,

    /// List of control-changing effects to automatically clean up at end of
    /// turn.
    control_changing_effects: Option<Vec<(EffectId, CardId)>>,

    /// Permanents that have lost all abilities this turn as of a given
    /// [Timestamp].
    #[serde_as(as = "Vec<(_, _)>")]
    lost_all_abilities: BTreeMap<PermanentId, Timestamp>,
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

    /// Changes all effects for the [AbilityId] ability to apply to a new target
    /// entity.
    pub fn change_affected_entity_id(
        &mut self,
        ability_id: AbilityId,
        old: EntityId,
        new: EntityId,
    ) {
        if self.active_effects(ability_id, old).next().is_none() {
            return;
        }

        let active = self.active_effects(ability_id, old).collect::<Vec<_>>();
        if let Some(effects) = self.effects.get_mut(&old) {
            effects.retain(|e| e.ability_id != ability_id);
        }
        active.into_iter().for_each(|effect_id| {
            self.add_effect(ability_id, effect_id, new);
        });
    }

    /// Returns & removes the list of control-changing effects to automatically
    /// clean up at end of turn
    pub fn remove_control_changing_effects(&mut self) -> Vec<(EffectId, CardId)> {
        self.control_changing_effects.take().unwrap_or_default()
    }

    /// Adds a control-changing effect to automatically clean up at end of turn.
    pub fn add_control_changing_effect(&mut self, effect_id: EffectId, card_id: CardId) {
        self.control_changing_effects.get_or_insert_with(Vec::new).push((effect_id, card_id));
    }

    /// Marks a permanent as having lost all abilities this turn.
    pub fn set_lost_all_abilities(&mut self, id: PermanentId, timestamp: Timestamp) {
        self.lost_all_abilities.insert(id, timestamp);
    }

    /// Checks if a permanent has lost all abilities this turn, and returns the
    /// [Timestamp] at which all abilities were removed.
    pub fn has_lost_all_abilities(&self, permanent_id: PermanentId) -> Option<Timestamp> {
        self.lost_all_abilities.get(&permanent_id).copied()
    }
}
