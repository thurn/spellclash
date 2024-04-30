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

use crate::card_states::card_state::CardState;
use crate::card_states::stack_ability::{StackAbility, StackAbilityId};
use crate::core::numerics::Timestamp;
use crate::core::primitives::{
    CardId, HasCardId, HasController, HasObjectId, HasOwner, HasTimestamp, ObjectId, PlayerName,
};

/// Identifies an ability on the stack, card, copy of a card on the stack,
/// token, or emblem
pub enum ZoneObjectId {
    CardId(CardId),
    StackAbilityId(StackAbilityId),
}

/// Represents a reference to an object that can exist in a zone
pub enum ZoneObject<'a> {
    Card(&'a CardState),
    Ability(&'a StackAbility),
}

/// Properties exposed by [ZoneObject].
pub trait ZoneObjectTrait:
    HasObjectId + HasOwner + HasController + HasCardId + HasTimestamp
{
    /// Current targets for this stack object, if any
    fn targets(&self) -> &[ObjectId];
}

impl<'a> HasObjectId for ZoneObject<'a> {
    fn object_id(&self) -> ObjectId {
        match self {
            ZoneObject::Card(card) => card.object_id(),
            ZoneObject::Ability(ability) => ability.object_id(),
        }
    }
}

impl<'a> HasCardId for ZoneObject<'a> {
    fn card_id(&self) -> CardId {
        match self {
            ZoneObject::Card(card) => card.card_id(),
            ZoneObject::Ability(ability) => ability.card_id(),
        }
    }
}

impl<'a> HasOwner for ZoneObject<'a> {
    fn owner(&self) -> PlayerName {
        match self {
            ZoneObject::Card(card) => card.owner(),
            ZoneObject::Ability(ability) => ability.owner(),
        }
    }
}

impl<'a> HasController for ZoneObject<'a> {
    fn controller(&self) -> PlayerName {
        match self {
            ZoneObject::Card(card) => card.controller(),
            ZoneObject::Ability(ability) => ability.controller(),
        }
    }
}

impl<'a> HasTimestamp for ZoneObject<'a> {
    fn timestamp(&self) -> Timestamp {
        match self {
            ZoneObject::Card(card) => card.timestamp(),
            ZoneObject::Ability(ability) => ability.timestamp(),
        }
    }
}

/// Manual version of enum_dispatch crate
impl<'a> ZoneObjectTrait for ZoneObject<'a> {
    fn targets(&self) -> &[ObjectId] {
        match self {
            ZoneObject::Card(card) => card.targets(),
            ZoneObject::Ability(ability) => ability.targets(),
        }
    }
}
