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
use data::core::function_types::{Effect, Predicate};
use data::core::primitives::{
    AbilityId, EventId, HasSource, PermanentId, PlayerName, Source, StackItemId,
};
use data::delegates::delegate_type::DelegateType;
use data::delegates::scope::AbilityScope;
use data::events::event_context::EventContext;
use data::events::game_event::GameEvent;
use data::game_states::game_state::GameState;
use utils::outcome;
use utils::outcome::Outcome;

/// Extensions to event delegates for triggering abilities.
///
/// This is primarily implemented as a trait to prevent crate cyclic
/// dependencies, since we don't want our core 'data' crate to have a bunch of
/// rules logic in it.
pub trait TriggerExt<TArg> {
    /// Trigger this ability if a predicate is true.
    ///
    /// Fires only while this card is on the battlefield. The ability will be
    /// placed on the stack the next time a player would receive priority.
    ///
    /// This creates a delegate using [DelegateType::Ability], meaning the
    /// trigger will not fire if the owning card loses all abilities.
    fn add_trigger(
        &mut self,
        scope: AbilityScope,
        predicate: impl Fn(&GameState, EventContext, &TArg) -> Option<bool>
            + Copy
            + Send
            + Sync
            + 'static,
    );

    /// Trigger an effect the next time a predicate is true, associated with a
    /// given permanent.
    ///
    /// The effect is only applied once and then removed, and will only trigger
    /// if the [PermanentId] permanent is still on the battlefield.
    ///
    ///
    /// This creates a delegate using [DelegateType::Effect], meaning the
    /// trigger *will* still fire if the owning card loses all abilities.
    fn add_one_time_trigger(
        &mut self,
        scope: AbilityScope,
        permanent_id: PermanentId,
        predicate: impl Fn(&GameState, Source, &TArg) -> Option<bool> + Clone + Send + Sync + 'static,
        effect: impl Fn(&mut GameState, EventContext) + Clone + Send + Sync + 'static,
    );

    /// Equivalent to [Self::add_trigger], but only triggers if the ability is
    /// not currently on the stack. Used to implement state-based triggers.
    fn add_trigger_if_not_on_stack(
        &mut self,
        scope: AbilityScope,
        predicate: impl Fn(&GameState, EventContext, &TArg) -> Option<bool>
            + Copy
            + Send
            + Sync
            + 'static,
    );
}

impl<TArg: Clone> TriggerExt<TArg> for GameEvent<TArg> {
    fn add_trigger(
        &mut self,
        scope: AbilityScope,
        predicate: impl Fn(&GameState, EventContext, &TArg) -> Option<bool>
            + Copy
            + Send
            + Sync
            + 'static,
    ) {
        self.add_battlefield_ability(scope, move |g, c, arg| {
            if predicate(g, c, arg) == Some(true) {
                trigger_ability(g, c.this, c.controller);
            }
        });
    }

    fn add_one_time_trigger(
        &mut self,
        scope: AbilityScope,
        permanent_id: PermanentId,
        predicate: impl Fn(&GameState, Source, &TArg) -> Option<bool> + Clone + Send + Sync + 'static,
        effect: impl Fn(&mut GameState, EventContext) + Clone + Send + Sync + 'static,
    ) {
    }

    fn add_trigger_if_not_on_stack(
        &mut self,
        scope: AbilityScope,
        predicate: impl Fn(&GameState, EventContext, &TArg) -> Option<bool>
            + Copy
            + Send
            + Sync
            + 'static,
    ) {
        self.add_battlefield_ability(scope, move |g, c, arg| {
            if predicate(g, c, arg) == Some(true) && !is_ability_on_stack(g, c.this) {
                trigger_ability(g, c.this, c.controller);
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
