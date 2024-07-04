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

use crate::card_states::zones::ToCardId;
use crate::core::function_types::CardPredicate;
use crate::core::primitives::{HasSource, Timestamp};
use crate::game_states::game_state::GameState;

/// Possible high-level types of game delegate
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum DelegateType {
    /// Ability delegate. Will not be invoked if a permanent loses abilities.
    Ability,

    /// Effect delegate. Will still be invoked if a permanent loses abilities.
    Effect,
}

/// The result of a query delegate function
#[derive(Debug, Clone, Copy)]
pub enum QueryValue<T> {
    Skip,
    Set(Timestamp, T),
    Add(T),
    And(bool),
}

impl<T> QueryValue<T> {
    /// Create a new query response which sets the result to the given value.
    pub fn set(timestamp: impl Into<Timestamp>, value: T) -> Self {
        Self::Set(timestamp.into(), value)
    }

    /// Create a new query response which adds the given value to the result.
    pub fn add(value: T) -> Self {
        Self::Add(value)
    }

    /// Create a new query response which performs a boolean 'and' operation on
    /// the given value
    pub fn and(value: bool) -> Self {
        Self::And(value)
    }

    /// Invokes a [CardPredicate] and passes the result to [Self::and].
    pub fn and_predicate<TId: ToCardId>(
        game: &GameState,
        source: impl HasSource,
        id: TId,
        predicate: impl CardPredicate<TId>,
    ) -> Self {
        Self::And(predicate(game, source.source(), id) == Some(true))
    }
}

impl QueryValue<bool> {
    pub fn set_if_true(timestamp: Timestamp, value: bool) -> Self {
        if value {
            Self::Set(timestamp, true)
        } else {
            Self::Skip
        }
    }
}
