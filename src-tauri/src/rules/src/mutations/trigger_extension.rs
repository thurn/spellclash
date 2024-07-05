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

use data::card_states::stack_ability_state::StackAbilityState;
use data::card_states::zones::ZoneQueries;
use data::core::primitives::{AbilityId, EffectId, PlayerName, StackItemId};
use data::delegates::delegate_type::DelegateType;
use data::delegates::event_delegate_list::EventDelegateList;
use data::delegates::scope::Scope;
use data::game_states::game_state::GameState;
use utils::outcome;
use utils::outcome::Outcome;

use crate::mutations::delayed_trigger;

/// Extensions to event delegates for triggering abilities.
///
/// This is primarily implemented as a trait to prevent crate cyclic
/// dependencies, since we don't want our core 'data' crate to have a bunch of
/// rules logic in it.
pub trait TriggerExt<TArg> {
    /// Trigger the [Scope] ability if a predicate is true. The ability will be
    /// placed on the stack the next time a player would receive priority.
    ///
    /// This creates a delegate using [DelegateType::Ability], meaning the
    /// trigger will not fire if the owning card loses all abilities.
    fn trigger_if(
        &mut self,
        predicate: impl Fn(&GameState, Scope, &TArg) -> bool + Copy + Send + Sync + 'static,
    );

    /// Trigger the [Scope] delayed trigger ability if a predicate is true *and*
    /// this delayed trigger has previously been enabled by a call to
    /// [delayed_trigger::enable].
    ///
    /// The delayed trigger is automatically disabled after being triggered.
    ///
    /// This creates a delegate using [DelegateType::Effect], meaning the
    /// trigger will still fire if the owning card loses all abilities.
    fn delayed_trigger_if(
        &mut self,
        predicate: impl Fn(&GameState, Scope, EffectId, &TArg) -> bool + Copy + Send + Sync + 'static,
    );

    /// Trigger the [Scope] ability as long as it is not currently on the stack.
    ///
    /// Used for state-based triggers.
    ///
    /// This creates a delegate using [DelegateType::Ability], meaning the
    /// trigger will not fire if the owning card loses all abilities.
    fn trigger_if_not_on_stack(
        &mut self,
        predicate: impl Fn(&GameState, Scope, &TArg) -> bool + Copy + Send + Sync + 'static,
    );
}

impl<TArg: Clone> TriggerExt<TArg> for EventDelegateList<TArg> {
    fn trigger_if(
        &mut self,
        predicate: impl Fn(&GameState, Scope, &TArg) -> bool + Copy + Send + Sync + 'static,
    ) {
        self.whenever(DelegateType::Ability, move |g, s, arg| {
            if predicate(g, s, arg) {
                trigger_ability(g, s.ability_id, s.controller);
            }
        });
    }

    fn delayed_trigger_if(
        &mut self,
        predicate: impl Fn(&GameState, Scope, EffectId, &TArg) -> bool + Copy + Send + Sync + 'static,
    ) {
        self.whenever(DelegateType::Effect, move |g, s, arg| {
            let Some(effect_ids) = g.ability_state.delayed_triggers.get(&s.ability_id) else {
                return;
            };

            let to_trigger = effect_ids
                .iter()
                .filter(|&&effect_id| predicate(g, s, effect_id, arg))
                .copied()
                .collect::<Vec<_>>();

            for effect_id in to_trigger {
                let ability = trigger_ability(g, s.ability_id, s.controller);
                ability.delayed_trigger_effect_id = Some(effect_id);
                delayed_trigger::disable(g, s.ability_id, effect_id);
            }
        });
    }

    fn trigger_if_not_on_stack(
        &mut self,
        predicate: impl Fn(&GameState, Scope, &TArg) -> bool + Copy + Send + Sync + 'static,
    ) {
        self.whenever(DelegateType::Ability, move |g, s, arg| {
            if predicate(g, s, arg) && !is_ability_on_stack(g, s.ability_id) {
                trigger_ability(g, s.ability_id, s.controller);
            }
        });
    }
}

/// Marks an ability as having triggered.
///
/// The ability is not placed on the stack immediately, it waits until the next
/// time a player would receive priority.
pub fn trigger_ability(
    game: &mut GameState,
    ability_id: AbilityId,
    owner: PlayerName,
) -> &mut StackAbilityState {
    game.zones.create_triggered_ability(ability_id, owner, vec![])
}

/// Returns true if an ability with the given [AbilityId] is currently on the
/// stack
pub fn is_ability_on_stack(game: &GameState, ability_id: AbilityId) -> bool {
    game.stack().iter().any(|&stack_item_id| match stack_item_id {
        StackItemId::StackAbility(id) => game.stack_ability(id).ability_id == ability_id,
        _ => false,
    })
}
