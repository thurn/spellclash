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

use enumset::EnumSet;

use crate::core::primitives::{AbilityId, EntityId, HasCardId, Zone};
use crate::delegates::has_delegates::HasDelegates;
use crate::delegates::scope::Scope;
use crate::delegates::stores_delegates::StoresDelegates;

pub type QueryFn<TData, TArg, TResult> =
    fn(&TData, &<TData as HasDelegates>::ScopeType, &TArg, TResult) -> TResult;

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
enum CardDelegateExecution {
    This,
    Any,
}

#[derive(Debug, Clone)]
struct StoredQueryDelegate<TData: HasDelegates, TArg, TResult> {
    zones: EnumSet<Zone>,
    ability_id: AbilityId,
    execution_type: CardDelegateExecution,
    query_fn: QueryFn<TData, TArg, TResult>,
}

#[derive(Debug, Clone)]
pub struct CardDelegateList<TData: HasDelegates, TArg: HasCardId, TResult> {
    current: Vec<(CardDelegateExecution, QueryFn<TData, TArg, TResult>)>,
    delegates: Vec<StoredQueryDelegate<TData, TArg, TResult>>,
}

impl<TData: HasDelegates, TArg: HasCardId, TResult> CardDelegateList<TData, TArg, TResult> {
    pub fn this(&mut self, value: QueryFn<TData, TArg, TResult>) {
        self.current.push((CardDelegateExecution::This, value));
    }

    pub fn any(&mut self, value: QueryFn<TData, TArg, TResult>) {
        self.current.push((CardDelegateExecution::Any, value));
    }

    pub fn query(&self, data: &TData, arg: &TArg, current: TResult) -> TResult {
        let mut result = current;
        for stored in &self.delegates {
            if stored.execution_type == CardDelegateExecution::This
                && arg.card_id() != stored.ability_id.card_id
            {
                continue;
            }

            if !stored.zones.contains(data.current_zone(stored.ability_id.card_id)) {
                continue;
            }

            let scope = data.create_scope(stored.ability_id);
            result = (stored.query_fn)(data, &scope, arg, result);
        }
        result
    }
}

impl<TData: HasDelegates, TArg: HasCardId, TResult> StoresDelegates
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

impl<TData: HasDelegates, TArg: HasCardId, TResult> Default
    for CardDelegateList<TData, TArg, TResult>
{
    fn default() -> Self {
        Self { current: vec![], delegates: vec![] }
    }
}
