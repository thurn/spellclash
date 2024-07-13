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

use std::any::Any;
use std::collections::BTreeMap;
use std::num::NonZeroU64;

use dyn_clone::DynClone;
use serde::{Deserialize, Serialize};
use utils::outcome::Outcome;

use crate::core::primitives::Source;
use crate::game_states::game_state::GameState;

#[derive(Clone, Serialize, Deserialize)]
pub struct Registered<F: Clone + Send + Sync + 'static> {
    id: u64,

    #[serde(skip)]
    function: Option<F>,
}

impl<F: Clone + Send + Sync + 'static> Registered<F> {
    pub fn initialize(&mut self, registry: &Registry) {
        let value = registry.registered.get(&self.id).expect("Registered function not found");
        self.function =
            Some(value.downcast_ref::<F>().expect("Registered function of incorrect type").clone());
    }

    pub fn get(&self) -> &F {
        &self.function.as_ref().expect("Registered function not initialized")
    }
}

#[derive(Default)]
pub struct Registry {
    counter: u64,
    registered: BTreeMap<u64, Box<dyn Any + Send + Sync>>,
}

impl Registry {
    pub fn add<F: Clone + Send + Sync + 'static>(&mut self, function: F) -> Registered<F> {
        self.counter += 1;
        self.registered.insert(self.counter, Box::new(function.clone()));
        Registered { id: self.counter, function: Some(function) }
    }

    pub fn add_query<TArg, TResult>(
        &mut self,
        function: impl QueryFn<TArg, TResult>,
    ) -> Registered<BoxedQueryFn<TArg, TResult>> {
        self.add(Box::new(function))
    }

    pub fn add_mutation<TArg>(
        &mut self,
        function: impl MutationFn<TArg>,
    ) -> Registered<BoxedMutationFn<TArg>> {
        self.add(Box::new(function))
    }
}

pub trait QueryFn<TArg, TResult>: DynClone + Send + Sync + 'static {
    fn invoke(&self, data: &GameState, source: Source, arg: &TArg) -> TResult;
}

dyn_clone::clone_trait_object!(<TArg, TResult> QueryFn<TArg, TResult>);

impl<TArg, TResult, F> QueryFn<TArg, TResult> for F
where
    F: Fn(&GameState, Source, &TArg) -> TResult + Copy + Clone + Send + Sync + 'static,
{
    fn invoke(&self, data: &GameState, source: Source, arg: &TArg) -> TResult {
        self(data, source, arg)
    }
}

pub type BoxedQueryFn<TArg, TResult> = Box<dyn QueryFn<TArg, TResult>>;

pub trait MutationFn<TArg>: DynClone + Send + Sync + 'static {
    fn invoke(&self, data: &mut GameState, source: Source, arg: &TArg) -> Outcome;
}

dyn_clone::clone_trait_object!(<TArg> MutationFn<TArg>);

impl<TArg, F> MutationFn<TArg> for F
where
    F: Fn(&GameState, Source, &TArg) -> Outcome + Copy + Clone + Send + Sync + 'static,
{
    fn invoke(&self, data: &mut GameState, source: Source, arg: &TArg) -> Outcome {
        self(data, source, arg)
    }
}

pub type BoxedMutationFn<TArg> = Box<dyn MutationFn<TArg>>;
