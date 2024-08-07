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

use std::marker::PhantomData;

use primitives::game_primitives::EventId;

use crate::game_states::game_state::GameState;
use crate::game_states::state_value::StateValue;

#[derive(Clone, Copy)]
pub struct EffectState<T: Into<StateValue> + TryFrom<StateValue> + PartialEq> {
    value: PhantomData<T>,
}

impl<T: Into<StateValue> + TryFrom<StateValue> + PartialEq> EffectState<T> {
    const STATE: EffectState<T> = EffectState { value: PhantomData };

    pub fn new() -> &'static Self {
        &Self::STATE
    }

    /// Sets the value of the state associated with the provided [EventId] to
    /// the given value.
    pub fn store(&self, game: &mut GameState, event_id: EventId, value: T) {
        game.ability_state.effect_state.insert(event_id, value.into());
    }

    /// Retrieves the value of the state associated with the provided
    /// [EventId], if one is present.
    ///
    /// Returns None if no state is associated with the provided [EventId] or
    /// if the value found cannot be converted to the expected type.
    pub fn get(&self, game: &GameState, event_id: EventId) -> Option<T> {
        let state = game.ability_state.effect_state.get(&event_id)?;
        T::try_from(state.clone()).ok()
    }

    /// Retrieves and removes the state value associated with the provided
    /// [EventId], if one is present.
    pub fn pop(&self, game: &mut GameState, event_id: EventId) -> Option<T> {
        let state = game.ability_state.effect_state.remove(&event_id)?;
        T::try_from(state).ok()
    }

    /// Returns true if `other` is equal to the stored state value for this
    /// [EventId]. Returns false if they are not equal or if no state is
    /// associated with the provided [EventId].
    pub fn matches(&self, game: &GameState, event_id: EventId, other: T) -> bool {
        self.get(game, event_id) == Some(other)
    }
}
