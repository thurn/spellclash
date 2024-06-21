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
use data::core::primitives::{AbilityId, PlayerName, StackItemId};
use data::delegates::event_delegate_list::EventDelegateList;
use data::delegates::scope::DelegateScope;
use data::game_states::game_state::GameState;
use utils::outcome;
use utils::outcome::Outcome;

pub trait TriggerExt<TArg> {
    fn trigger_if(
        &mut self,
        predicate: impl Fn(&GameState, DelegateScope, &TArg) -> bool + Copy + Send + Sync + 'static,
    );

    fn trigger_if_not_on_stack(
        &mut self,
        predicate: impl Fn(&GameState, DelegateScope, &TArg) -> bool + Copy + Send + Sync + 'static,
    );
}

impl<TArg> TriggerExt<TArg> for EventDelegateList<GameState, TArg> {
    fn trigger_if(
        &mut self,
        predicate: impl Fn(&GameState, DelegateScope, &TArg) -> bool + Copy + Send + Sync + 'static,
    ) {
        self.whenever(move |g, s, arg| {
            if predicate(g, s, arg) {
                trigger_ability(g, s.ability_id, s.controller)?;
            }
            outcome::OK
        });
    }

    fn trigger_if_not_on_stack(
        &mut self,
        predicate: impl Fn(&GameState, DelegateScope, &TArg) -> bool + Copy + Send + Sync + 'static,
    ) {
        self.whenever(move |g, s, arg| {
            if predicate(g, s, arg) && !is_ability_on_stack(g, s.ability_id) {
                trigger_ability(g, s.ability_id, s.controller)?;
            }
            outcome::OK
        });
    }
}

/// Marks an ability as having triggered.
///
/// The ability is not placed on the stack immediately, it waits until the next
/// time a player would receive priority.
pub fn trigger_ability(game: &mut GameState, ability_id: AbilityId, owner: PlayerName) -> Outcome {
    game.zones.create_triggered_ability(ability_id, owner, vec![]);
    outcome::OK
}

/// Returns true if an ability with the given [AbilityId] is currently on the
/// stack
pub fn is_ability_on_stack(game: &GameState, ability_id: AbilityId) -> bool {
    game.stack().iter().any(|&stack_item_id| match stack_item_id {
        StackItemId::StackAbility(id) => game.stack_ability(id).ability_id == ability_id,
        _ => false,
    })
}