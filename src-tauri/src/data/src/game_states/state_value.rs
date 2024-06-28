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

use crate::core::primitives::{CardId, EntityId, PermanentId, PlayerName};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum StateValue {
    PlayerName(PlayerName),
    CardId(CardId),
    PermanentId(PermanentId),
    EntityId(EntityId),
}

impl From<PlayerName> for StateValue {
    fn from(value: PlayerName) -> Self {
        Self::PlayerName(value)
    }
}

impl TryFrom<StateValue> for PlayerName {
    type Error = ();

    fn try_from(value: StateValue) -> Result<Self, Self::Error> {
        match value {
            StateValue::PlayerName(name) => Ok(name),
            _ => Err(()),
        }
    }
}

impl From<CardId> for StateValue {
    fn from(value: CardId) -> Self {
        Self::CardId(value)
    }
}

impl TryFrom<StateValue> for CardId {
    type Error = ();

    fn try_from(value: StateValue) -> Result<Self, Self::Error> {
        match value {
            StateValue::CardId(id) => Ok(id),
            _ => Err(()),
        }
    }
}

impl From<PermanentId> for StateValue {
    fn from(value: PermanentId) -> Self {
        Self::PermanentId(value)
    }
}

impl TryFrom<StateValue> for PermanentId {
    type Error = ();

    fn try_from(value: StateValue) -> Result<Self, Self::Error> {
        match value {
            StateValue::PermanentId(id) => Ok(id),
            _ => Err(()),
        }
    }
}

impl From<EntityId> for StateValue {
    fn from(value: EntityId) -> Self {
        Self::EntityId(value)
    }
}

impl TryFrom<StateValue> for EntityId {
    type Error = ();

    fn try_from(value: StateValue) -> Result<Self, Self::Error> {
        match value {
            StateValue::EntityId(id) => Ok(id),
            _ => Err(()),
        }
    }
}
