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

use std::fmt::{Debug, Formatter};

use enumset::EnumSet;
use invokable::InvokableType;
use primitives::game_primitives::{AbilityId, HasController, Source, Zone};

use crate::card_states::zones::ZoneQueries;
use crate::core::ability_scope::AbilityScope;
use crate::core::function_types::Mutation;
use crate::core::layer::{EffectSortingKey, PRINTED_RULE_SORTING_KEY};
use crate::core::rule_type::RuleType;
use crate::events::event_context::EventContext;
use crate::game_states::game_state::GameState;
use crate::properties::duration::Duration;

#[derive(Clone)]
pub struct GameEventCallback<TArg> {
    pub ability_id: AbilityId,
    pub zones: EnumSet<Zone>,
    pub duration: Duration,
    pub rule_type: RuleType,
    pub effect_sorting_key: Option<EffectSortingKey>,
    pub function: Box<dyn Mutation<TArg>>,
}

impl<TArg> Debug for GameEventCallback<TArg> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("GameEventCallback").field("zones", &self.zones).finish()
    }
}

#[derive(Debug, Clone)]
pub struct GameEvent<TArg> {
    pub callbacks: Vec<GameEventCallback<TArg>>,
}

impl<TArg> Default for GameEvent<TArg> {
    fn default() -> Self {
        Self { callbacks: Vec::new() }
    }
}

impl<TArg> GameEvent<TArg> {
    /// Adds an event callback for a printed ability of a card which only fires
    /// while this card is on the battlefield.
    pub fn add_battlefield_ability(
        &mut self,
        scope: AbilityScope,
        function: impl Fn(&mut GameState, EventContext, &TArg) + Copy + Send + Sync + 'static,
    ) {
        self.add_ability(scope, Zone::Battlefield, function);
    }

    /// Adds an event callback for a printed ability of a card.
    pub fn add_ability(
        &mut self,
        scope: AbilityScope,
        zones: impl Into<EnumSet<Zone>>,
        function: impl Fn(&mut GameState, EventContext, &TArg) + Copy + Send + Sync + 'static,
    ) {
        self.callbacks.push(GameEventCallback {
            ability_id: scope.ability_id,
            zones: zones.into(),
            duration: Duration::Continuous,
            rule_type: RuleType::Ability(scope.ability_id.card_id),
            effect_sorting_key: Some(PRINTED_RULE_SORTING_KEY),
            function: Box::new(function),
        });
    }

    /// Adds an effect callback.
    ///
    /// Unlike ability callbacks, this function will still be invoked if the
    /// owning card loses all abilities.
    pub fn add_effect(
        &mut self,
        context: EventContext,
        zones: impl Into<EnumSet<Zone>>,
        function: impl Fn(&mut GameState, EventContext, &TArg) + Copy + Send + Sync + 'static,
    ) {
        self.callbacks.push(GameEventCallback {
            ability_id: context.this,
            zones: zones.into(),
            duration: Duration::Continuous,
            rule_type: RuleType::Effect,
            effect_sorting_key: None,
            function: Box::new(function),
        });
    }

    /// Removes all callbacks added by the given [AbilityId].
    pub fn remove_callbacks(&mut self, ability_id: AbilityId) {
        self.callbacks.retain(|callback| callback.ability_id != ability_id);
    }
}
