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
use utils::outcome;
use utils::outcome::Outcome;

use crate::core::primitives::{AbilityId, EntityId, Zone};
use crate::delegates::flag::Flag;
use crate::delegates::has_delegates::HasDelegates;
use crate::delegates::scope::DelegateScope;
use crate::delegates::stores_delegates::StoresDelegates;
use crate::game_states::game_state::GameState;

/// Wrapper around event functions to enable closures to be cloned.
pub trait EventFnWrapper<TData: HasDelegates, TArg>: DynClone + Send {
    fn invoke(&self, data: &mut TData, scope: TData::ScopeType, arg: &TArg) -> Outcome;
}

dyn_clone::clone_trait_object!(<TData: HasDelegates, TArg> EventFnWrapper<TData, TArg>);

impl<TData: HasDelegates, TArg, F> EventFnWrapper<TData, TArg> for F
where
    F: Fn(&mut TData, TData::ScopeType, &TArg) -> Outcome + Copy + Send,
{
    fn invoke(&self, data: &mut TData, scope: TData::ScopeType, arg: &TArg) -> Outcome {
        self(data, scope, arg)
    }
}

pub type BoxedEventFn<TData, TArg> = Box<dyn EventFnWrapper<TData, TArg>>;

#[derive(Clone)]
pub struct StoredEventDelegate<TData: HasDelegates, TArg> {
    zones: EnumSet<Zone>,
    ability_id: AbilityId,
    event_fn: BoxedEventFn<TData, TArg>,
}

#[derive(Clone)]
pub struct EventDelegateList<TData: HasDelegates, TArg> {
    current: Vec<BoxedEventFn<TData, TArg>>,
    delegates: Vec<StoredEventDelegate<TData, TArg>>,
}

impl<TData: HasDelegates, TArg> EventDelegateList<TData, TArg> {
    pub fn whenever(
        &mut self,
        value: impl Fn(&mut TData, <TData as HasDelegates>::ScopeType, &TArg) -> Outcome
            + Copy
            + Send
            + Sync
            + 'static,
    ) {
        self.current.push(Box::new(value));
    }

    pub fn invoke_with<'a>(
        &self,
        data: &TData,
        arg: &'a TArg,
    ) -> EventDelegateInvoker<'a, TData, TArg> {
        let mut result: Vec<StoredEventDelegate<TData, TArg>> = vec![];
        for stored in &self.delegates {
            if !stored.zones.contains(data.current_zone(stored.ability_id.card_id)) {
                continue;
            }
            result.push(StoredEventDelegate {
                zones: stored.zones,
                ability_id: stored.ability_id,
                event_fn: stored.event_fn.clone(),
            });
        }
        EventDelegateInvoker::new(result, arg)
    }

    /// True if no delegates have been defined for this list.
    pub fn is_empty(&self) -> bool {
        self.delegates.is_empty()
    }
}

impl<TData: HasDelegates, TArg> StoresDelegates for EventDelegateList<TData, TArg> {
    fn apply_writes(&mut self, id: AbilityId, zones: EnumSet<Zone>) {
        for function in self.current.drain(..) {
            self.delegates.push(StoredEventDelegate { zones, ability_id: id, event_fn: function });
        }
    }
}

impl<TData: HasDelegates, TArg> Default for EventDelegateList<TData, TArg> {
    fn default() -> Self {
        Self { current: vec![], delegates: vec![] }
    }
}

pub struct EventDelegateInvoker<'a, TData: HasDelegates, TArg> {
    delegates: Vec<StoredEventDelegate<TData, TArg>>,
    arg: &'a TArg,
}

impl<'a, TData: HasDelegates, TArg> EventDelegateInvoker<'a, TData, TArg> {
    pub fn new(delegates: Vec<StoredEventDelegate<TData, TArg>>, arg: &'a TArg) -> Self {
        Self { delegates, arg }
    }

    pub fn run(&self, data: &mut TData) -> Outcome {
        for stored in &self.delegates {
            let scope = data.create_delegate_scope(stored.ability_id);
            stored.event_fn.invoke(data, scope, self.arg)?;
        }
        outcome::OK
    }
}
