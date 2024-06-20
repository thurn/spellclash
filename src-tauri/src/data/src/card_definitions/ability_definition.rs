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

use std::marker::PhantomData;

use enumset::EnumSet;
use utils::outcome;
use utils::outcome::Outcome;

use crate::core::primitives::Zone;
use crate::costs::cost::Cost;
use crate::delegates::event_delegate_list::EventDelegateList;
#[allow(unused)] // Used in docs
use crate::delegates::game_delegates::GameDelegates;
use crate::delegates::scope::Scope;
use crate::game_states::game_state::GameState;

/// A predicate to apply to a delegate activation.
pub type RequirementFn = fn(&GameState, Scope) -> bool;

pub struct Delegate {
    /// [Zone]s in which this delegate should be active.
    pub zones: EnumSet<Zone>,

    /// Function to populate callbacks for this delegate
    pub run: Box<dyn Fn(&mut GameDelegates) + Send + Sync + 'static>,
}

pub type EffectFn = Box<dyn Fn(&mut GameState, Scope) -> Outcome + 'static + Send + Sync>;

/// Defines the game rules for an ability.
///
/// Each ability for a card should be defined sequentially in the same order
/// in which they appear in that card's oracle text_strings, as this will be
/// used to generate text_strings to terminal_ui in-game.
///
/// This struct should never be instantiated directly. Always use one of the
/// builders defined in this file instead.
pub struct AbilityDefinition {
    /// Type of ability
    pub ability_type: AbilityType,
    /// Effect of this ability when it is resolved.
    ///
    /// Note that static abilities do not resolve via the stack and thus have no
    /// effects.
    pub effect: Option<EffectFn>,
    /// Event listeners for this ability
    pub delegates: Vec<Delegate>,
    /// Costs to activate an activated ability
    pub costs: Vec<Cost>,
}

pub trait AbilityBuilder {
    /// Create a new [AbilityDefinition].
    fn build(self) -> AbilityDefinition;
}

/// Restriction on when an [AbilityRequirement] is checked.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum RequirementTiming {
    /// Requirement is checked before putting ability on the stack
    OnCreation,

    /// Requirement is checked both before putting ability on the stack and
    /// before resolving ability. This is known as an "intervening if" check.
    ///
    /// See <https://yawgatog.com/resources/magic-rules/#R6034>
    OnCreationAndResolution,
}

/// A required for an ability to fire
#[derive(Clone, Debug)]
pub struct AbilityRequirement {
    pub predicate: RequirementFn,
    pub timing: RequirementTiming,
}

pub struct NoEffects;
pub struct WithEffects(pub EffectFn);

/// Builder for spell abilities.
///
/// > 113.3a. Spell abilities are abilities that are followed as instructions
/// > while an instant or sorcery spell is resolving. Any text on an instant or
/// > sorcery spell is a spell ability unless it's an activated ability, a
/// > triggered ability, or a static ability that fits the criteria described in
/// > rule 113.6.
///
/// <https://yawgatog.com/resources/magic-rules/#R1133a>
pub struct SpellAbility<TEffects> {
    effects: TEffects,
    delegates: Vec<Delegate>,
}

impl SpellAbility<NoEffects> {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        Self { effects: NoEffects, delegates: vec![] }
    }

    /// Effects when this spell resolves.
    pub fn effects(
        self,
        effect: impl Fn(&mut GameState, Scope) -> Outcome + 'static + Copy + Send + Sync,
    ) -> SpellAbility<WithEffects> {
        SpellAbility { effects: WithEffects(Box::new(effect)), delegates: self.delegates }
    }
}

impl SpellAbility<WithEffects> {
    /// Adds a new delegate creation function to this ability. See
    /// [GameDelegates] for more information.
    pub fn delegate(
        mut self,
        zones: impl Into<EnumSet<Zone>>,
        delegate: impl Fn(&mut GameDelegates) + 'static + Copy + Send + Sync,
    ) -> Self {
        self.delegates.push(Delegate { zones: zones.into(), run: Box::new(delegate) });
        self
    }
}

impl AbilityBuilder for SpellAbility<WithEffects> {
    fn build(self) -> AbilityDefinition {
        AbilityDefinition {
            ability_type: AbilityType::Spell,
            effect: Some(self.effects.0),
            delegates: self.delegates,
            costs: vec![],
        }
    }
}

pub struct NoCosts;
pub struct WithCosts(pub Vec<Cost>);

/// Builder for activated abilities.
///
/// > 113.3b. Activated abilities have a cost and an effect. They are written as
/// > `"[Cost]: [Effect.] [Activation instructions (if any).]"` A player may
/// > activate such an ability whenever they have priority. Doing so puts it on
/// > the stack, where it remains until it's countered, it resolves, or it
/// > otherwise leaves the stack.
///
/// <https://yawgatog.com/resources/magic-rules/#R1133b>
pub struct ActivatedAbility<TCosts, TEffects> {
    costs: TCosts,
    effects: TEffects,
    delegates: Vec<Delegate>,
}

