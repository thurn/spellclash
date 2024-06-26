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

use crate::core::primitives::CardId;
use crate::delegates::scope::{EffectContext, Scope};
use crate::game_states::game_state::GameState;

#[derive(Clone, Copy)]
pub struct EffectState<T> {
    _number: u32,
    value: PhantomData<T>,
}

impl<T> EffectState<T> {
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

    pub fn store(&self, game: &mut GameState, scope: EffectContext, value: T) {
        todo!("")
    }

    pub fn get(&self, game: &GameState, scope: Scope) -> T {
        todo!("")
    }
}
