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

use data::card_states::stack_ability_state::{StackAbilityCustomEffect, StackAbilityState};
use data::card_states::zones::ZoneQueries;
use data::core::ability_scope::AbilityScope;
use data::events::event_context::EventContext;
use data::events::game_event::GameEvent;
use data::game_states::game_state::GameState;
use enumset::EnumSet;
use primitives::game_primitives::{
    AbilityId, EventId, HasSource, PermanentId, PlayerName, Source, StackItemId,
};

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
    /// This creates an ability callback, meaning the trigger will not fire if
    /// the owning card loses all abilities.
    fn add_trigger(
        &mut self,
        scope: AbilityScope,
        predicate: impl Fn(&GameState, Source, &TArg) -> Option<bool> + Copy + Send + Sync + 'static,
    );

    /// Trigger an effect the next time a predicate is true, associated with a
    /// given permanent.    
    ///
    /// The effect is associated with the [EventId] in the provided
    /// [EventContext] in order to only be applied once. The effect will only
    /// trigger if the [PermanentId] permanent is still on the battlefield.

    /// This creates an effect callback, meaning the trigger *will* still fire
    /// if the owning card loses all abilities.
    fn add_one_time_trigger(
        &mut self,
        context: EventContext,
        permanent_id: PermanentId,
        predicate: impl Fn(&GameState, Source, &TArg) -> Option<bool> + Copy + Send + Sync + 'static,
        effect: impl Fn(&mut GameState, EventContext) + Copy + Send + Sync + 'static,
    );

    /// Equivalent to [Self::add_trigger], but only triggers if the ability is
    /// not currently on the stack.
    fn add_state_trigger(
        &mut self,
        scope: AbilityScope,
        predicate: impl Fn(&GameState, Source, &TArg) -> Option<bool> + Copy + Send + Sync + 'static,
    );
}

impl<TArg: Clone> TriggerExt<TArg> for GameEvent<TArg> {
    fn add_trigger(
        &mut self,
        scope: AbilityScope,
        predicate: impl Fn(&GameState, Source, &TArg) -> Option<bool> + Copy + Send + Sync + 'static,
    ) {
        self.add_battlefield_ability(scope, move |g, c, arg| {
            if predicate(g, c.source(), arg) == Some(true) {
                trigger_ability(g, c.this, c.controller);
            }
        });
    }

    fn add_one_time_trigger(
        &mut self,
        context: EventContext,
        permanent_id: PermanentId,
        predicate: impl Fn(&GameState, Source, &TArg) -> Option<bool> + Copy + Send + Sync + 'static,
        effect: impl Fn(&mut GameState, EventContext) + Copy + Send + Sync + 'static,
    ) {
        self.add_effect(context, EnumSet::all(), move |g, c, arg| {
            if g.has_card(permanent_id)
                && !g.ability_state.fired_one_time_effects.contains(&context.event_id)
                && predicate(g, c.original_source, arg) == Some(true)
            {
                let ability = g.zones.create_triggered_ability(c.this, c.controller, vec![]);
                ability.custom_effect = Some(StackAbilityCustomEffect::new(c.event_id, effect));
                g.ability_state.fired_one_time_effects.insert(context.event_id);
            }
        });
    }

    fn add_state_trigger(
        &mut self,
        scope: AbilityScope,
        predicate: impl Fn(&GameState, Source, &TArg) -> Option<bool> + Copy + Send + Sync + 'static,
    ) {
        self.add_battlefield_ability(scope, move |g, c, arg| {
            if predicate(g, c.source(), arg) == Some(true) && !is_ability_on_stack(g, c.this) {
                trigger_ability(g, c.this, c.controller);
            }
        });
    }
}

/// Marks an ability as having triggered.
///
/// The ability is not placed on the stack immediately, it waits until the next
/// time a player would receive priority.
fn trigger_ability(
    game: &mut GameState,
    ability_id: AbilityId,
    owner: PlayerName,
) -> &mut StackAbilityState {
    game.zones.create_triggered_ability(ability_id, owner, vec![])
}

/// Returns true if an ability with the given [AbilityId] is currently on the
/// stack
fn is_ability_on_stack(game: &GameState, ability_id: AbilityId) -> bool {
    game.stack().iter().any(|&stack_item_id| match stack_item_id {
        StackItemId::StackAbility(id) => game.stack_ability(id).ability_id == ability_id,
        _ => false,
    })
}
