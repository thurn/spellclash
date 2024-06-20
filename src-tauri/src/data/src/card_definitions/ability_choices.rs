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
use utils::outcome::Outcome;

use crate::core::function_types::{CardPredicateFn, PlayerPredicateFn, StackAbilityPredicateFn};
use crate::core::primitives::{CardId, EntityId, PlayerName, StackItemId, Zone};
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
    /// Target cards matching a predicate in a set of zones.
    Card(CardAbilityTarget),

    /// Target a player matching a predicate from a set of players.
    Player(PlayerAbilityTarget),

    /// Target cards or players matching these predicates
    CardOrPlayer(CardOrPlayerAbilityTarget),

    /// Target an ability on the stack matching a predicate
    StackAbility(StackAbilityPredicateFn),

    /// Target may satisfy any of the provided list of predicates.
    AnyOf(Vec<AbilityTargetPredicate>),
}

pub struct CardAbilityTarget {
    pub zones: EnumSet<Zone>,
    pub predicate: CardPredicateFn,
}

pub struct PlayerAbilityTarget {
    pub players: EnumSet<PlayerName>,
    pub predicate: PlayerPredicateFn,
}

pub struct CardOrPlayerAbilityTarget {
    pub card_target: CardAbilityTarget,
    pub player_target: PlayerAbilityTarget,
}

#[derive(Clone, Copy, Eq, PartialEq, Debug)]
pub enum CardOrPlayer {
    Card(CardId),
    Player(PlayerName),
}

pub trait AbilityChoiceBuilder: Sized {
    #[doc(hidden)]
    fn get_choices_mut(&mut self) -> &mut AbilityChoices;

    /// Adds a single target card for this ability.
    ///
    /// The ID of the card that is targeted will be passed as a parameter to the
    /// effect function.
    fn target_card(
        mut self,
        target: impl Into<CardAbilityTarget>,
    ) -> SingleCardTargetAbilityBuilder<Self> {
        self.get_choices_mut().targets.push(AbilityTarget {
            quantity: AbilityTargetQuantity::Exactly(1),
            predicate: AbilityTargetPredicate::Card(target.into()),
        });
        SingleCardTargetAbilityBuilder { builder: self }
    }

    /// Adds a single target player for this ability.
    ///
    /// The name of the chosen player will be passed as a parameter to the
    /// effect function.
    fn target_player(
        mut self,
        target: impl Into<PlayerAbilityTarget>,
    ) -> SinglePlayerTargetAbilityBuilder<Self> {
        self.get_choices_mut().targets.push(AbilityTarget {
            quantity: AbilityTargetQuantity::Exactly(1),
            predicate: AbilityTargetPredicate::Player(target.into()),
        });
        SinglePlayerTargetAbilityBuilder { builder: self }
    }

    /// Adds a single target card or player for this ability.
    ///
    /// The ID of the chosen card or name of the chosen player will be passed as
    /// a parameter to the effect function.
    fn target_card_or_player(
        mut self,
        target: impl Into<CardOrPlayerAbilityTarget>,
    ) -> SingleCardOrPlayerTargetAbilityBuilder<Self> {
        self.get_choices_mut().targets.push(AbilityTarget {
            quantity: AbilityTargetQuantity::Exactly(1),
            predicate: AbilityTargetPredicate::CardOrPlayer(target.into()),
        });
        SingleCardOrPlayerTargetAbilityBuilder { builder: self }
    }
}

pub struct SingleCardTargetAbilityBuilder<TResult: AbilityChoiceBuilder> {
    pub builder: TResult,
}

impl<TResult: AbilityChoiceBuilder> SingleCardTargetAbilityBuilder<TResult> {
    pub fn effect(
        self,
        effect: impl Fn(&mut GameState, Scope, CardId) -> Outcome + 'static + Copy + Send + Sync,
    ) -> TResult {
        self.builder
    }
}

pub struct SinglePlayerTargetAbilityBuilder<TResult: AbilityChoiceBuilder> {
    pub builder: TResult,
}

impl<TResult: AbilityChoiceBuilder> SinglePlayerTargetAbilityBuilder<TResult> {
    pub fn effect(
        self,
        effect: impl Fn(&mut GameState, Scope, PlayerName) -> Outcome + 'static + Copy + Send + Sync,
    ) -> TResult {
        self.builder
    }
}

pub struct SingleCardOrPlayerTargetAbilityBuilder<TResult: AbilityChoiceBuilder> {
    pub builder: TResult,
}

impl<TResult: AbilityChoiceBuilder> SingleCardOrPlayerTargetAbilityBuilder<TResult> {
    pub fn effect(
        self,
        effect: impl Fn(&mut GameState, Scope, CardOrPlayer) -> Outcome + 'static + Copy + Send + Sync,
    ) -> TResult {
        self.builder
    }
}
