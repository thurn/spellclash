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
use crate::card_states::stack_ability::StackAbility;
use crate::core::numerics::Timestamp;
use crate::core::primitives::{
    CardId, HasCardId, HasController, HasObjectId, HasOwner, HasTimestamp, ObjectId, PlayerName,
};

/// Represents a reference to a card or ability on the stack.
pub enum StackObject<'a> {
    Card(&'a CardState),
    Ability(&'a StackAbility),
}

/// Properties exposed by [StackObject].
pub trait StackObjectTrait:
    HasObjectId + HasOwner + HasController + HasCardId + HasTimestamp
{
    /// Current targets for this stack object, if any
    fn targets(&self) -> &[ObjectId];
}

impl<'a> HasObjectId for StackObject<'a> {
    fn object_id(&self) -> ObjectId {
        match self {
            StackObject::Card(card) => card.object_id(),
            StackObject::Ability(ability) => ability.object_id(),
        }
    }
}

impl<'a> HasCardId for StackObject<'a> {
    fn card_id(&self) -> CardId {
        match self {
            StackObject::Card(card) => card.card_id(),
            StackObject::Ability(ability) => ability.card_id(),
        }
    }
}

impl<'a> HasOwner for StackObject<'a> {
    fn owner(&self) -> PlayerName {
        match self {
            StackObject::Card(card) => card.owner(),
            StackObject::Ability(ability) => ability.owner(),
        }
    }
}

impl<'a> HasController for StackObject<'a> {
    fn controller(&self) -> PlayerName {
        match self {
            StackObject::Card(card) => card.controller(),
            StackObject::Ability(ability) => ability.controller(),
        }
    }
}

impl<'a> HasTimestamp for StackObject<'a> {
    fn timestamp(&self) -> Timestamp {
        match self {
            StackObject::Card(card) => card.timestamp(),
            StackObject::Ability(ability) => ability.timestamp(),
        }
    }
}

/// Manual version of enum_dispatch crate
impl<'a> StackObjectTrait for StackObject<'a> {
    fn targets(&self) -> &[ObjectId] {
        match self {
            StackObject::Card(card) => card.targets(),
            StackObject::Ability(ability) => ability.targets(),
        }
    }
}
