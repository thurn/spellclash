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
use crate::card_states::zones::Zones;
use crate::core::primitives::{
    AbilityId, CardId, EntityId, HasCardId, HasController, HasEntityId, HasPlayerName, PlayerName,
};
use crate::delegates::scope::Scope;

/// Represents the state of a triggered or activated ability which has triggered
/// or is on the stack
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StackAbilityState {
    /// ID of this ability.
    pub ability_id: AbilityId,

    /// Entity ID for this ability.
    ///
    /// Do not mutate this field directly, use the methods on the [Zones] struct
    /// instead.
    pub entity_id: EntityId,

    /// True if this ability has been placed on the stack.
    ///
    /// Activated abilities are created directly on the stack. Triggered
    /// abilities are moved to the stack the next time a player gains priority
    /// after they trigger.
    pub placed_on_stack: bool,

    /// The player who this ability belongs to, who initially created it.
    pub owner: PlayerName,

    /// The player who can currently make decisions about this ability.
    pub controller: PlayerName,

    /// Targets for this ability, selected when it is placed on the stack.
    pub targets: Vec<EntityId>,
}

impl HasCardId for StackAbilityState {
    fn card_id(&self) -> CardId {
        self.ability_id.card_id
    }
}

impl HasEntityId for StackAbilityState {
    fn entity_id(&self) -> EntityId {
        self.entity_id
    }
}

impl HasPlayerName for StackAbilityState {
    fn player_name(&self) -> PlayerName {
        self.owner
    }
}

impl HasController for StackAbilityState {
    fn controller(&self) -> PlayerName {
        self.controller
    }
}
