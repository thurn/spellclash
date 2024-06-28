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

use crate::card_states::zones::ToCardId;
use crate::core::primitives::{AbilityId, EntityId, Zone};
use crate::delegates::flag::Flag;
use crate::delegates::has_delegates::HasDelegates;
use crate::delegates::scope::Scope;
use crate::delegates::stores_delegates::StoresDelegates;

/// Wrapper around query functions to enable closures to be cloned.
trait QueryFnWrapper<TData: HasDelegates, TArg, TResult>: DynClone + Send {
    fn invoke(&self, data: &TData, scope: TData::ScopeType, arg: &TArg, result: TResult)
        -> TResult;
}

dyn_clone::clone_trait_object!(<TData: HasDelegates, TArg, TResult> QueryFnWrapper<TData, TArg, TResult>);

impl<TData: HasDelegates, TArg, TResult, F> QueryFnWrapper<TData, TArg, TResult> for F
where
    F: Fn(&TData, TData::ScopeType, &TArg, TResult) -> TResult + Copy + Send,
{
    fn invoke(
        &self,
        data: &TData,
        scope: TData::ScopeType,
        arg: &TArg,
        result: TResult,
    ) -> TResult {
        self(data, scope, arg, result)
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
enum CardDelegateExecution {
    This,
    Any,
}

type BoxedQueryFn<TData, TArg, TResult> = Box<dyn QueryFnWrapper<TData, TArg, TResult>>;

#[derive(Clone)]
struct StoredQueryDelegate<TData: HasDelegates, TArg, TResult> {
    zones: EnumSet<Zone>,
    ability_id: AbilityId,
    execution_type: CardDelegateExecution,
    query_fn: BoxedQueryFn<TData, TArg, TResult>,
}

#[derive(Clone)]
pub struct CardDelegateList<TData: HasDelegates, TArg: ToCardId, TResult> {
    current: Vec<(CardDelegateExecution, BoxedQueryFn<TData, TArg, TResult>)>,
    delegates: Vec<StoredQueryDelegate<TData, TArg, TResult>>,
}

impl<TData: HasDelegates, TArg: ToCardId, TResult> CardDelegateList<TData, TArg, TResult> {
    pub fn this(
        &mut self,
        value: impl Fn(&TData, <TData as HasDelegates>::ScopeType, &TArg, TResult) -> TResult
            + Copy
            + Send
            + Sync
            + 'static,
    ) {
        self.current.push((CardDelegateExecution::This, Box::new(value)));
    }

    pub fn any(
        &mut self,
        value: impl Fn(&TData, <TData as HasDelegates>::ScopeType, &TArg, TResult) -> TResult
            + Copy
            + Send
            + Sync
            + 'static,
    ) {
        self.current.push((CardDelegateExecution::Any, Box::new(value)));
    }

    #[must_use]
    pub fn query(&self, data: &TData, arg: &TArg, current: TResult) -> TResult {
        let mut result = current;
        for stored in &self.delegates {
            if stored.execution_type == CardDelegateExecution::This
                && arg.to_card_id(data.game_state()) != Some(stored.ability_id.card_id)
            {
                continue;
            }

            let Some(zone) = data.current_zone(stored.ability_id.card_id) else {
                continue;
            };

            if !stored.zones.contains(zone) {
                continue;
            }

            let Some(scope) = data.create_scope(stored.ability_id) else {
                continue;
            };

            result = stored.query_fn.invoke(data, scope, arg, result);
        }
        result
    }

    /// True if no delegates have been defined for this list.
    pub fn is_empty(&self) -> bool {
        self.delegates.is_empty()
    }
}

impl<TData: HasDelegates, TArg: ToCardId> CardDelegateList<TData, TArg, Flag> {
    /// Runs a boolean query to see if any item in the provided iterator matches
    /// a predicate. Returns `current` if no delegates are present in the map.
    ///
    /// Prefer using this function over directly calling `query` because it
    /// short-circuits for empty delegate lists and avoids invoking the
    /// iterator, which can be a significant performance win.
    pub fn query_any(
        &self,
        data: &TData,
        mut iterator: impl Iterator<Item = TArg>,
        current: Flag,
    ) -> Flag {
        if self.is_empty() {
            current
        } else {
            Flag::from_bool(iterator.any(|arg| self.query(data, &arg, current).value()))
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
        data: &TData,
        mut iterator: impl Iterator<Item = TArg>,
        current: Flag,
    ) -> Flag {
        if self.is_empty() {
            current
        } else {
            Flag::from_bool(iterator.all(|arg| self.query(data, &arg, current).value()))
        }
    }
}

impl<TData: HasDelegates, TArg: ToCardId, TResult> StoresDelegates
    for CardDelegateList<TData, TArg, TResult>
{
    fn apply_writes(&mut self, id: AbilityId, zones: EnumSet<Zone>) {
        for (execution_type, function) in self.current.drain(..) {
            self.delegates.push(StoredQueryDelegate {
                zones,
                ability_id: id,
                execution_type,
                query_fn: function,
            });
        }
    }
}

impl<TData: HasDelegates, TArg: ToCardId, TResult> Default
    for CardDelegateList<TData, TArg, TResult>
{
    fn default() -> Self {
        Self { current: vec![], delegates: vec![] }
    }
}
