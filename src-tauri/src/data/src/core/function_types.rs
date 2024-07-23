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

use dyn_clone::DynClone;
use utils::outcome::Outcome;

use crate::card_definitions::registry::Registered;
use crate::card_states::zones::ToCardId;
use crate::core::primitives::{CardId, EntityId, PermanentId, PlayerName, Source, StackAbilityId};
use crate::events::event_context::EventContext;
use crate::game_states::game_state::GameState;

/// Marker trait for predicate functions which return a boolean value.
///
/// Functions can implement this trait by returning `Option<bool>` in which case
/// both None and Some(false) are converted to false.
pub trait Predicate<TArg>: DynClone + Send + Sync + 'static {
    fn invoke(&self, data: &GameState, source: Source, arg: &TArg) -> bool;
}

dyn_clone::clone_trait_object!(<TArg> Predicate<TArg>);

impl<TArg, F> Predicate<TArg> for F
where
    F: Fn(&GameState, Source, &TArg) -> Option<bool> + Copy + Clone + Send + Sync + 'static,
{
    fn invoke(&self, data: &GameState, source: Source, arg: &TArg) -> bool {
        self(data, source, arg) == Some(true)
    }
}

/// Function which performs a boolean query on the state of a card.
pub trait CardPredicate<TId: ToCardId>:
    Fn(&GameState, Source, TId) -> Option<bool> + 'static + Copy + Send + Sync
{
}

impl<TId: ToCardId, F> CardPredicate<TId> for F where
    F: Fn(&GameState, Source, TId) -> Option<bool> + 'static + Copy + Send + Sync
{
}

pub type CardPredicateFn<TId> =
    Box<dyn Fn(&GameState, Source, TId) -> Option<bool> + 'static + Send + Sync>;

pub trait Effect: DynClone + Send + Sync + 'static {
    fn invoke(&self, data: &mut GameState, context: EventContext);
}

dyn_clone::clone_trait_object!(Effect);

impl<F> Effect for F
where
    F: Fn(&mut GameState, EventContext) + Clone + Send + Sync + 'static,
{
    fn invoke(&self, data: &mut GameState, context: EventContext) {
        self(data, context)
    }
}

pub trait Mutation<TArg>: DynClone + Send + Sync + 'static {
    fn invoke(&self, data: &mut GameState, context: EventContext, arg: &TArg);
}

dyn_clone::clone_trait_object!(<TArg> Mutation<TArg>);

impl<TArg, F> Mutation<TArg> for F
where
    F: Fn(&mut GameState, EventContext, &TArg) + Copy + Send + Sync + 'static,
{
    fn invoke(&self, data: &mut GameState, context: EventContext, arg: &TArg) {
        self(data, context, arg)
    }
}

/// Function which performs a mutation on the state of a card.
pub trait CardMutation<TId: ToCardId>:
    Fn(&mut GameState, Source, TId) -> Outcome + 'static + Copy + Send + Sync
{
}

impl<TId: ToCardId, F> CardMutation<TId> for F where
    F: Fn(&mut GameState, Source, TId) -> Outcome + 'static + Copy + Send + Sync
{
}

pub type CardMutationFn<TId> =
    Box<dyn Fn(&mut GameState, Source, TId) -> Outcome + 'static + Send + Sync>;

pub trait PlayerPredicate:
    Fn(&GameState, Source, PlayerName) -> Option<bool> + 'static + Copy + Send + Sync
{
}

impl<F> PlayerPredicate for F where
    F: Fn(&GameState, Source, PlayerName) -> Option<bool> + 'static + Copy + Send + Sync
{
}

pub type PlayerPredicateFn =
    Box<dyn Fn(&GameState, Source, PlayerName) -> bool + 'static + Send + Sync>;

pub trait StackAbilityPredicate:
    Fn(&GameState, Source, StackAbilityId) -> Option<bool> + 'static + Copy + Send + Sync
{
}

impl<F> StackAbilityPredicate for F where
    F: Fn(&GameState, Source, StackAbilityId) -> Option<bool> + 'static + Copy + Send + Sync
{
}

pub type StackAbilityPredicateFn =
    Box<dyn Fn(&GameState, Source, StackAbilityId) -> Option<bool> + 'static + Send + Sync>;
