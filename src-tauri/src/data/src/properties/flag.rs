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

use crate::card_definitions::registry::{QueryFn, Registered, Registry};
use crate::card_states::zones::ToCardId;
use crate::core::function_types::CardPredicate;
use crate::core::primitives::{HasSource, Source, Timestamp};
use crate::delegates::layer::{EffectSortingKey, Layer};
use crate::delegates::query_value::QueryValue;
use crate::game_states::game_state::GameState;
use crate::properties::query_condition::QueryCondition;

#[derive(Clone, Serialize, Deserialize)]
pub enum Flag<TArg: 'static> {
    Overwrite(EffectSortingKey, bool),
    And(QueryCondition<TArg>),
    Or(QueryCondition<TArg>),
}

impl<TArg: 'static> Flag<TArg> {
    pub fn overwrite(layer: Layer, timestamp: impl Into<Timestamp>, value: bool) -> Flag<TArg> {
        Self::Overwrite(EffectSortingKey::new(layer, timestamp.into()), value)
    }

    pub fn and(condition: QueryCondition<TArg>) -> Flag<TArg> {
        Self::And(condition)
    }

    pub fn and_predicate(predicate: QueryFn<TArg, Option<bool>>) -> Flag<TArg> {
        Self::And(QueryCondition::Predicate(predicate))
    }

    pub fn or(condition: QueryCondition<TArg>) -> Flag<TArg> {
        Self::Or(condition)
    }

    pub fn or_predicate(predicate: QueryFn<TArg, Option<bool>>) -> Flag<TArg> {
        Self::Or(QueryCondition::Predicate(predicate))
    }
}

impl<TArg> QueryValue for Flag<TArg> {
    fn initialize(&mut self, registry: &Registry) {
        match self {
            Flag::And(condition) => condition.initialize(registry),
            Flag::Or(condition) => condition.initialize(registry),
            _ => {}
        }
    }

    fn effect_sorting_key(&self) -> Option<EffectSortingKey> {
        match self {
            Flag::Overwrite(key, _) => Some(*key),
            _ => None,
        }
    }
}
