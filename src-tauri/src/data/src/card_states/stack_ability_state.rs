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

#[allow(unused)] // Used in docs
use crate::card_states::zones::Zones;
use crate::card_states::zones::{HasZones, ToCardId};
use crate::core::function_types::Effect;
use crate::core::primitives::{
    AbilityId, CardId, EntityId, EventId, HasController, HasPlayerName, ObjectId, PlayerName,
    StackAbilityId,
};
use crate::events::event_context::EventContext;
use crate::game_states::game_state::GameState;

/// Custom effect to invoke when an ability on the stack resolves.
#[derive(Clone)]
pub struct StackAbilityCustomEffect {
    /// Original [EventId] which caused this effect to be created
    pub event_id: EventId,

    /// Effect function to apply
    pub effect: Box<dyn Effect>,
}

impl StackAbilityCustomEffect {
    pub fn new(
        event_id: EventId,
        effect: impl Fn(&mut GameState, EventContext) + Copy + Send + Sync + 'static,
    ) -> Self {
        Self { event_id, effect: Box::new(effect) }
    }
}

/// Represents the state of a triggered or activated ability which has triggered
/// or is on the stack
#[derive(Clone)]
pub struct StackAbilityState {
    /// ID of this ability on the stack.
    pub id: StackAbilityId,

    /// Identifies this ability within its parent card's oracle text.
    pub ability_id: AbilityId,

    /// Identifier for the ability within its zone
    pub object_id: ObjectId,

    /// True if this ability has been placed on the stack.
    ///
    /// Activated abilities are created directly on the stack. Triggered
    /// abilities are moved to the stack the next time a player gains priority
    /// after they trigger.
    pub placed_on_stack: bool,

    /// The player who this ability belongs to, who initially created it.
    pub owner: PlayerName,

    /// The player who can currently make decisions about this ability.
    pub controller: PlayerName,

    /// Targets for this ability, selected when it is placed on the stack.
    pub targets: Vec<EntityId>,

    /// A custom effect function to invoke when this stack ability resolves.
    ///
    /// By default, the effect function on the underlying ability is invoked
    /// when an item on the stack resolves. Custom effects are used for things
    /// like delayed triggers, where an effect is created separately from its
    /// primary ability.
    pub custom_effect: Option<StackAbilityCustomEffect>,
}

impl HasPlayerName for StackAbilityState {
    fn player_name(&self) -> PlayerName {
        self.owner
    }
}

impl HasController for StackAbilityState {
    fn controller(&self) -> PlayerName {
        self.controller
    }
}
