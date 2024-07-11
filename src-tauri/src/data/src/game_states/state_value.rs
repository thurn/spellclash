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

use either::Either;
use serde::{Deserialize, Serialize};

use crate::core::primitives::{CardId, Color, EntityId, PermanentId, PlayerName};
use crate::printed_cards::card_subtypes::LandType;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum StateValue {
    PlayerName(PlayerName),
    CardId(CardId),
    PermanentId(PermanentId),
    EntityId(EntityId),
    Color(Color),
    LandSubtype(LandType),
    Pair(Box<(StateValue, StateValue)>),
    Either(Box<Either<StateValue, StateValue>>),
}

impl<T, U> From<(T, U)> for StateValue
where
    T: Into<StateValue>,
    U: Into<StateValue>,
{
    fn from(value: (T, U)) -> Self {
        Self::Pair(Box::new((value.0.into(), value.1.into())))
    }
}

impl<T, U> TryFrom<StateValue> for (T, U)
where
    T: TryFrom<StateValue, Error = ()>,
    U: TryFrom<StateValue, Error = ()>,
{
    type Error = ();

    fn try_from(value: StateValue) -> Result<Self, Self::Error> {
        match value {
            StateValue::Pair(pair) => {
                let (left, right) = *pair;
                Ok((T::try_from(left)?, U::try_from(right)?))
            }
            _ => Err(()),
        }
    }
}

impl<T, U> From<Either<T, U>> for StateValue
where
    T: Into<StateValue>,
    U: Into<StateValue>,
{
    fn from(value: Either<T, U>) -> Self {
        Self::Either(Box::new(match value {
            Either::Left(left) => Either::Left(left.into()),
            Either::Right(right) => Either::Right(right.into()),
        }))
    }
}

impl<T, U> TryFrom<StateValue> for Either<T, U>
where
    T: TryFrom<StateValue, Error = ()>,
    U: TryFrom<StateValue, Error = ()>,
{
    type Error = ();

    fn try_from(value: StateValue) -> Result<Self, Self::Error> {
        match value {
            StateValue::Either(either) => match *either {
                Either::Left(left) => Ok(Either::Left(T::try_from(left)?)),
                Either::Right(right) => Ok(Either::Right(U::try_from(right)?)),
            },
            _ => Err(()),
        }
    }
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

impl From<Color> for StateValue {
    fn from(value: Color) -> Self {
        Self::Color(value)
    }
}

impl TryFrom<StateValue> for Color {
    type Error = ();

    fn try_from(value: StateValue) -> Result<Self, Self::Error> {
        match value {
            StateValue::Color(color) => Ok(color),
            _ => Err(()),
        }
    }
}

impl From<LandType> for StateValue {
    fn from(value: LandType) -> Self {
        Self::LandSubtype(value)
    }
}

impl TryFrom<StateValue> for LandType {
    type Error = ();

    fn try_from(value: StateValue) -> Result<Self, Self::Error> {
        match value {
            StateValue::LandSubtype(subtype) => Ok(subtype),
            _ => Err(()),
        }
    }
}
