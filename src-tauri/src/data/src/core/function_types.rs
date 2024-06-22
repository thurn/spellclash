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

use crate::core::primitives::{CardId, EntityId, PermanentId, PlayerName, Source, StackAbilityId};
use crate::delegates::scope::DelegateScope;
use crate::game_states::game_state::GameState;

/// Function which performs a boolean query on the state of a permanent.
pub trait PermanentPredicate:
    Fn(&GameState, DelegateScope, PermanentId) -> bool + 'static + Copy + Send + Sync
{
}

impl<F> PermanentPredicate for F where
    F: Fn(&GameState, DelegateScope, PermanentId) -> bool + 'static + Copy + Send + Sync
{
}

pub type PermanentPredicateFn =
    Box<dyn Fn(&GameState, DelegateScope, PermanentId) -> bool + 'static + Send + Sync>;

/// Function which performs a boolean query on the state of a card.
pub trait CardPredicate:
    Fn(&GameState, DelegateScope, CardId) -> bool + 'static + Copy + Send + Sync
{
}

impl<F> CardPredicate for F where
    F: Fn(&GameState, DelegateScope, CardId) -> bool + 'static + Copy + Send + Sync
{
}

pub type CardPredicateFn =
    Box<dyn Fn(&GameState, DelegateScope, CardId) -> bool + 'static + Send + Sync>;

pub trait PermanentMutation:
    Fn(&mut GameState, Source, PermanentId) -> Outcome + 'static + Copy + Send + Sync
{
}

impl<F> PermanentMutation for F where
    F: Fn(&mut GameState, Source, PermanentId) -> Outcome + 'static + Copy + Send + Sync
{
}

pub type PermanentMutationFn =
    Box<dyn Fn(&mut GameState, Source, PermanentId) -> Outcome + 'static + Send + Sync>;

/// Function which performs a mutation on the state of a card.
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
    Fn(&GameState, DelegateScope, PlayerName) -> bool + 'static + Copy + Send + Sync
{
}

impl<F> PlayerPredicate for F where
    F: Fn(&GameState, DelegateScope, PlayerName) -> bool + 'static + Copy + Send + Sync
{
}

pub type PlayerPredicateFn =
    Box<dyn Fn(&GameState, DelegateScope, PlayerName) -> bool + 'static + Send + Sync>;

pub trait StackAbilityPredicate:
    Fn(&GameState, DelegateScope, StackAbilityId) -> bool + 'static + Copy + Send + Sync
{
}

impl<F> StackAbilityPredicate for F where
    F: Fn(&GameState, DelegateScope, StackAbilityId) -> bool + 'static + Copy + Send + Sync
{
}

pub type StackAbilityPredicateFn =
    Box<dyn Fn(&GameState, DelegateScope, StackAbilityId) -> bool + 'static + Send + Sync>;
