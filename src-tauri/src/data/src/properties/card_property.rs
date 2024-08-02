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
use primitives::game_primitives::{HasSource, Source, Timestamp};
use utils::outcome;
use utils::outcome::Outcome;

use crate::core::ability_scope::AbilityScope;
use crate::core::layer::{EffectSortingKey, Layer};
use crate::core::modifier_data::ModifierMode;
use crate::core::rule_type::RuleType;
use crate::events::event_context::EventContext;
use crate::game_states::game_state::GameState;
use crate::properties::card_modifier::CardModifier;
use crate::properties::duration::Duration;
use crate::properties::flag::Flag;
use crate::properties::property_value::{ChangeText, EnumSets, Ints, PropertyValue};

/// Represents a permanent card losing all its current abilities as of a given
/// [Timestamp].
#[derive(Clone)]
pub struct LostAllAbilities {
    pub timestamp: Timestamp,
    pub duration: Duration,
}

#[derive(Clone)]
pub struct CardProperty<TModifier> {
    modifiers: Vec<CardModifier<TModifier>>,
}

impl<TModifier: PropertyValue> Default for CardProperty<TModifier> {
    fn default() -> Self {
        Self { modifiers: vec![] }
    }
}

impl<TModifier: PropertyValue> CardProperty<TModifier> {
    pub fn add(&mut self, modifier: CardModifier<TModifier>) {
        self.modifiers.push(modifier);
    }

    pub fn add_ability(&mut self, scope: AbilityScope, effect: TModifier) {
        self.add(CardModifier {
            source: Source::Game,
            duration: Duration::Continuous,
            rule_type: RuleType::Ability(scope.ability_id.card_id),
            effect,
        });
    }

    /// Applies an effect modifier to this card for a given [Duration].
    pub fn add_effect(
        &mut self,
        source: impl HasSource,
        duration: Duration,
        modifier: TModifier,
    ) -> Outcome {
        self.add(CardModifier {
            source: source.source(),
            duration,
            rule_type: RuleType::Effect,
            effect: modifier,
        });
        outcome::OK
    }

    /// Adds a modifier to this card with a given [ModifierMode].
    pub fn add_with_mode(&mut self, mode: ModifierMode, modifier: TModifier) -> Outcome {
        match mode {
            ModifierMode::PrintedAbility(scope) => self.add_ability(scope, modifier),
            ModifierMode::Effect(context, _, duration) => {
                self.add_effect(context, duration, modifier);
            }
        }
        outcome::OK
    }
}

impl<T: EnumSetType> CardProperty<EnumSets<T>> {
    #[must_use]
    pub fn query(&self, game: &GameState, _: Source, current: EnumSet<T>) -> EnumSet<T> {
        let mut largest_key = EffectSortingKey::default();
        let mut result = current;
        for modifier in &self.modifiers {
            if !modifier.active(game) {
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

impl CardProperty<Flag<()>> {
    #[must_use]
    pub fn query(&self, game: &GameState, source: Source, current: bool) -> Option<bool> {
        self.query_with(game, source, &(), current)
    }
}

impl<TArg> CardProperty<Flag<TArg>> {
    #[must_use]
    pub fn query_with(
        &self,
        game: &GameState,
        _: Source,
        arg: &TArg,
        current: bool,
    ) -> Option<bool> {
        let mut largest_key = EffectSortingKey::default();
        let mut result = current;
        let mut and = true;
        let mut or = false;
        for modifier in &self.modifiers {
            if !modifier.active(game) {
                continue;
            }

            match &modifier.effect {
                Flag::Overwrite(key, value) if *key >= largest_key => {
                    result = *value;
                    largest_key = *key;
                }
                Flag::And(condition) => {
                    let value = condition.invoke(game, modifier.source, arg);
                    and &= value;
                }
                Flag::Or(condition) => {
                    let value = condition.invoke(game, modifier.source, arg);
                    or |= value;
                }
                _ => {}
            };
        }

        Some((result || or) && and)
    }
}

impl<T: Default + Copy + Add<Output = T>> CardProperty<Ints<T>> {
    #[must_use]
    pub fn query(&self, game: &GameState, _: Source, current: T) -> T {
        let mut largest_key = EffectSortingKey::default();
        let mut result = current;
        let mut add = T::default();
        for modifier in &self.modifiers {
            if !modifier.active(game) {
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

impl<TResult: EnumSetType> CardProperty<ChangeText<TResult>> {
    #[must_use]
    pub fn query(&self, game: &GameState, _: Source, current: TResult) -> TResult {
        let mut largest_key = EffectSortingKey::default();
        let mut result = current;
        for modifier in &self.modifiers {
            if !modifier.active(game) {
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
