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

use data::card_states::zones::ZoneQueries;
use data::core::primitives::HasCardId;
use data::delegates::card_delegate_list::CardDelegateList;
use data::delegates::event_delegate_list::EventDelegateList;
use data::delegates::scope::DelegateScope;
use data::game_states::game_state::GameState;
use utils::outcome;

use crate::mutations::trigger_extension::{is_ability_on_stack, trigger_ability};

pub trait QueryExt<TArg, TResult> {
    /// Apply a transformation function only during turns in which the card's
    /// ability has been marked as being applied to the target argument of the
    /// event.
    fn this_turn(
        &mut self,
        transformation: impl Fn(&GameState, DelegateScope, TResult) -> TResult
            + Copy
            + Send
            + Sync
            + 'static,
    );
}

impl<TArg: HasCardId, TResult> QueryExt<TArg, TResult>
    for CardDelegateList<GameState, TArg, TResult>
{
    fn this_turn(
        &mut self,
        transformation: impl Fn(&GameState, DelegateScope, TResult) -> TResult
            + Copy
            + Send
            + Sync
            + 'static,
    ) {
        self.any(move |g, s, arg, mut result| {
            for _ in 0..g.this_turn.effect_count(s.ability_id, g.card(arg).entity_id) {
                result = transformation(g, s, result);
            }
            result
        })
    }
}
