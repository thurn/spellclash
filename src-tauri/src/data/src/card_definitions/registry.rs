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
use std::marker::PhantomData;
use std::num::NonZeroU64;

use dyn_clone::DynClone;
use primitives::game_primitives::Source;
use serde::{Deserialize, Serialize};
use utils::outcome::Outcome;

use crate::game_states::game_state::GameState;
use crate::properties::property_value::PropertyValue;

#[derive(Clone, Serialize, Deserialize)]
pub struct Registered<F: Send + Sync + Default + 'static> {
    id: u64,

    #[serde(skip)]
    function: F,
}

impl<F: Clone + Send + Sync + Default + 'static> Registered<F> {
    pub fn initialize(&mut self, registry: &Registry) {
        let value = registry.registered.get(&self.id).expect("Registered function not found");
        self.function =
            value.downcast_ref::<F>().expect("Registered function of incorrect type").clone();
    }
}

pub fn invoke_query<TArg, TResult>(
    function: &QueryFn<TArg, TResult>,
    data: &GameState,
    source: Source,
    arg: &TArg,
) -> TResult {
    match function.function {
        Some(ref function) => function.invoke(data, source, arg),
        None => panic!("Function not initialized"),
    }
}

pub fn invoke_mutation<TArg>(
    function: &MutationFn<TArg>,
    data: &mut GameState,
    source: Source,
    arg: &TArg,
) -> Outcome {
    match function.function {
        Some(ref function) => function.invoke(data, source, arg),
        None => panic!("Function not initialized"),
    }
}

#[derive(Default)]
pub struct Registry {
    counter: u64,
    registered: BTreeMap<u64, Box<dyn Any + Send + Sync>>,
}

impl Registry {
    pub fn add_query<TArg, TResult>(
        &mut self,
        function: impl CloneableQueryFn<TArg, TResult>,
    ) -> QueryFn<TArg, TResult> {
        self.counter += 1;
        let f = dyn_clone::clone_box(&function);
        self.registered.insert(self.counter, f);
        Registered { id: self.counter, function: Some(dyn_clone::clone_box(&function)) }
    }

    pub fn add_mutation<TArg>(
        &mut self,
        function: impl CloneableMutationFn<TArg>,
    ) -> MutationFn<TArg> {
        self.counter += 1;
        let f = dyn_clone::clone_box(&function);
        self.registered.insert(self.counter, f);
        Registered { id: self.counter, function: Some(dyn_clone::clone_box(&function)) }
    }
}

pub trait CloneableQueryFn<TArg, TResult>: DynClone + Send + Sync + 'static {
    fn invoke(&self, data: &GameState, source: Source, arg: &TArg) -> TResult;
}

dyn_clone::clone_trait_object!(<TArg, TResult> CloneableQueryFn<TArg, TResult>);

impl<TArg, TResult, F> CloneableQueryFn<TArg, TResult> for F
where
    F: Fn(&GameState, Source, &TArg) -> TResult + Copy + Clone + Send + Sync + 'static,
{
    fn invoke(&self, data: &GameState, source: Source, arg: &TArg) -> TResult {
        self(data, source, arg)
    }
}

pub type QueryFn<TArg, TResult> = Registered<Option<Box<dyn CloneableQueryFn<TArg, TResult>>>>;

pub trait CloneableMutationFn<TArg>: DynClone + Send + Sync + 'static {
    fn invoke(&self, data: &mut GameState, source: Source, arg: &TArg) -> Outcome;
}

dyn_clone::clone_trait_object!(<TArg> CloneableMutationFn<TArg>);

impl<TArg, F> CloneableMutationFn<TArg> for F
where
    F: Fn(&GameState, Source, &TArg) -> Outcome + Copy + Clone + Send + Sync + 'static,
{
    fn invoke(&self, data: &mut GameState, source: Source, arg: &TArg) -> Outcome {
        self(data, source, arg)
    }
}

pub type MutationFn<TArg> = Registered<Option<Box<dyn CloneableMutationFn<TArg>>>>;
