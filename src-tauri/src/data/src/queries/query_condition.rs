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

use dyn_clone::DynClone;

use crate::core::primitives::Source;
use crate::delegates::query_value::QueryValue;
use crate::game_states::game_state::GameState;

/// Wrapper around condition functions to enable closures to be cloned.
pub trait ConditionFnWrapper<TArg>: DynClone + Send {
    fn invoke(&self, data: &GameState, source: Source, arg: &TArg) -> Option<bool>;
}

dyn_clone::clone_trait_object!(<TArg> ConditionFnWrapper<TArg>);

impl<TArg, F> ConditionFnWrapper<TArg> for F
where
    F: Fn(&GameState, Source, &TArg) -> Option<bool> + Copy + Send,
{
    fn invoke(&self, data: &GameState, source: Source, arg: &TArg) -> Option<bool> {
        self(data, source, arg)
    }
}

pub type BoxedConditionFn<TArg> = Box<dyn ConditionFnWrapper<TArg>>;

#[derive(Clone)]
pub enum QueryCondition<TArg> {
    Predicate(BoxedConditionFn<TArg>),
}

impl<TArg> QueryCondition<TArg> {
    /// Returns true if this condition is currently satisfied and the query
    /// modifier should take effect.
    pub fn passes(&self, game: &GameState, source: Source, arg: &TArg) -> bool {
        self.passes_helper(game, source, arg) == Some(true)
    }

    fn passes_helper(&self, game: &GameState, source: Source, arg: &TArg) -> Option<bool> {
        match self {
            QueryCondition::Predicate(function) => function.invoke(game, source, arg),
        }
    }
}
