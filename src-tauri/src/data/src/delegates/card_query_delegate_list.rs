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

use std::collections::HashMap;
use std::ops::Add;

use dyn_clone::DynClone;
use enumset::EnumSet;

use crate::card_states::zones::{ToCardId, ZoneQueries};
use crate::core::primitives::{AbilityId, EntityId, Timestamp, Zone};
use crate::delegates::delegate_data::{DelegateType, QueryValue};
use crate::delegates::scope::Scope;
use crate::delegates::stores_delegates::StoresDelegates;
use crate::game_states::game_state::GameState;

/// Wrapper around query functions to enable closures to be cloned.
trait QueryFnWrapper<TArg, TResult>: DynClone + Send {
    fn invoke(&self, data: &GameState, scope: Scope, arg: &TArg) -> QueryValue<TResult>;
}

dyn_clone::clone_trait_object!(<TArg, TResult> QueryFnWrapper<TArg, TResult>);

impl<TArg, TResult, F> QueryFnWrapper<TArg, TResult> for F
where
    F: Fn(&GameState, Scope, &TArg) -> QueryValue<TResult> + Copy + Send,
{
    fn invoke(&self, data: &GameState, scope: Scope, arg: &TArg) -> QueryValue<TResult> {
        self(data, scope, arg)
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
pub enum CardDelegateExecution {
    This,
    Any,
}

type BoxedQueryFn<TArg, TResult> = Box<dyn QueryFnWrapper<TArg, TResult>>;

#[derive(Clone)]
struct DelegateBuilder<TArg: ToCardId, TResult> {
    delegate_type: DelegateType,
    execution_type: CardDelegateExecution,
    query: BoxedQueryFn<TArg, TResult>,
}

#[derive(Clone)]
struct StoredQueryDelegate<TArg, TResult> {
    zones: EnumSet<Zone>,
    ability_id: AbilityId,
    delegate_type: DelegateType,
    execution_type: CardDelegateExecution,
    query_fn: BoxedQueryFn<TArg, TResult>,
}

#[derive(Clone)]
pub struct CardQueryDelegateList<TArg: ToCardId, TResult> {
    current: Vec<DelegateBuilder<TArg, TResult>>,
    delegates: Vec<StoredQueryDelegate<TArg, TResult>>,
}

impl<TArg: ToCardId, TResult> CardQueryDelegateList<TArg, TResult> {
    /// Adds a new query transformation which only applies to the card which
    /// owns this delegate.
    ///
    /// This adds a delegate with [DelegateType::Ability], i.e. the delegate
    /// will be automatically disabled if the owning card loses its abilities.
    pub fn this(
        &mut self,
        value: impl Fn(&GameState, Scope, &TArg) -> QueryValue<TResult> + Copy + Send + Sync + 'static,
    ) {
        self.add_delegate(DelegateType::Ability, CardDelegateExecution::This, value);
    }

    /// Adds a new query transformation which applies to *any* query of this
    /// type.
    ///
    /// This adds a delegate with [DelegateType::Ability], i.e. the delegate
    /// will be automatically disabled if the owning card loses its abilities.
    pub fn any(
        &mut self,
        value: impl Fn(&GameState, Scope, &TArg) -> QueryValue<TResult> + Copy + Send + Sync + 'static,
    ) {
        self.add_delegate(DelegateType::Ability, CardDelegateExecution::Any, value);
    }

    /// Adds a new query transformation with the given [DelegateType] and
    /// [CardDelegateExecution].
    pub fn add_delegate(
        &mut self,
        delegate_type: DelegateType,
        execution_type: CardDelegateExecution,
        value: impl Fn(&GameState, Scope, &TArg) -> QueryValue<TResult> + Copy + Send + Sync + 'static,
    ) {
        self.current.push(DelegateBuilder {
            delegate_type,
            execution_type,
            query: Box::new(value),
        });
    }

    #[must_use]
    pub fn query(&self, game: &GameState, arg: &TArg, current: TResult) -> TResult {
        let mut largest_timestamp = Timestamp(0);
        let mut result = current;
        for stored in &self.delegates {
            let Some(scope) = validate_scope(game, stored, &mut largest_timestamp) else {
                continue;
            };

            match stored.query_fn.invoke(game, scope, arg) {
                QueryValue::Set(timestamp, value) if timestamp >= largest_timestamp => {
                    result = value;
                    largest_timestamp = timestamp;
                }
                QueryValue::Add(_) => {
                    panic!("Query is not numeric")
                }
                QueryValue::And(_) => {
                    panic!("Query is not boolean")
                }
                _ => {}
            };
        }
        result
    }

    /// True if no delegates have been defined for this list.
    pub fn is_empty(&self) -> bool {
        self.delegates.is_empty()
    }
}

impl<TArg: ToCardId, TResult: Add<Output = TResult> + Default>
    CardQueryDelegateList<TArg, TResult>
{
    #[must_use]
    pub fn query_numeric(&self, game: &GameState, arg: &TArg, current: TResult) -> TResult {
        let mut largest_timestamp = Timestamp(0);
        let mut result = current;
        let mut add = TResult::default();
        for stored in &self.delegates {
            let Some(scope) = validate_scope(game, stored, &mut largest_timestamp) else {
                continue;
            };

            match stored.query_fn.invoke(game, scope, arg) {
                QueryValue::Set(timestamp, value) if timestamp >= largest_timestamp => {
                    result = value;
                    largest_timestamp = timestamp;
                }
                QueryValue::Add(value) => {
                    add = add + value;
                }
                QueryValue::And(_) => {
                    panic!("Query is not boolean")
                }
                _ => {}
            };
        }

        result + add
    }
}

impl<TArg: ToCardId> CardQueryDelegateList<TArg, bool> {
    /// Runs a boolean query to see if any item in the provided iterator matches
    /// a predicate. Returns `current` if no delegates are present in the map.
    ///
    /// Prefer using this function over directly calling `query` because it
    /// short-circuits for empty delegate lists and avoids invoking the
    /// iterator, which can be a significant performance win.
    pub fn query_any(
        &self,
        game: &GameState,
        mut iterator: impl Iterator<Item = TArg>,
        current: bool,
    ) -> bool {
        if self.is_empty() {
            current
        } else {
            iterator.any(|arg| self.query_boolean(game, &arg, current))
        }
    }

    /// Runs a boolean query to see if all items in the provided iterator
    /// match a predicate. Returns `current` if no delegates are present
    /// in the map.
    ///
    /// Prefer using this function over directly calling `query` because it
    /// short-circuits for empty delegate lists and avoids invoking the
    /// iterator, which can be a significant performance win.
    pub fn query_all(
        &self,
        game: &GameState,
        mut iterator: impl Iterator<Item = TArg>,
        current: bool,
    ) -> bool {
        if self.is_empty() {
            current
        } else {
            iterator.all(|arg| self.query_boolean(game, &arg, current))
        }
    }

    #[must_use]
    pub fn query_boolean(&self, game: &GameState, arg: &TArg, current: bool) -> bool {
        let mut largest_timestamp = Timestamp(0);
        let mut result = current;
        let mut and = true;
        for stored in &self.delegates {
            let Some(scope) = validate_scope(game, stored, &mut largest_timestamp) else {
                continue;
            };

            match stored.query_fn.invoke(game, scope, arg) {
                QueryValue::Set(timestamp, value) if timestamp >= largest_timestamp => {
                    result = value;
                    largest_timestamp = timestamp;
                }
                QueryValue::Add(_) => {
                    panic!("Query is not numeric")
                }
                QueryValue::And(value) => {
                    and &= value;
                }
                _ => {}
            };
        }

        result & and
    }
}

impl<TArg: ToCardId, TResult> StoresDelegates for CardQueryDelegateList<TArg, TResult> {
    fn apply_writes(&mut self, id: AbilityId, zones: EnumSet<Zone>) {
        for builder in self.current.drain(..) {
            self.delegates.push(StoredQueryDelegate {
                zones,
                ability_id: id,
                delegate_type: builder.delegate_type,
                execution_type: builder.execution_type,
                query_fn: builder.query,
            });
        }
    }
}

impl<TArg: ToCardId, TResult> Default for CardQueryDelegateList<TArg, TResult> {
    fn default() -> Self {
        Self { current: vec![], delegates: vec![] }
    }
}

fn validate_scope<TArg: ToCardId, TResult>(
    game: &GameState,
    stored: &StoredQueryDelegate<TArg, TResult>,
    largest_timestamp: &mut Timestamp,
) -> Option<Scope> {
    let card = game.card(stored.ability_id.card_id)?;

    if stored.execution_type == CardDelegateExecution::This && card.id != stored.ability_id.card_id
    {
        return None;
    }

    if !stored.zones.contains(card.zone) {
        return None;
    }

    let scope = game.create_scope(stored.ability_id)?;

    if stored.delegate_type == DelegateType::Ability {
        if let Some(timestamp) = card.permanent_id().and_then(|id| game.has_lost_all_abilities(id))
        {
            if timestamp > *largest_timestamp {
                // Set the largest timestamp to the time at which this permanent lost all
                // abilities, thus ignoring any earlier effects which set a value.
                *largest_timestamp = timestamp;
            }
        }
    }

    Some(scope)
}