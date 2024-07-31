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
use primitives::game_primitives::Timestamp;

use crate::delegates::layer::{EffectSortingKey, Layer};
/// Marker trait for the return value of queries
pub trait QueryValue {
    fn effect_sorting_key(&self) -> Option<EffectSortingKey>;
}

#[derive(Clone, Copy, Debug)]
pub enum Ints<T: Default + Add<Output = T>> {
    Set(EffectSortingKey, T),
    Add(T),
}

impl<T: Default + Add<Output = T>> Ints<T> {
    pub fn set(layer: Layer, timestamp: impl Into<Timestamp>, value: T) -> Ints<T> {
        Self::Set(EffectSortingKey::new(layer, timestamp.into()), value)
    }

    pub fn add(value: T) -> Ints<T> {
        Self::Add(value)
    }
}

impl<T: Default + Add<Output = T>> QueryValue for Ints<T> {
    fn effect_sorting_key(&self) -> Option<EffectSortingKey> {
        match self {
            Self::Set(key, _) => Some(*key),
            Self::Add(_) => None,
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub enum EnumSets<T: EnumSetType> {
    Set(EffectSortingKey, EnumSet<T>),
    Add(EffectSortingKey, EnumSet<T>),
    Replace(EffectSortingKey, T, T),
}

impl<T: EnumSetType> EnumSets<T> {
    pub fn set(
        layer: Layer,
        timestamp: impl Into<Timestamp>,
        value: impl Into<EnumSet<T>>,
    ) -> EnumSets<T> {
        Self::Set(EffectSortingKey::new(layer, timestamp.into()), value.into())
    }

    pub fn add(
        layer: Layer,
        timestamp: impl Into<Timestamp>,
        value: impl Into<EnumSet<T>>,
    ) -> EnumSets<T> {
        Self::Add(EffectSortingKey::new(layer, timestamp.into()), value.into())
    }

    pub fn replace(layer: Layer, timestamp: impl Into<Timestamp>, old: T, new: T) -> EnumSets<T> {
        Self::Replace(EffectSortingKey::new(layer, timestamp.into()), old, new)
    }
}

impl<T: EnumSetType> QueryValue for EnumSets<T> {
    fn effect_sorting_key(&self) -> Option<EffectSortingKey> {
        match self {
            Self::Set(key, _) => Some(*key),
            Self::Add(key, _) => Some(*key),
            Self::Replace(key, _, _) => Some(*key),
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub enum ChangeText<T: EnumSetType> {
    Replace(Timestamp, T, T),
}

impl<T: EnumSetType> ChangeText<T> {
    pub fn replace(timestamp: impl Into<Timestamp>, old: T, new: T) -> ChangeText<T> {
        Self::Replace(timestamp.into(), old, new)
    }
}

impl<T: EnumSetType> QueryValue for ChangeText<T> {
    fn effect_sorting_key(&self) -> Option<EffectSortingKey> {
        match self {
            Self::Replace(timestamp, _, _) => {
                Some(EffectSortingKey::new(Layer::TextChangingEffects, *timestamp))
            }
        }
    }
}
