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

use crate::card_states::zones::{ToCardId, ZoneQueries};
use crate::core::primitives::{AbilityId, EntityId, Zone};
use crate::delegates::card_query_delegate_list::CardDelegateExecution;
use crate::delegates::delegate_type::DelegateType;
use crate::delegates::scope::Scope;
use crate::delegates::stores_delegates::StoresDelegates;
use crate::game_states::game_state::GameState;

/// Wrapper around event functions to enable closures to be cloned.
pub trait EventFnWrapper<TArg>: DynClone + Send {
    fn invoke(&self, data: &mut GameState, scope: Scope, arg: &TArg) -> Outcome;
}

dyn_clone::clone_trait_object!(<TArg> EventFnWrapper<TArg>);

impl<TArg, F> EventFnWrapper<TArg> for F
where
    F: Fn(&mut GameState, Scope, &TArg) -> Outcome + Copy + Send,
{
    fn invoke(&self, data: &mut GameState, scope: Scope, arg: &TArg) -> Outcome {
        self(data, scope, arg)
    }
}

pub type BoxedEventFn<TArg> = Box<dyn EventFnWrapper<TArg>>;

#[derive(Clone)]
pub struct StoredEventDelegate<TArg> {
    zones: EnumSet<Zone>,
    ability_id: AbilityId,
    delegate_type: DelegateType,
    event_fn: BoxedEventFn<TArg>,
}

#[derive(Clone)]
struct EventDelegateBuilder<TArg> {
    delegate_type: DelegateType,
    callback: BoxedEventFn<TArg>,
}

#[derive(Clone)]
pub struct EventDelegateList<TArg> {
    current: Vec<EventDelegateBuilder<TArg>>,
    delegates: Vec<StoredEventDelegate<TArg>>,
}

impl<TArg: Clone> EventDelegateList<TArg> {
    /// Adds a new callback which will be invoked whenever the underlying event
    /// occurs, with a given [DelegateType].
    pub fn whenever(
        &mut self,
        delegate_type: DelegateType,
        value: impl Fn(&mut GameState, Scope, &TArg) -> Outcome + Copy + Send + Sync + 'static,
    ) {
        self.current.push(EventDelegateBuilder { delegate_type, callback: Box::new(value) });
    }

    pub fn invoke_with<'a>(
        &self,
        game: &GameState,
        arg: &'a TArg,
    ) -> EventDelegateInvoker<'a, TArg> {
        let mut result = vec![];
        for stored in &self.delegates {
            let Some(card) = game.card(stored.ability_id) else {
                continue;
            };

            if !stored.zones.contains(card.zone) {
                continue;
            }

            if stored.delegate_type == DelegateType::Ability
                && card.permanent_id().and_then(|id| game.has_lost_all_abilities(id)).is_some()
            {
                // Do not fire callbacks for permanents that have lost all abilities.
                continue;
            }

            result.push(stored.clone());
        }
        EventDelegateInvoker::new(result, arg)
    }

    /// True if no delegates have been defined for this list.
    pub fn is_empty(&self) -> bool {
        self.delegates.is_empty()
    }
}

impl<TArg> StoresDelegates for EventDelegateList<TArg> {
    fn apply_writes(&mut self, id: AbilityId, zones: EnumSet<Zone>) {
        for builder in self.current.drain(..) {
            self.delegates.push(StoredEventDelegate {
                zones,
                ability_id: id,
                delegate_type: builder.delegate_type,
                event_fn: builder.callback,
            });
        }
    }
}

impl<TArg> Default for EventDelegateList<TArg> {
    fn default() -> Self {
        Self { current: vec![], delegates: vec![] }
    }
}

pub struct EventDelegateInvoker<'a, TArg> {
    delegates: Vec<StoredEventDelegate<TArg>>,
    arg: &'a TArg,
}

impl<'a, TArg> EventDelegateInvoker<'a, TArg> {
    pub fn new(delegates: Vec<StoredEventDelegate<TArg>>, arg: &'a TArg) -> Self {
        Self { delegates, arg }
    }

    pub fn run(&self, data: &mut GameState) {
        for stored in &self.delegates {
            let Some(scope) = data.create_scope(stored.ability_id) else {
                continue;
            };
            stored.event_fn.invoke(data, scope, self.arg);
        }
    }
}
