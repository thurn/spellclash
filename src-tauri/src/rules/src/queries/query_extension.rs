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
use data::delegates::card_query_delegate_list::{
    CardDelegateExecution, CardQueryDelegateList, DelegateBuilder,
};
use data::delegates::delegate_type::DelegateType;
use data::delegates::query_value::QueryValue;
use data::delegates::scope::{EffectContext, Scope};
use data::game_states::game_state::GameState;

pub trait QueryExt<TArg, TResult> {
    /// Apply a transformation function only during turns in which the card's
    /// ability has been marked as being applied to the target argument of the
    /// event.
    ///
    /// Note: this adds a [DelegateType::Effect] delegate which will still be
    /// invoked if the card owning this delegate loses all abilities. Use
    /// [Self::this_turn_ability] to add an ability for one turn.
    fn this_turn(
        &mut self,
        transformation: impl Fn(&GameState, EffectContext, &TArg) -> Option<TResult>
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
        transformation: impl Fn(&GameState, EffectContext, &TArg) -> Option<TResult>
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
        transformation: impl Fn(&GameState, EffectContext, &TArg) -> Option<TResult>
            + Copy
            + Send
            + Sync
            + 'static,
    ) {
        this_turn_impl(self, DelegateType::Effect, transformation);
    }

    fn this_turn_ability(
        &mut self,
        transformation: impl Fn(&GameState, EffectContext, &TArg) -> Option<TResult>
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
    transformation: impl Fn(&GameState, EffectContext, &TArg) -> Option<TResult>
        + Copy
        + Send
        + Sync
        + 'static,
) {
    list.add_delegate(DelegateBuilder {
        delegate_type,
        execution_type: CardDelegateExecution::Any,
        query: Box::new(move |g: &GameState, s: Scope, arg: &TArg| {
            let entity_id = g.card(*arg)?.entity_id();
            let mut result = None;
            for effect_id in g.ability_state.this_turn.active_effects(s.ability_id, entity_id) {
                let context = EffectContext {
                    scope: Scope {
                        controller: s.controller,
                        ability_id: s.ability_id,
                        timestamp: effect_id.timestamp(),
                    },
                    effect_id,
                };
                result = transformation(g, context, arg);
            }
            result
        }),
    });
}
