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
use crate::delegates::scope::{DelegateScope, EffectScope};
use crate::game_states::game_state::GameState;

#[derive(Clone, Copy)]
pub struct EffectState<T> {
    _number: usize,
    value: PhantomData<T>,
}

impl<T> EffectState<T> {
    pub const fn new(number: usize) -> Self {
        Self { _number: number, value: PhantomData }
    }

    pub fn store(&self, game: &mut GameState, scope: EffectScope, value: T) {
        todo!("")
    }

    pub fn get(&self, game: &GameState, scope: DelegateScope) -> T {
        todo!("")
    }
}
