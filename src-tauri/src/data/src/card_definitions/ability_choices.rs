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
use crate::card_states::zones::{ToCardId, ZoneQueries};
use crate::core::function_types::{CardPredicateFn, PlayerPredicateFn, StackAbilityPredicateFn};
use crate::core::primitives::{CardId, EntityId, PermanentId, PlayerName, StackItemId, Zone};
use crate::delegates::scope::{EffectContext, Scope};
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
    /// Target permanents matching a predicate
    Permanent(AbilityTargetPermanent),

    /// Target a player matching a predicate from a set of players.
    Player(AbilityTargetPlayer),

    /// Target cards or players matching these predicates
    CardOrPlayer(PermanentOrPlayerAbilityTarget),

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

/// A target for an ability that targets a permanent.
pub struct AbilityTargetPermanent {
    /// Players whose cards should be searched
    pub players: PlayerSet,

    /// Predicate that must be satisfied by the selected permanent
    pub predicate: CardPredicateFn<PermanentId>,
}

pub struct AbilityTargetPlayer {
    pub players: EnumSet<PlayerName>,
    pub predicate: PlayerPredicateFn,
}

pub struct PermanentOrPlayerAbilityTarget {
    pub target_permanent: AbilityTargetPermanent,
    pub target_player: AbilityTargetPlayer,
}

#[derive(Clone, Copy, Eq, PartialEq, Debug)]
pub enum PermanentOrPlayer {
    Permanent(PermanentId),
    Player(PlayerName),
}

pub trait AbilityChoiceBuilder: Sized {
    #[doc(hidden)]
    fn get_choices_mut(&mut self) -> &mut AbilityChoices;

    #[doc(hidden)]
    fn set_effect_fn(&mut self, effect: EffectFn);

    fn effect(
        mut self,
        effect: impl Fn(&mut GameState, EffectContext) + 'static + Copy + Send + Sync,
    ) -> Self {
        self.set_effect_fn(Box::new(effect));
        self
    }

    /// Adds a single target permanent for this ability.
    ///
    /// The ID of the permanent that is targeted will be passed as a parameter
    /// to the effect function.
    fn target(
        mut self,
        target: impl Into<AbilityTargetPermanent>,
    ) -> EffectAbilityBuilder<PermanentId, Self> {
        self.get_choices_mut().targets.push(AbilityTarget {
            quantity: AbilityTargetQuantity::Exactly(1),
            predicate: AbilityTargetPredicate::Permanent(target.into()),
        });
        EffectAbilityBuilder { argument_builder: permanent_target_builder, builder: self }
    }

    /// Adds a single target player for this ability.
    ///
    /// The name of the chosen player will be passed as a parameter to the
    /// effect function.
    fn target_player(
        mut self,
        target: impl Into<AbilityTargetPlayer>,
    ) -> EffectAbilityBuilder<PlayerName, Self> {
        self.get_choices_mut().targets.push(AbilityTarget {
            quantity: AbilityTargetQuantity::Exactly(1),
            predicate: AbilityTargetPredicate::Player(target.into()),
        });
        EffectAbilityBuilder { argument_builder: player_target_builder, builder: self }
    }
}

pub struct EffectAbilityBuilder<TArg: 'static, TResult: AbilityChoiceBuilder> {
    pub argument_builder: fn(&GameState, EffectContext) -> Option<TArg>,
    pub builder: TResult,
}

impl<TArg: 'static, TResult: AbilityChoiceBuilder> EffectAbilityBuilder<TArg, TResult> {
    pub fn effect(
        mut self,
        effect: impl Fn(&mut GameState, EffectContext, TArg) + 'static + Copy + Send + Sync,
    ) -> TResult {
        self.builder.set_effect_fn(Box::new(move |game, scope| {
            if let Some(argument) = (self.argument_builder)(game, scope) {
                effect(game, scope, argument);
            }
        }));
        self.builder
    }
}

fn permanent_target_builder(game: &GameState, scope: EffectContext) -> Option<PermanentId> {
    game.card(*game.card(scope)?.targets.first()?)?.permanent_id()
}

fn player_target_builder(game: &GameState, scope: EffectContext) -> Option<PlayerName> {
    match game.card(scope)?.targets.first() {
        Some(EntityId::Player(player_name)) => Some(*player_name),
        _ => None,
    }
}
