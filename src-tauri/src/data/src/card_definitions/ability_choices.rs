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

use enumset::EnumSet;

use crate::core::function_types::{CardPredicateFn, PlayerPredicateFn, StackAbilityPredicateFn};
use crate::core::primitives::{EntityId, PlayerName, StackItemId, Zone};
use crate::delegates::scope::Scope;
use crate::game_states::game_state::GameState;

/// Set of choices available to be made when placing an ability on the stack.
#[derive(Default)]
pub struct AbilityChoices {
    /// Target selector predicates for this ability
    pub targets: Vec<AbilityTarget>,
}

/// Represents a single instance of the word 'target' in an ability's oracle
/// text.
pub struct AbilityTarget {
    /// The number of objects that can be selected as targets
    pub quantity: AbilityTargetQuantity,

    /// The predicate that must be satisfied by the selected targets.
    pub predicate: AbilityTargetPredicate,
}

/// Number of matching objects that can be selected as targets for one
/// predicate.
pub enum AbilityTargetQuantity {
    AnyNumber,
    Exactly(usize),
    UpTo(usize),
}

/// Possible restrictions on the targets of an ability
pub enum AbilityTargetPredicate {
    /// Target a player matching a predicate from a set of players.
    Player(EnumSet<PlayerName>, PlayerPredicateFn),

    /// Target cards matching a predicate in a set of zones.
    Card(EnumSet<Zone>, CardPredicateFn),

    /// Target an ability on the stack matching a predicate
    StackAbility(StackAbilityPredicateFn),

    /// Target may satisfy any of the provided list of predicates.
    AnyOf(Vec<AbilityTargetPredicate>),
}

pub trait AbilityChoiceBuilder: Sized {
    #[doc(hidden)]
    fn get_choices_mut(&mut self) -> &mut AbilityChoices;

    /// Add a target for this ability.
    ///
    /// This method should be called once for each instance of the word 'target'
    /// in the ability's oracle text.
    fn target(mut self, target: impl Into<AbilityTarget>) -> Self {
        self.get_choices_mut().targets.push(target.into());
        self
    }
}
