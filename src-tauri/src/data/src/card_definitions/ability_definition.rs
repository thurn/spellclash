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

use primitives::game_primitives::{EntityId, PlayerName, Source};

use crate::card_definitions::modal_effect::ModalEffect;
use crate::card_states::card_state::CardState;
use crate::card_states::play_card_plan::PlayCardChoices;
use crate::card_states::zones::ZoneQueries;
use crate::core::ability_scope::AbilityScope;
use crate::events::card_events::CardEvents;
use crate::events::event_context::EventContext;
use crate::events::game_events::GlobalEvents;
use crate::game_states::game_state::GameState;
use crate::properties::card_properties::CardProperties;

/// Represents the possible types of ability
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AbilityType {
    Spell,
    Activated,
    Triggered,
    Static,
}

pub trait AbilityData: Sync + Send {
    /// Creates the initial property state of this ability on a card.
    fn add_properties(&self, scope: AbilityScope, card: &mut CardState);

    /// Creates the initial event callbacks for this ability.
    fn add_global_events(&self, scope: AbilityScope, events: &mut GlobalEvents);

    /// Creates the initial event callbacks for this ability.
    fn add_card_events(&self, scope: AbilityScope, events: &mut CardEvents);

    /// Returns the type of this ability.
    fn get_ability_type(&self) -> AbilityType;
}

pub trait Ability: AbilityData {
    /// Returns true if this ability could require targets to be chosen.
    ///
    /// This will return true even if e.g. targets are part of an additional
    /// cost or only required in a certain mode.
    fn requires_targets(&self) -> bool;

    /// Returns an iterator over entities which could be targeted by this
    /// ability in the current game state, given a set of [PlayCardChoices].
    ///
    /// Returns an empty iterator if there are no valid targets or this is an
    /// untargeted ability that requires no targets.
    fn valid_targets<'a>(
        &'a self,
        game: &'a GameState,
        choices: &'a PlayCardChoices,
        source: Source,
    ) -> Box<dyn Iterator<Item = EntityId> + 'a>;

    /// Invokes the effect of this ability, given a set of [PlayCardChoices].
    ///
    /// This is a no-op if invoked on an ability with no effect, like a static
    /// ability.
    fn invoke_effect(
        &self,
        game: &mut GameState,
        context: EventContext,
        choices: &Option<PlayCardChoices>,
    );
}

pub trait TargetSelector: Sync + Send {
    type Target;

    fn valid_targets<'a>(
        &'a self,
        game: &'a GameState,
        choices: &'a PlayCardChoices,
        source: Source,
    ) -> Box<dyn Iterator<Item = EntityId> + 'a>;

    fn build_target_data(&self, game: &GameState, targets: &[EntityId]) -> Option<Self::Target>;
}

pub struct SpellAbility;

impl SpellAbility {
    #[allow(clippy::new_ret_no_self)]
    pub fn new() -> AbilityBuilder<NoEffect> {
        AbilityBuilder {
            ability_type: AbilityType::Spell,
            properties: None,
            global_events: None,
            card_events: None,
            effect: NoEffect,
        }
    }
}

pub struct TriggeredAbility;

impl TriggeredAbility {
    #[allow(clippy::new_ret_no_self)]
    pub fn new() -> AbilityBuilder<NoEffect> {
        AbilityBuilder {
            ability_type: AbilityType::Triggered,
            properties: None,
            global_events: None,
            card_events: None,
            effect: NoEffect,
        }
    }
}

pub struct StaticAbility;

impl StaticAbility {
    #[allow(clippy::new_ret_no_self)]
    pub fn new() -> AbilityBuilder<StaticEffect> {
        AbilityBuilder {
            ability_type: AbilityType::Static,
            properties: None,
            global_events: None,
            card_events: None,
            effect: StaticEffect,
        }
    }
}

pub struct NoEffect;

pub struct WithSelector<TSelector> {
    selector: TSelector,
}

pub struct UntargetedEffect<TFn>
where
    TFn: Fn(&mut GameState, EventContext) + 'static + Send + Sync + Clone,
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

pub type PropertiesFn = Box<dyn Fn(AbilityScope, &mut CardProperties) + Send + Sync + 'static>;

pub type GlobalEventsFn = Box<dyn Fn(AbilityScope, &mut GlobalEvents) + Send + Sync + 'static>;

pub type CardEventsFn = Box<dyn Fn(AbilityScope, &mut CardEvents) + Send + Sync + 'static>;

pub struct AbilityBuilder<TEffect> {
    pub(crate) ability_type: AbilityType,

    pub(crate) properties: Option<PropertiesFn>,

    pub(crate) global_events: Option<GlobalEventsFn>,

    pub(crate) card_events: Option<CardEventsFn>,

    pub(crate) effect: TEffect,
}

impl<TEffect> AbilityBuilder<TEffect> {
    pub fn properties(
        mut self,
        initialize: impl Fn(AbilityScope, &mut CardProperties) + 'static + Copy + Send + Sync,
    ) -> Self {
        self.properties = Some(Box::new(initialize));
        self
    }

    pub fn events(
        mut self,
        initialize: impl Fn(AbilityScope, &mut CardEvents) + 'static + Copy + Send + Sync,
    ) -> Self {
        self.card_events = Some(Box::new(initialize));
        self
    }

    pub fn global_events(
        mut self,
        initialize: impl Fn(AbilityScope, &mut GlobalEvents) + 'static + Copy + Send + Sync,
    ) -> Self {
        self.global_events = Some(Box::new(initialize));
        self
    }
}

