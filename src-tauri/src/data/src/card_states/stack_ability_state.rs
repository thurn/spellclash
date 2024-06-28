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
use crate::card_states::zones::{HasZones, ToCardId};
use crate::core::primitives::{
    AbilityId, CardId, EffectId, EntityId, HasController, HasPlayerName, ObjectId, PlayerName,
    StackAbilityId,
};
use crate::delegates::scope::Scope;
use crate::game_states::game_state::GameState;

/// Represents the state of a triggered or activated ability which has triggered
/// or is on the stack
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StackAbilityState {
    /// ID of this ability on the stack.
    pub id: StackAbilityId,

    /// Identifies this ability within its parent card's oracle text.
    pub ability_id: AbilityId,

    /// Identifier for the ability within its zone
    pub object_id: ObjectId,

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

    /// Marks this as a delayed trigger on the stack created by the effect with
    /// the indicated [EffectId].
    pub delayed_trigger_effect_id: Option<EffectId>,
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
