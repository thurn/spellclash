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

use std::iter;
use std::iter::Empty;
use std::marker::PhantomData;

use enumset::EnumSet;
use utils::outcome::Outcome;

use crate::card_states::card_state::CardState;
use crate::card_states::zones::ZoneQueries;
use crate::core::primitives::{CardId, EntityId, PlayerName, Zone, ALL_ZONES};
use crate::costs::cost::Cost;
#[allow(unused)] // Used in docs
use crate::delegates::game_delegates::GameDelegates;
use crate::delegates::scope::{EffectContext, Scope};
use crate::game_states::game_state::GameState;
use crate::properties::card_properties::CardProperties;

/// An event callback function.
pub struct Delegate {
    /// [Zone]s in which this delegate should be active.
    pub zones: EnumSet<Zone>,

    /// Function to populate callbacks for this delegate
    pub run: Box<dyn Fn(&mut GameDelegates) + Send + Sync + 'static>,
}

/// Represents the possible types of ability
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AbilityType {
    Spell,
    Activated,
    Triggered,
    Static,
}

/// Function to apply the effects of of an ability to the game.
pub type EffectFn = Box<dyn Fn(&mut GameState, EffectContext) + 'static + Send + Sync>;

pub trait AbilityData: Sync + Send {
    /// Returns the type of this ability.
    fn get_ability_type(&self) -> AbilityType;

    /// Callback delegates for this ability.
    fn get_delegates(&self) -> &[Delegate];
}

pub trait Ability: AbilityData {
    /// Returns true if this ability requires targets.
    fn requires_targets(&self) -> bool;

    /// Returns an iterator over entities which could be targeted by this
    /// ability in the current game state. Returns an empty iterator if there
    /// are no valid targets or this is an untargeted ability that
    /// requires no targets.
    fn valid_targets<'a>(
        &'a self,
        game: &'a GameState,
        scope: Scope,
    ) -> Box<dyn Iterator<Item = EntityId> + 'a>;

    /// Invokes the effect of this ability.
    ///
    /// This is a no-op if invoked on an ability with no effect, like a static
    /// ability.
    fn invoke_effect(&self, game: &mut GameState, context: EffectContext);

    /// Invokes a delayed trigger effect associated with this ability.
    ///
    /// This applies the effect of a delayed trigger after it has been triggered
    /// and resolved. The `EffectId` in the provided [EffectContext] will be the
    /// ID from the effect which originally produced this delayed trigger.
    fn invoke_delayed_trigger_effect(&self, game: &mut GameState, context: EffectContext);
}

pub trait TargetSelector: Sync + Send {
    type Target;

    fn valid_targets<'a>(
        &'a self,
        game: &'a GameState,
        scope: Scope,
    ) -> Box<dyn Iterator<Item = EntityId> + 'a>;

    fn build_target_data(&self, game: &GameState, targets: &[EntityId]) -> Option<Self::Target>;
}

pub struct SpellAbility;

impl SpellAbility {
    #[allow(clippy::new_ret_no_self)]
    pub fn new() -> AbilityBuilder<NoEffect, DelayedTrigger<NoEffect>> {
        AbilityBuilder {
            ability_type: AbilityType::Spell,
            initialize: None,
            delegates: vec![],
            effect: NoEffect,
            delayed_trigger_effect: DelayedTrigger { delayed_trigger_effect: NoEffect },
        }
    }
}

pub struct TriggeredAbility;

impl TriggeredAbility {
    #[allow(clippy::new_ret_no_self)]
    pub fn new() -> AbilityBuilder<NoEffect, DelayedTrigger<NoEffect>> {
        AbilityBuilder {
            ability_type: AbilityType::Triggered,
            initialize: None,
            delegates: vec![],
            effect: NoEffect,
            delayed_trigger_effect: DelayedTrigger { delayed_trigger_effect: NoEffect },
        }
    }
}

pub struct StaticAbility;

impl StaticAbility {
    #[allow(clippy::new_ret_no_self)]
    pub fn new() -> AbilityBuilder<StaticEffect, DelayedTrigger<StaticEffect>> {
        AbilityBuilder {
            ability_type: AbilityType::Static,
            initialize: None,
            delegates: vec![],
            effect: StaticEffect,
            delayed_trigger_effect: DelayedTrigger { delayed_trigger_effect: StaticEffect },
        }
    }
}

pub struct NoEffect;

pub struct WithSelector<TSelector> {
    selector: TSelector,
}

pub struct UntargetedEffect<TFn>
where
    TFn: Fn(&mut GameState, EffectContext),
{
    function: TFn,
}

pub struct TargetedEffect<TSelector, TFn>
where
    TSelector: TargetSelector,
{
    selector: TSelector,
    function: TFn,
}

pub struct StaticEffect;

pub trait DelayedTriggerEffect {
    fn invoke(&self, game: &mut GameState, context: EffectContext);
}

pub struct AbilityBuilder<TEffect, TDelayed: DelayedTriggerEffect> {
    ability_type: AbilityType,

    initialize: Option<Box<dyn Fn(&mut CardProperties) + Send + Sync + 'static>>,

