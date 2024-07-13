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

use crate::card_states::zones::{HasZones, ToCardId};
use crate::core::primitives::{CardId, EntityId, PermanentId, PlayerName};
use crate::game_states::combat_state::{AttackTarget, AttackerId, BlockerId};

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct CanAttackTarget {
    pub attacker_id: AttackerId,
    pub target: AttackTarget,
}

impl ToCardId for CanAttackTarget {
    fn to_card_id(&self, zones: &impl HasZones) -> Option<CardId> {
        self.attacker_id.to_card_id(zones)
    }
}

#[derive(Debug, Clone, Copy)]
pub struct CanBeBlocked {
    pub attacker_id: AttackerId,
    pub target: AttackTarget,
    pub blocker_id: BlockerId,
}

impl ToCardId for CanBeBlocked {
    fn to_card_id(&self, zones: &impl HasZones) -> Option<CardId> {
        self.attacker_id.to_card_id(zones)
    }
}

#[derive(Debug, Clone, Copy)]
pub struct PermanentControllerChangedEvent {
    pub permanent_id: PermanentId,
    pub old_controller: PlayerName,
    pub new_controller: PlayerName,
}

#[derive(Debug, Clone, Copy)]
pub struct WillEnterBattlefieldEvent {
    pub card_id: CardId,

    /// Note that this is *not yet* the current [PermanentId] of this entity.
    pub future_permanent_id: PermanentId,
}
