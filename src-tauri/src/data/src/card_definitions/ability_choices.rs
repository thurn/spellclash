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
use utils::outcome;
use utils::outcome::Outcome;

use crate::card_definitions::ability_definition::EffectFn;
use crate::card_states::card_state::CardState;
use crate::card_states::zones::ZoneQueries;
use crate::core::function_types::{CardPredicateFn, PlayerPredicateFn, StackAbilityPredicateFn};
use crate::core::primitives::{CardId, EntityId, HasCardId, PlayerName, StackItemId, Zone};
use crate::delegates::scope::{DelegateScope, EffectScope};
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
#[derive(Clone, Copy, Eq, PartialEq, Debug)]
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

#[derive(Clone, Copy, Eq, PartialEq, Debug)]
pub enum PlayerSet {
    /// Search the set of all players in the game
    AllPlayers,

    /// Search the cards controlled by this ability's controller
    You,

    /// Search the set of opponents of this ability's controller
    Opponents,
}

/// A target for an ability that targets a card.
pub struct CardAbilityTarget {
    /// Zones to search for the target card
    pub zones: EnumSet<Zone>,

    /// Players whose cards should be searched
    pub players: PlayerSet,

    /// Predicate that must be satisfied by the selected card
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

    #[doc(hidden)]
    fn set_effect_fn(&mut self, effect: EffectFn);

    fn effect(
        mut self,
        effect: impl Fn(&mut GameState, EffectScope) -> Outcome + 'static + Copy + Send + Sync,
    ) -> Self {
        self.set_effect_fn(Box::new(effect));
        self
    }

    /// Adds a single target card for this ability.
    ///
    /// The ID of the card that is targeted will be passed as a parameter to the
    /// effect function.
    fn target(
        mut self,
        target: impl Into<CardAbilityTarget>,
    ) -> EffectAbilityBuilder<CardId, Self> {
        self.get_choices_mut().targets.push(AbilityTarget {
            quantity: AbilityTargetQuantity::Exactly(1),
            predicate: AbilityTargetPredicate::Card(target.into()),
        });
        EffectAbilityBuilder { argument_builder: card_target_builder, builder: self }
    }

    /// Adds a single target player for this ability.
    ///
    /// The name of the chosen player will be passed as a parameter to the
    /// effect function.
    fn target_player(
        mut self,
        target: impl Into<PlayerAbilityTarget>,
    ) -> EffectAbilityBuilder<PlayerName, Self> {
        self.get_choices_mut().targets.push(AbilityTarget {
            quantity: AbilityTargetQuantity::Exactly(1),
            predicate: AbilityTargetPredicate::Player(target.into()),
        });
        EffectAbilityBuilder { argument_builder: player_target_builder, builder: self }
    }
}

pub struct EffectAbilityBuilder<TArg: 'static, TResult: AbilityChoiceBuilder> {
    pub argument_builder: fn(&GameState, EffectScope) -> Option<TArg>,
    pub builder: TResult,
}

impl<TArg: 'static, TResult: AbilityChoiceBuilder> EffectAbilityBuilder<TArg, TResult> {
    pub fn effect(
        mut self,
        effect: impl Fn(&mut GameState, EffectScope, TArg) -> Outcome + 'static + Copy + Send + Sync,
    ) -> TResult {
        self.builder.set_effect_fn(Box::new(move |game, scope| {
            if let Some(argument) = (self.argument_builder)(game, scope) {
                effect(game, scope, argument)?;
            }
            outcome::OK
        }));
        self.builder
    }
}

fn card_target_builder(game: &GameState, scope: EffectScope) -> Option<CardId> {
    match game.card(scope.card_id()).targets.first() {
        Some(EntityId::Card(card_id, _)) => Some(*card_id),
        _ => None,
    }
}

fn player_target_builder(game: &GameState, scope: EffectScope) -> Option<PlayerName> {
    match game.card(scope.card_id()).targets.first() {
        Some(EntityId::Player(player_name)) => Some(*player_name),
        _ => None,
    }
}