    delegates: Vec<Delegate>,

    effect: TEffect,

    delayed_trigger_effect: TDelayed,
}

impl<TEffect, TDelayed: DelayedTriggerEffect> AbilityBuilder<TEffect, TDelayed> {
    pub fn initialize(
        mut self,
        initialize: impl Fn(&mut CardProperties) + 'static + Copy + Send + Sync,
    ) -> Self {
        self.initialize = Some(Box::new(initialize));
        self
    }

    /// Adds new [Delegate]s to this ability which functions only for the
    /// default zones.
    ///
    /// For a static ability or triggered ability, these delegates will function
    /// only on the battlefield. For a spell ability or activated ability, these
    /// delegates will function in all zones.
    pub fn delegates(
        mut self,
        delegate: impl Fn(&mut GameDelegates) + 'static + Copy + Send + Sync,
    ) -> Self {
        let zones = match self.ability_type {
            AbilityType::Spell => ALL_ZONES,
            AbilityType::Activated => ALL_ZONES,
            AbilityType::Triggered => EnumSet::only(Zone::Battlefield),
            AbilityType::Static => EnumSet::only(Zone::Battlefield),
        };
        self.delegates.push(Delegate { zones, run: Box::new(delegate) });
        self
    }

    /// Adds new [Delegate]s to this ability which function in the provided
    /// set of [Zone]s.
    pub fn delegates_for(
        mut self,
        zones: impl Into<EnumSet<Zone>>,
        delegate: impl Fn(&mut GameDelegates) + 'static + Copy + Send + Sync,
    ) -> Self {
        self.delegates.push(Delegate { zones: zones.into(), run: Box::new(delegate) });
        self
    }

    /// Adds a new [DelayedTrigger] to this ability which can be fired after its
    /// main effect.
    pub fn delayed_trigger<TNew: DelayedTriggerEffect>(
        self,
        trigger: TNew,
    ) -> AbilityBuilder<TEffect, TNew> {
        AbilityBuilder {
            ability_type: self.ability_type,
            initialize: self.initialize,
            delegates: self.delegates,
            effect: self.effect,
            delayed_trigger_effect: trigger,
        }
    }
}

impl<TDelayed> AbilityBuilder<NoEffect, TDelayed>
where
    TDelayed: DelayedTriggerEffect,
{
    pub fn effect<TFn>(self, effect: TFn) -> AbilityBuilder<UntargetedEffect<TFn>, TDelayed>
    where
        TFn: Fn(&mut GameState, EffectContext) + 'static + Copy + Send + Sync,
    {
        AbilityBuilder {
            ability_type: self.ability_type,
            effect: UntargetedEffect { function: effect },
            initialize: self.initialize,
            delegates: self.delegates,
            delayed_trigger_effect: self.delayed_trigger_effect,
        }
    }

    pub fn targets<TSelector>(
        self,
        selector: TSelector,
    ) -> AbilityBuilder<WithSelector<TSelector>, TDelayed>
    where
        TSelector: TargetSelector,
    {
        AbilityBuilder {
            ability_type: self.ability_type,
            effect: WithSelector { selector },
            initialize: self.initialize,
            delegates: self.delegates,
            delayed_trigger_effect: self.delayed_trigger_effect,
        }
    }
}

impl<TSelector, TDelayed> AbilityBuilder<WithSelector<TSelector>, TDelayed>
where
    TSelector: TargetSelector,
    TDelayed: DelayedTriggerEffect,
{
    pub fn effect<TTarget, TFn>(
        self,
        effect: TFn,
    ) -> AbilityBuilder<TargetedEffect<TSelector, TFn>, TDelayed>
    where
        TFn: Fn(&mut GameState, EffectContext, TTarget) + 'static + Copy + Send + Sync,
    {
        AbilityBuilder {
            ability_type: self.ability_type,
            effect: TargetedEffect { selector: self.effect.selector, function: effect },
            initialize: self.initialize,
            delegates: self.delegates,
            delayed_trigger_effect: self.delayed_trigger_effect,
        }
    }
}

impl<TEffect, TDelayed> AbilityData for AbilityBuilder<TEffect, TDelayed>
where
    TEffect: Sync + Send,
    TDelayed: Sync + Send + DelayedTriggerEffect,
{
    #[doc(hidden)]
    fn get_ability_type(&self) -> AbilityType {
        self.ability_type
    }

    #[doc(hidden)]
    fn get_delegates(&self) -> &[Delegate] {
        &self.delegates
    }
}

impl<TFn, TDelayed> Ability for AbilityBuilder<UntargetedEffect<TFn>, TDelayed>
where
    TFn: Fn(&mut GameState, EffectContext) + 'static + Copy + Send + Sync,
    TDelayed: DelayedTriggerEffect + 'static + Send + Sync,
{
    #[doc(hidden)]
    fn requires_targets(&self) -> bool {
        false
    }

    #[doc(hidden)]
    fn valid_targets(&self, game: &GameState, scope: Scope) -> Box<dyn Iterator<Item = EntityId>> {
        Box::new(iter::empty())
    }

    #[doc(hidden)]
    fn invoke_effect(&self, game: &mut GameState, context: EffectContext) {
        (self.effect.function)(game, context)
    }

    #[doc(hidden)]
    fn invoke_delayed_trigger_effect(&self, game: &mut GameState, context: EffectContext) {
        self.delayed_trigger_effect.invoke(game, context);
    }
}