impl ActivatedAbility<NoCosts, NoEffects> {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        Self { costs: NoCosts, effects: NoEffects, delegates: vec![] }
    }

    /// Cost to activate this ability
    pub fn cost(self, cost: Cost) -> ActivatedAbility<WithCosts, NoEffects> {
        ActivatedAbility {
            costs: WithCosts(vec![cost]),
            effects: NoEffects,
            delegates: self.delegates,
        }
    }
}

impl ActivatedAbility<WithCosts, NoEffects> {
    /// Effects when this ability resolves.
    pub fn effects(
        self,
        effects: impl Fn(&mut GameState, Scope) -> Outcome + 'static + Copy + Send + Sync,
    ) -> ActivatedAbility<WithCosts, WithEffects> {
        ActivatedAbility {
            costs: self.costs,
            effects: WithEffects(Box::new(effects)),
            delegates: self.delegates,
        }
    }
}

impl ActivatedAbility<WithCosts, WithEffects> {
    /// Adds a new delegate creation function to this ability. See
    /// [GameDelegates] for more information.
    pub fn delegate(
        mut self,
        zones: impl Into<EnumSet<Zone>>,
        delegate: impl Fn(&mut GameDelegates) + 'static + Send + Sync,
    ) -> Self {
        self.delegates.push(Delegate { zones: zones.into(), run: Box::new(delegate) });
        self
    }
}

impl AbilityBuilder for ActivatedAbility<WithCosts, WithEffects> {
    fn build(self) -> AbilityDefinition {
        AbilityDefinition {
            ability_type: AbilityType::Activated,
            effect: Some(self.effects.0),
            delegates: self.delegates,
            costs: self.costs.0,
        }
    }
}

/// Builder for triggered abilities.
///
/// > 113.3c. Triggered abilities have a trigger condition and an effect. They
/// > are written as `"[Trigger condition], [effect],"` and include (and usually
/// > begin with) the word "when," "whenever," or "at." Whenever the trigger
/// > event occurs, the ability is put on the stack the next time a player would
/// > receive priority and stays there until it's countered, it resolves, or it
/// > otherwise leaves the stack.
///
/// <https://yawgatog.com/resources/magic-rules/#R1133c>
pub struct TriggeredAbility<TEffects> {
    delegates: Vec<Delegate>,
    effects: TEffects,
}

impl TriggeredAbility<NoEffects> {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        Self { delegates: vec![], effects: NoEffects }
    }

    /// Effects when this ability resolves.
    pub fn effects(
        self,
        effects: impl Fn(&mut GameState, Scope) -> Outcome + 'static + Copy + Send + Sync,
    ) -> TriggeredAbility<WithEffects> {
        TriggeredAbility { delegates: self.delegates, effects: WithEffects(Box::new(effects)) }
    }
}

impl<TEffects> TriggeredAbility<TEffects> {
    /// Adds a new delegate creation function to this ability. See
    /// [GameDelegates] for more information.
    pub fn delegate(
        mut self,
        zones: impl Into<EnumSet<Zone>>,
        delegate: impl Fn(&mut GameDelegates) + 'static + Copy + Send + Sync,
    ) -> Self {
        self.delegates.push(Delegate { zones: zones.into(), run: Box::new(delegate) });
        self
    }
}

impl AbilityBuilder for TriggeredAbility<WithEffects> {
    fn build(self) -> AbilityDefinition {
        AbilityDefinition {
            ability_type: AbilityType::Triggered,
            effect: Some(self.effects.0),
            delegates: self.delegates,
            costs: vec![],
        }
    }
}

/// Builder for static abilities.
///
/// > 113.3d. Static abilities are written as statements. They're simply true.
/// > Static abilities create continuous effects which are active while the
/// > permanent with the ability is on the battlefield and has the ability, or
/// > while the object with the ability is in the appropriate zone.
///
/// <https://yawgatog.com/resources/magic-rules/#R1133d>
pub struct StaticAbility {
    delegates: Vec<Delegate>,
}

impl StaticAbility {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        Self { delegates: vec![] }
    }

    /// Adds a new delegate creation function to this ability. See
    /// [GameDelegates] for more information.
    pub fn delegate(
        mut self,
        zones: impl Into<EnumSet<Zone>>,
        delegate: impl Fn(&mut GameDelegates) + 'static + Copy + Send + Sync,
    ) -> Self {
        self.delegates.push(Delegate { zones: zones.into(), run: Box::new(delegate) });
        self
    }
}

impl AbilityBuilder for StaticAbility {
    fn build(self) -> AbilityDefinition {
        AbilityDefinition {
            ability_type: AbilityType::Static,
            effect: None,
            delegates: self.delegates,
            costs: vec![],
        }
    }
}

/// Represents the possible types of ability
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AbilityType {
    Spell,
    Activated,
    Triggered,
    Static,
}
