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

use crate::core::primitives::{CardId, EffectId};
use crate::delegates::scope::{EffectContext, Scope};
use crate::game_states::game_state::GameState;
use crate::game_states::state_value::StateValue;

#[derive(Clone, Copy)]
pub struct EffectState<T: Into<StateValue> + TryFrom<StateValue>> {
    _number: u32,
    value: PhantomData<T>,
}

impl<T: Into<StateValue> + TryFrom<StateValue>> EffectState<T> {
    const STATE0: EffectState<T> = EffectState { _number: 0, value: PhantomData };
    const STATE1: EffectState<T> = EffectState { _number: 1, value: PhantomData };
    const STATE2: EffectState<T> = EffectState { _number: 2, value: PhantomData };
    const STATE3: EffectState<T> = EffectState { _number: 3, value: PhantomData };
    const STATE4: EffectState<T> = EffectState { _number: 4, value: PhantomData };

    pub fn new(number: u32) -> &'static Self {
        match number {
            0 => &Self::STATE0,
            1 => &Self::STATE1,
            2 => &Self::STATE2,
            3 => &Self::STATE3,
            4 => &Self::STATE4,
            _ => panic!("Invalid effect state number"),
        }
    }

    /// Sets the value of the state associated with the provided [EffectId] to
    /// the given value.
    pub fn store(&self, game: &mut GameState, effect_id: EffectId, value: T) {
        game.ability_state.effect_state.insert(effect_id, value.into());
    }

    /// Retrieves the value of the state associated with the provided
    /// [EffectId], if one is present.
    ///
    /// Returns None if no state is associated with the provided [EffectId] or
    /// if the value found cannot be converted to the expected type.
    pub fn get(&self, game: &GameState, effect_id: EffectId) -> Option<T> {
        let state = game.ability_state.effect_state.get(&effect_id)?;
        T::try_from(state.clone()).ok()
    }
}
