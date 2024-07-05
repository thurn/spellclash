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

use std::ops::Add;

use enumset::{EnumSet, EnumSetType};

use crate::card_states::zones::ToCardId;
use crate::core::function_types::CardPredicate;
use crate::core::primitives::{HasSource, Timestamp};
use crate::delegates::layer::{EffectSortingKey, Layer};
use crate::game_states::game_state::GameState;

/// Marker trait for the return value of queries
pub trait QueryValue {}

#[derive(Clone, Copy, Debug)]
pub enum Flag {
    Set(EffectSortingKey, bool),
    And(bool),
    Or(bool),
}

impl Flag {
    pub fn set(layer: Layer, timestamp: impl Into<Timestamp>, value: bool) -> Option<Flag> {
        Some(Self::Set(EffectSortingKey::new(layer, timestamp.into()), value))
    }

    pub fn and(value: bool) -> Option<Flag> {
        Some(Self::And(value))
    }

    pub fn or(value: bool) -> Option<Flag> {
        Some(Self::Or(value))
    }

    pub fn and_predicate<TId: ToCardId>(
        game: &GameState,
        source: impl HasSource,
        id: TId,
        predicate: impl CardPredicate<TId>,
    ) -> Option<Flag> {
        Some(Self::And(predicate(game, source.source(), id) == Some(true)))
    }
}

impl QueryValue for Flag {}

#[derive(Clone, Copy, Debug)]
pub enum Ints<T: Default + Add<Output = T>> {
    Set(EffectSortingKey, T),
    Add(T),
}

impl<T: Default + Add<Output = T>> Ints<T> {
    pub fn set(layer: Layer, timestamp: impl Into<Timestamp>, value: T) -> Option<Ints<T>> {
        Some(Self::Set(EffectSortingKey::new(layer, timestamp.into()), value))
    }

    pub fn add(value: T) -> Option<Ints<T>> {
        Some(Self::Add(value))
    }
}

impl<T: Default + Add<Output = T>> QueryValue for Ints<T> {}

#[derive(Clone, Copy, Debug)]
pub enum EnumSets<T: EnumSetType> {
    Set(EffectSortingKey, EnumSet<T>),
    Replace(EffectSortingKey, T, T),
}

impl<T: EnumSetType> EnumSets<T> {
    pub fn set(
        layer: Layer,
        timestamp: impl Into<Timestamp>,
        value: EnumSet<T>,
    ) -> Option<EnumSets<T>> {
        Some(Self::Set(EffectSortingKey::new(layer, timestamp.into()), value))
    }

    pub fn replace(
        layer: Layer,
        timestamp: impl Into<Timestamp>,
        old: T,
        new: T,
    ) -> Option<EnumSets<T>> {
        Some(Self::Replace(EffectSortingKey::new(layer, timestamp.into()), old, new))
    }
}

impl<T: EnumSetType> QueryValue for EnumSets<T> {}

#[derive(Clone, Copy, Debug)]
pub enum ChangeText<T: EnumSetType> {
    Replace(Timestamp, T, T),
}

impl<T: EnumSetType> ChangeText<T> {
    pub fn replace(timestamp: impl Into<Timestamp>, old: T, new: T) -> Option<ChangeText<T>> {
        Some(Self::Replace(timestamp.into(), old, new))
    }
}

impl<T: EnumSetType> QueryValue for ChangeText<T> {}
