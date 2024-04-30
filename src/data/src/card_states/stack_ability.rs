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
use slotmap::new_key_type;

use crate::card_states::zone_object::ZoneObjectTrait;
use crate::core::numerics::Timestamp;
use crate::core::primitives::{
    CardId, HasCardId, HasController, HasObjectId, HasOwner, HasTimestamp, ObjectId, PlayerName,
};

new_key_type! {
    /// Identifies an ability on the stack
    pub struct StackAbilityId;
}

/// Represents the state of a triggered or activated ability while it is on the
/// stack.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StackAbility {
    /// Object ID for this ability
    pub object_id: ObjectId,

    /// Card which created this ability.
    pub source: CardId,

    /// The player who this ability belongs to.
    pub owner: PlayerName,

    /// The player who can currently make decisions about this ability.
    pub controller: PlayerName,

    /// Timestamp at which this object arrived on the stack
    pub timestamp: Timestamp,

    /// Targets for this ability, selected when it is placed on the stack.
    pub targets: Vec<ObjectId>,
}

impl HasObjectId for StackAbility {
    fn object_id(&self) -> ObjectId {
        self.object_id
    }
}

impl HasCardId for StackAbility {
    fn card_id(&self) -> CardId {
        self.source
    }
}

impl HasOwner for StackAbility {
    fn owner(&self) -> PlayerName {
        self.owner
    }
}

impl HasController for StackAbility {
    fn controller(&self) -> PlayerName {
        self.controller
    }
}

impl HasTimestamp for StackAbility {
    fn timestamp(&self) -> Timestamp {
        self.timestamp
    }
}

impl ZoneObjectTrait for StackAbility {
    fn targets(&self) -> &[ObjectId] {
        &self.targets
    }
}
