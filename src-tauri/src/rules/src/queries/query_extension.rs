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

use data::card_states::zones::{ToCardId, ZoneQueries};
use data::delegates::card_query_delegate_list::{CardDelegateExecution, CardQueryDelegateList};
use data::delegates::delegate_data::{DelegateType, QueryValue};
use data::delegates::event_delegate_list::EventDelegateList;
use data::delegates::scope::Scope;
use data::game_states::game_state::GameState;
use utils::outcome;

use crate::mutations::trigger_extension::{is_ability_on_stack, trigger_ability};

pub trait QueryExt<TArg, TResult> {
    /// Apply a transformation function only during turns in which the card's
    /// ability has been marked as being applied to the target argument of the
    /// event.
    ///
    /// This adds a [DelegateType::Effect] delegate which will still be invoked
    /// if the card owning this delegate loses all abilities.
    fn this_turn(
        &mut self,
        transformation: impl Fn(&GameState, Scope, &TArg) -> Option<TResult>
            + Copy
            + Send
            + Sync
            + 'static,
    );

    /// Equivalent of [Self::this_turn] which adds a
    /// [DelegateType::Ability] delegate, i.e. one which will stop being
    /// invoked if the card owning this delegate loses all abilities.
    fn this_turn_ability(
        &mut self,
        transformation: impl Fn(&GameState, Scope, &TArg) -> Option<TResult>
            + Copy
            + Send
            + Sync
            + 'static,
    );
}

impl<TArg: ToCardId, TResult: QueryValue> QueryExt<TArg, TResult>
    for CardQueryDelegateList<TArg, TResult>
{
    fn this_turn(
        &mut self,
        transformation: impl Fn(&GameState, Scope, &TArg) -> Option<TResult>
            + Copy
            + Send
            + Sync
            + 'static,
    ) {
        this_turn_impl(self, DelegateType::Effect, transformation);
    }

    fn this_turn_ability(
        &mut self,
        transformation: impl Fn(&GameState, Scope, &TArg) -> Option<TResult>
            + Copy
            + Send
            + Sync
            + 'static,
    ) {
        this_turn_impl(self, DelegateType::Ability, transformation);
    }
}

fn this_turn_impl<TArg: ToCardId, TResult: QueryValue>(
    list: &mut CardQueryDelegateList<TArg, TResult>,
    delegate_type: DelegateType,
    transformation: impl Fn(&GameState, Scope, &TArg) -> Option<TResult> + Copy + Send + Sync + 'static,
) {
    list.add_delegate(delegate_type, CardDelegateExecution::Any, move |g, s, arg| {
        let entity_id = g.card(*arg)?.entity_id();
        let mut result = None;
        for effect_id in g.ability_state.this_turn.active_effects(s.ability_id, entity_id) {
            let scope = Scope {
                controller: s.controller,
                ability_id: s.ability_id,
                timestamp: effect_id.timestamp(),
            };
            result = transformation(g, scope, arg);
        }
        result
    })
}
