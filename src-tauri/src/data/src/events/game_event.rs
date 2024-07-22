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

use crate::card_states::zones::ZoneQueries;
use crate::core::function_types::Mutation;
use crate::core::primitives::{AbilityId, HasController, Source, Zone};
use crate::delegates::delegate_type::DelegateType;
use crate::delegates::scope::AbilityScope;
use crate::events::event_context::EventContext;
use crate::game_states::game_state::GameState;

#[derive(Clone)]
pub struct GameEventCallback<TArg> {
    pub ability_id: AbilityId,
    pub zones: EnumSet<Zone>,
    pub delegate_type: DelegateType,
    pub function: Box<dyn Mutation<TArg>>,
}

impl<TArg> Debug for GameEventCallback<TArg> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("GameEventCallback").field("zones", &self.zones).finish()
    }
}

impl<TArg: 'static> GameEventCallback<TArg> {
    pub fn build_context(&self, game: &GameState, event_source: Source) -> Option<EventContext> {
        let card = game.card(self.ability_id)?;
        if !self.zones.contains(card.zone) {
            return None;
        };

        Some(EventContext {
            event_id: Default::default(),
            this: self.ability_id,
            controller: card.controller(),
            current_turn: game.turn,
            event_source,
        })
    }
}

#[derive(Debug, Clone, Default)]
pub struct GameEvent<TArg> {
    pub callbacks: Vec<GameEventCallback<TArg>>,
}

impl<TArg> GameEvent<TArg> {
    pub fn add_battlefield_ability(
        &mut self,
        scope: AbilityScope,
        function: impl Fn(&mut GameState, EventContext, &TArg) + Copy + Send + Sync + 'static,
    ) {
        self.add_ability(scope, Zone::Battlefield, function);
    }

    pub fn add_ability(
        &mut self,
        scope: AbilityScope,
        zones: impl Into<EnumSet<Zone>>,
        function: impl Fn(&mut GameState, EventContext, &TArg) + Copy + Send + Sync + 'static,
    ) {
        self.callbacks.push(GameEventCallback {
            ability_id: scope.ability_id,
            zones: zones.into(),
            delegate_type: DelegateType::Ability,
            function: Box::new(function),
        });
    }

    /// Removes all callbacks added by the given [AbilityId].
    pub fn remove_callbacks(&mut self, ability_id: AbilityId) {
        self.callbacks.retain(|callback| callback.ability_id != ability_id);
    }
}
