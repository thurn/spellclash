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

use utils::outcome::Outcome;

use crate::core::primitives::{CardId, EntityId, PlayerName, Source, StackAbilityId};
use crate::delegates::scope::Scope;
use crate::game_states::game_state::GameState;

/// Function which performs a boolean query on the state of a card.
pub trait CardPredicate:
    Fn(&GameState, Scope, CardId) -> bool + 'static + Copy + Send + Sync
{
}

impl<F> CardPredicate for F where
    F: Fn(&GameState, Scope, CardId) -> bool + 'static + Copy + Send + Sync
{
}

pub type CardPredicateFn = Box<dyn Fn(&GameState, Scope, CardId) -> bool + 'static + Send + Sync>;

/// Function which performs a boolean query on the state of a card.
pub trait CardMutation:
    Fn(&mut GameState, Source, CardId) -> Outcome + 'static + Copy + Send + Sync
{
}

impl<F> CardMutation for F where
    F: Fn(&mut GameState, Source, CardId) -> Outcome + 'static + Copy + Send + Sync
{
}

pub type CardMutationFn =
    Box<dyn Fn(&mut GameState, Source, CardId) -> Outcome + 'static + Send + Sync>;

pub trait PlayerPredicate:
    Fn(&GameState, Scope, PlayerName) -> bool + 'static + Copy + Send + Sync
{
}

impl<F> PlayerPredicate for F where
    F: Fn(&GameState, Scope, PlayerName) -> bool + 'static + Copy + Send + Sync
{
}

pub type PlayerPredicateFn =
    Box<dyn Fn(&GameState, Scope, PlayerName) -> bool + 'static + Send + Sync>;

pub trait StackAbilityPredicate:
    Fn(&GameState, Scope, StackAbilityId) -> bool + 'static + Copy + Send + Sync
{
}

impl<F> StackAbilityPredicate for F where
    F: Fn(&GameState, Scope, StackAbilityId) -> bool + 'static + Copy + Send + Sync
{
}

pub type StackAbilityPredicateFn =
    Box<dyn Fn(&GameState, Scope, StackAbilityId) -> bool + 'static + Send + Sync>;