impl AbilityBuilder<NoEffect> {
    pub fn effect<TFn>(self, effect: TFn) -> AbilityBuilder<UntargetedEffect<TFn>>
    where
        TFn: Fn(&mut GameState, EventContext) + 'static + Clone + Send + Sync,
    {
        AbilityBuilder {
            ability_type: self.ability_type,
            effect: UntargetedEffect { function: effect },
            properties: self.properties,
            global_events: self.global_events,
            card_events: self.card_events,
        }
    }

    pub fn modal_effect(self, effect: ModalEffect) -> AbilityBuilder<ModalEffect> {
        AbilityBuilder {
            ability_type: self.ability_type,
            effect,
            properties: self.properties,
            global_events: self.global_events,
            card_events: self.card_events,
        }
    }

    pub fn targets<TSelector>(self, selector: TSelector) -> AbilityBuilder<WithSelector<TSelector>>
    where
        TSelector: TargetSelector,
    {
        AbilityBuilder {
            ability_type: self.ability_type,
            effect: WithSelector { selector },
            properties: self.properties,
            global_events: self.global_events,
            card_events: self.card_events,
        }
    }
}

impl<TSelector> AbilityBuilder<WithSelector<TSelector>>
where
    TSelector: TargetSelector,
{
    pub fn effect<TTarget, TFn>(self, effect: TFn) -> AbilityBuilder<TargetedEffect<TSelector, TFn>>
    where
        TFn: Fn(&mut GameState, EventContext, TTarget) + 'static + Send + Sync,
    {
        AbilityBuilder {
            ability_type: self.ability_type,
            effect: TargetedEffect { selector: self.effect.selector, function: effect },
            properties: self.properties,
            global_events: self.global_events,
            card_events: self.card_events,
        }
    }
}

impl<TEffect> AbilityData for AbilityBuilder<TEffect>
where
    TEffect: Sync + Send,
{
    fn add_properties(&self, scope: AbilityScope, card: &mut CardState) {
        if let Some(f) = self.properties.as_ref() {
            f(scope, &mut card.properties)
        }
    }

    fn add_global_events(&self, scope: AbilityScope, card: &mut GlobalEvents) {
        if let Some(f) = self.global_events.as_ref() {
            f(scope, card)
        }
    }

    fn add_card_events(&self, scope: AbilityScope, card: &mut CardEvents) {
        if let Some(f) = self.card_events.as_ref() {
            f(scope, card)
        }
    }

    #[doc(hidden)]
    fn get_ability_type(&self) -> AbilityType {
        self.ability_type
    }
}

impl<TFn> Ability for AbilityBuilder<UntargetedEffect<TFn>>
where
    TFn: Fn(&mut GameState, EventContext) + 'static + Clone + Send + Sync,
{
    #[doc(hidden)]
    fn requires_targets(&self) -> bool {
        false
    }

    #[doc(hidden)]
    fn valid_targets(
        &self,
        game: &GameState,
        choices: &PlayCardChoices,
        source: Source,
    ) -> Box<dyn Iterator<Item = EntityId>> {
        Box::new(iter::empty())
    }

    #[doc(hidden)]
    fn invoke_effect(
        &self,
        game: &mut GameState,
        context: EventContext,
        _: &Option<PlayCardChoices>,
    ) {
        (self.effect.function)(game, context)
    }
}

impl<TSelector, TFn> Ability for AbilityBuilder<TargetedEffect<TSelector, TFn>>
where
    TSelector: TargetSelector,
    TFn: Fn(&mut GameState, EventContext, TSelector::Target) + 'static + Clone + Send + Sync,
{
    #[doc(hidden)]
    fn requires_targets(&self) -> bool {
        true
    }

    #[doc(hidden)]
    fn valid_targets<'a>(
        &'a self,
        game: &'a GameState,
        choices: &'a PlayCardChoices,
        source: Source,
    ) -> Box<dyn Iterator<Item = EntityId> + 'a> {
        self.effect.selector.valid_targets(game, choices, source)
    }

    #[doc(hidden)]
    fn invoke_effect(
        &self,
        game: &mut GameState,
        context: EventContext,
        _: &Option<PlayCardChoices>,
    ) {
        let Some(targets) = game.card(context.this).map(|c| &c.targets) else {
            return;
        };

        if let Some(data) = self.effect.selector.build_target_data(game, targets) {
            (self.effect.function)(game, context, data);
        }
    }
}

impl Ability for AbilityBuilder<StaticEffect> {
    #[doc(hidden)]
    fn requires_targets(&self) -> bool {
        false
    }

    #[doc(hidden)]
    fn valid_targets(
        &self,
        game: &GameState,
        choices: &PlayCardChoices,
        source: Source,
    ) -> Box<dyn Iterator<Item = EntityId>> {
        Box::new(iter::empty())
    }

    #[doc(hidden)]
    fn invoke_effect(
        &self,
        game: &mut GameState,
        context: EventContext,
        _: &Option<PlayCardChoices>,
    ) {
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
        TFn: Fn(&mut GameState, EventContext) + 'static + Clone + Send + Sync,
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
        TFn: Fn(&mut GameState, EventContext, TTarget) + 'static + Clone + Send + Sync,
    {
        DelayedTrigger {
            delayed_trigger_effect: TargetedEffect {
                selector: self.delayed_trigger_effect.selector,
                function: effect,
            },
        }
    }
}
