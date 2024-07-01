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

use dyn_clone::DynClone;
use enumset::EnumSet;

use crate::card_states::zones::{ToCardId, ZoneQueries};
use crate::core::primitives::{AbilityId, EntityId, Zone};
use crate::delegates::delegate_type::DelegateType;
use crate::delegates::flag::Flag;
use crate::delegates::scope::Scope;
use crate::delegates::stores_delegates::StoresDelegates;
use crate::game_states::game_state::GameState;

/// Wrapper around query functions to enable closures to be cloned.
trait QueryFnWrapper<TArg, TResult>: DynClone + Send {
    fn invoke(&self, data: &GameState, scope: Scope, arg: &TArg, result: TResult) -> TResult;
}

dyn_clone::clone_trait_object!(<TArg, TResult> QueryFnWrapper<TArg, TResult>);

impl<TArg, TResult, F> QueryFnWrapper<TArg, TResult> for F
where
    F: Fn(&GameState, Scope, &TArg, TResult) -> TResult + Copy + Send,
{
    fn invoke(&self, data: &GameState, scope: Scope, arg: &TArg, result: TResult) -> TResult {
        self(data, scope, arg, result)
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
pub struct CardDelegateList<TArg: ToCardId, TResult> {
    current: Vec<DelegateBuilder<TArg, TResult>>,
    delegates: Vec<StoredQueryDelegate<TArg, TResult>>,
}

impl<TArg: ToCardId, TResult> CardDelegateList<TArg, TResult> {
    /// Adds a new query transformation which only applies to the card which
    /// owns this delegate.
    ///
    /// This adds a delegate with [DelegateType::Ability], i.e. the delegate
    /// will be automatically disabled if the owning card loses its abilities.
    pub fn this(
        &mut self,
        value: impl Fn(&GameState, Scope, &TArg, TResult) -> TResult + Copy + Send + Sync + 'static,
    ) {
        self.current.push(DelegateBuilder {
            delegate_type: DelegateType::Ability,
            execution_type: CardDelegateExecution::This,
            query: Box::new(value),
        });
    }

    /// Adds a new query transformation which applies to *any* query of this
    /// type.
    ///
    /// This adds a delegate with [DelegateType::Ability], i.e. the delegate
    /// will be automatically disabled if the owning card loses its abilities.
    pub fn any(
        &mut self,
        value: impl Fn(&GameState, Scope, &TArg, TResult) -> TResult + Copy + Send + Sync + 'static,
    ) {
        self.current.push(DelegateBuilder {
            delegate_type: DelegateType::Ability,
            execution_type: CardDelegateExecution::This,
            query: Box::new(value),
        });
    }

    /// Adds a new query transformation with the given [DelegateType] and
    /// [CardDelegateExecution].
    pub fn add_delegate(
        &mut self,
        delegate_type: DelegateType,
        execution_type: CardDelegateExecution,
        value: impl Fn(&GameState, Scope, &TArg, TResult) -> TResult + Copy + Send + Sync + 'static,
    ) {
        self.current.push(DelegateBuilder {
            delegate_type,
            execution_type,
            query: Box::new(value),
        });
    }

    #[must_use]
    pub fn query(&self, game: &GameState, arg: &TArg, current: TResult) -> TResult {
        let mut result = current;
        for stored in &self.delegates {
            let Some(card) = game.card(stored.ability_id.card_id) else {
                continue;
            };

            if stored.execution_type == CardDelegateExecution::This
                && card.id != stored.ability_id.card_id
            {
                continue;
            }

            if !stored.zones.contains(card.zone) {
                continue;
            }

            let Some(scope) = game.create_scope(stored.ability_id) else {
                continue;
            };

            if stored.delegate_type == DelegateType::Ability
                && card.permanent_id().map_or(false, |id| game.has_lost_all_abilities(id))
            {
                continue;
            }

            result = stored.query_fn.invoke(game, scope, arg, result);
        }
        result
    }

    /// True if no delegates have been defined for this list.
    pub fn is_empty(&self) -> bool {
        self.delegates.is_empty()
    }
}

impl<TArg: ToCardId> CardDelegateList<TArg, Flag> {
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
        current: Flag,
    ) -> Flag {
        if self.is_empty() {
            current
        } else {
            Flag::new(iterator.any(|arg| self.query(game, &arg, current).value()))
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
        current: Flag,
    ) -> Flag {
        if self.is_empty() {
            current
        } else {
            Flag::new(iterator.all(|arg| self.query(game, &arg, current).value()))
        }
    }
}

impl<TArg: ToCardId, TResult> StoresDelegates for CardDelegateList<TArg, TResult> {
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

impl<TArg: ToCardId, TResult> Default for CardDelegateList<TArg, TResult> {
    fn default() -> Self {
        Self { current: vec![], delegates: vec![] }
    }
}