impl<TSelector, TFn, TDelayed> Ability for AbilityBuilder<TargetedEffect<TSelector, TFn>, TDelayed>
where
    TSelector: TargetSelector,
    TFn: Fn(&mut GameState, EffectContext, TSelector::Target) + 'static + Copy + Send + Sync,
    TDelayed: DelayedTriggerEffect + 'static + Send + Sync,
{
    #[doc(hidden)]
    fn requires_targets(&self) -> bool {
        true
    }

    #[doc(hidden)]
    fn valid_targets<'a>(
        &'a self,
        game: &'a GameState,
        scope: Scope,
    ) -> Box<dyn Iterator<Item = EntityId> + 'a> {
        self.effect.selector.valid_targets(game, scope)
    }

    #[doc(hidden)]
    fn invoke_effect(&self, game: &mut GameState, context: EffectContext) {
        let Some(targets) = game.card(context).map(|c| &c.targets) else {
            return;
        };

        if let Some(data) = self.effect.selector.build_target_data(game, targets) {
            (self.effect.function)(game, context, data);
        }
    }

    #[doc(hidden)]
    fn invoke_delayed_trigger_effect(&self, game: &mut GameState, context: EffectContext) {
        self.delayed_trigger_effect.invoke(game, context);
    }
}

impl<TDelayed> Ability for AbilityBuilder<StaticEffect, TDelayed>
where
    TDelayed: DelayedTriggerEffect + 'static + Send + Sync,
{
    #[doc(hidden)]
    fn requires_targets(&self) -> bool {
        false
    }

    #[doc(hidden)]
    fn valid_targets(&self, game: &GameState, scope: Scope) -> Box<dyn Iterator<Item = EntityId>> {
        Box::new(iter::empty())
    }

    #[doc(hidden)]
    fn invoke_effect(&self, game: &mut GameState, context: EffectContext) {}

    #[doc(hidden)]
    fn invoke_delayed_trigger_effect(&self, game: &mut GameState, context: EffectContext) {
        self.delayed_trigger_effect.invoke(game, context);
    }
}

pub struct DelayedTrigger<TDelayed> {
    delayed_trigger_effect: TDelayed,
}

impl DelayedTrigger<NoEffect> {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        DelayedTrigger { delayed_trigger_effect: NoEffect }
    }

    pub fn effect<TFn>(self, effect: TFn) -> DelayedTrigger<UntargetedEffect<TFn>>
    where
        TFn: Fn(&mut GameState, EffectContext) + 'static + Copy + Send + Sync,
    {
        DelayedTrigger { delayed_trigger_effect: UntargetedEffect { function: effect } }
    }

    pub fn targets<TSelector>(self, selector: TSelector) -> DelayedTrigger<WithSelector<TSelector>>
    where
        TSelector: TargetSelector,
    {
        DelayedTrigger { delayed_trigger_effect: WithSelector { selector } }
    }
}

impl<TSelector> DelayedTrigger<WithSelector<TSelector>>
where
    TSelector: TargetSelector,
{
    pub fn effect<TTarget, TFn>(self, effect: TFn) -> DelayedTrigger<TargetedEffect<TSelector, TFn>>
    where
        TFn: Fn(&mut GameState, EffectContext, TTarget) + 'static + Copy + Send + Sync,
    {
        DelayedTrigger {
            delayed_trigger_effect: TargetedEffect {
                selector: self.delayed_trigger_effect.selector,
                function: effect,
            },
        }
    }
}

impl DelayedTriggerEffect for DelayedTrigger<NoEffect> {
    fn invoke(&self, game: &mut GameState, context: EffectContext) {}
}

impl<TFn> DelayedTriggerEffect for DelayedTrigger<UntargetedEffect<TFn>>
where
    TFn: Fn(&mut GameState, EffectContext) + 'static + Copy + Send + Sync,
{
    #[doc(hidden)]
    fn invoke(&self, game: &mut GameState, context: EffectContext) {
        (self.delayed_trigger_effect.function)(game, context)
    }
}

impl<TSelector, TFn> DelayedTriggerEffect for DelayedTrigger<TargetedEffect<TSelector, TFn>>
where
    TSelector: TargetSelector,
    TFn: Fn(&mut GameState, EffectContext, TSelector::Target) + 'static + Copy + Send + Sync,
{
    #[doc(hidden)]
    fn invoke(&self, game: &mut GameState, context: EffectContext) {
        let Some(targets) = game.card(context).map(|c| &c.targets) else {
            return;
        };

        if let Some(data) = self.delayed_trigger_effect.selector.build_target_data(game, targets) {
            (self.delayed_trigger_effect.function)(game, context, data);
        }
    }
}

impl DelayedTriggerEffect for DelayedTrigger<StaticEffect> {
    fn invoke(&self, game: &mut GameState, context: EffectContext) {}
}
