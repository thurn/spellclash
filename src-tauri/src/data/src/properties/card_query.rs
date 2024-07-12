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

use std::marker::PhantomData;
use std::ops::Add;

use enumset::{EnumSet, EnumSetType};

use crate::card_states::zones::ToCardId;
use crate::core::primitives::{AbilityId, CardId, PermanentId, Source, Timestamp};
use crate::delegates::delegate_type::DelegateType;
use crate::delegates::layer::{EffectSortingKey, Layer};
use crate::delegates::query_value::{ChangeText, EnumSets, Ints, QueryValue};
use crate::game_states::game_state::GameState;
use crate::properties::card_modifier::CardModifier;
use crate::properties::duration::Duration;
use crate::properties::flag::Flag;

pub type CardProperty<TModifier> = CardArgumentProperty<(), TModifier>;

/// Represents a permanent card losing all its current abilities as of a given
/// [Timestamp].
#[derive(Clone, Copy)]
pub struct LostAllAbilities {
    pub timestamp: Timestamp,
    pub permanent_id: PermanentId,
}

#[derive(Clone)]
pub struct CardArgumentProperty<TArg, TModifier: QueryValue> {
    modifiers: Vec<CardModifier<TModifier>>,
    lost_all_abilities: Option<LostAllAbilities>,
    phantom_data: PhantomData<TArg>,
}

impl<TArg, TModifier: QueryValue> CardArgumentProperty<TArg, TModifier> {
    pub fn add_static(&mut self, effect: TModifier) {
        self.modifiers.push(CardModifier {
            source: Source::Game,
            duration: Duration::Continuous,
            delegate_type: DelegateType::Ability,
            effect,
        });
    }

    pub fn add(&mut self, modifier: CardModifier<TModifier>) {
        self.modifiers.push(modifier);
    }

    pub fn set_lost_all_abilities(&mut self, lost_all_abilities: LostAllAbilities) {
        self.lost_all_abilities = Some(lost_all_abilities);
    }
}

impl<TArg, TModifier: QueryValue> Default for CardArgumentProperty<TArg, TModifier> {
    fn default() -> Self {
        Self { modifiers: vec![], lost_all_abilities: None, phantom_data: PhantomData }
    }
}

impl<TResult: EnumSetType> CardArgumentProperty<(), EnumSets<TResult>> {
    #[must_use]
    pub fn query(
        &self,
        game: &GameState,
        source: Source,
        current: EnumSet<TResult>,
    ) -> EnumSet<TResult> {
        self.query_with(game, source, &(), current)
    }
}

impl<TArg, TResult: EnumSetType> CardArgumentProperty<TArg, EnumSets<TResult>> {
    #[must_use]
    pub fn query_with(
        &self,
        game: &GameState,
        _: Source,
        arg: &TArg,
        current: EnumSet<TResult>,
    ) -> EnumSet<TResult> {
        let mut largest_key = EffectSortingKey::default();
        let mut result = current;
        for modifier in &self.modifiers {
            if !modifier.active(game, &self.lost_all_abilities) {
                continue;
            }

            match modifier.effect {
                EnumSets::Set(key, value) if key >= largest_key => {
                    result = value;
                    largest_key = key;
                }
                EnumSets::Add(_, value) => {
                    result.insert_all(value);
                }
                EnumSets::Replace(key, old, new) if result.contains(old) && key >= largest_key => {
                    result.remove(old);
                    result.insert(new);
                    largest_key = key;
                }
                _ => {}
            };
        }

        result
    }
}

impl CardArgumentProperty<(), Flag<()>> {
    #[must_use]
    pub fn query(&self, game: &GameState, source: Source, current: bool) -> bool {
        self.query_with(game, source, &(), current)
    }
}

impl<TArg> CardArgumentProperty<TArg, Flag<TArg>> {
    #[must_use]
    pub fn query_with(&self, game: &GameState, _: Source, arg: &TArg, current: bool) -> bool {
        let mut largest_key = EffectSortingKey::default();
        let mut result = current;
        let mut and = true;
        let mut or = false;
        for modifier in &self.modifiers {
            if !modifier.active(game, &self.lost_all_abilities) {
                continue;
            }

            match &modifier.effect {
                Flag::Overwrite(key, value) if *key >= largest_key => {
                    result = *value;
                    largest_key = *key;
                }
                Flag::And(condition) => {
                    let value = condition.passes(game, modifier.source, arg);
                    and &= value;
                }
                Flag::Or(condition) => {
                    let value = condition.passes(game, modifier.source, arg);
                    or |= value;
                }
                _ => {}
            };
        }

        (result || or) && and
    }
}

impl<T: Default + Copy + Add<Output = T>> CardArgumentProperty<(), Ints<T>> {
    #[must_use]
    pub fn query(&self, game: &GameState, source: Source, current: T) -> T {
        self.query_with(game, source, &(), current)
    }
}

impl<TArg, T: Default + Copy + Add<Output = T>> CardArgumentProperty<TArg, Ints<T>> {
    #[must_use]
    pub fn query_with(&self, game: &GameState, _: Source, arg: &TArg, current: T) -> T {
        let mut largest_key = EffectSortingKey::default();
        let mut result = current;
        let mut add = T::default();
        for modifier in &self.modifiers {
            if !modifier.active(game, &self.lost_all_abilities) {
                continue;
            }

            match modifier.effect {
                Ints::Set(key, value) if key >= largest_key => {
                    result = value;
                    largest_key = key;
                }
                Ints::Add(to_add) => {
                    add = add + to_add;
                }
                _ => {}
            };
        }

        result + add
    }
}

impl<TResult: EnumSetType> CardArgumentProperty<(), ChangeText<TResult>> {
    #[must_use]
    pub fn query(&self, game: &GameState, source: Source, current: TResult) -> TResult {
        self.query_with(game, source, &(), current)
    }
}

impl<TArg, TResult: EnumSetType> CardArgumentProperty<TArg, ChangeText<TResult>> {
    #[must_use]
    pub fn query_with(&self, game: &GameState, _: Source, arg: &TArg, current: TResult) -> TResult {
        let mut largest_key = EffectSortingKey::default();
        let mut result = current;
        for modifier in &self.modifiers {
            if !modifier.active(game, &self.lost_all_abilities) {
                continue;
            }

            let ChangeText::Replace(timestamp, old, new) = modifier.effect;
            let key = EffectSortingKey::new(Layer::TextChangingEffects, timestamp);
            if old == current && key >= largest_key {
                result = new;
                largest_key = key;
            }
        }

        result
    }
}
