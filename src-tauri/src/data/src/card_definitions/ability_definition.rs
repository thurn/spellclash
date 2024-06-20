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

use crate::card_definitions::ability_choices::{AbilityChoiceBuilder, AbilityChoices};
use crate::core::primitives::Zone;
use crate::costs::cost::Cost;
#[allow(unused)] // Used in docs
use crate::delegates::game_delegates::GameDelegates;
use crate::delegates::scope::Scope;
use crate::game_states::game_state::GameState;

/// An event callback function.
pub struct Delegate {
    /// [Zone]s in which this delegate should be active.
    pub zones: EnumSet<Zone>,

    /// Function to populate callbacks for this delegate
    pub run: Box<dyn Fn(&mut GameDelegates) + Send + Sync + 'static>,
}

/// Function to apply the effects of of an ability to the game.
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

    /// Choices available to the player when placing this ability on the stack.
    pub choices: AbilityChoices,

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
pub struct SpellAbility {
    effect: Option<EffectFn>,
    choices: AbilityChoices,
    delegates: Vec<Delegate>,
}

impl SpellAbility {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        Self { effect: None, delegates: vec![], choices: AbilityChoices::default() }
    }

    /// Effect when this spell resolves.
    pub fn effect(
        self,
        effect: impl Fn(&mut GameState, Scope) -> Outcome + 'static + Copy + Send + Sync,
    ) -> SpellAbility {
        SpellAbility {
            choices: self.choices,
            effect: Some(Box::new(effect)),
            delegates: self.delegates,
        }
    }
}

impl AbilityChoiceBuilder for SpellAbility {
    #[doc(hidden)]
    fn get_choices_mut(&mut self) -> &mut AbilityChoices {
        &mut self.choices
    }
}

impl SpellAbility {
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

impl AbilityBuilder for SpellAbility {
    fn build(self) -> AbilityDefinition {
        AbilityDefinition {
            ability_type: AbilityType::Spell,
            choices: self.choices,
            effect: self.effect,
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
    choices: AbilityChoices,
    effects: TEffects,
    delegates: Vec<Delegate>,
}

impl ActivatedAbility<NoCosts, NoEffects> {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        Self {
            costs: NoCosts,
            choices: AbilityChoices::default(),
            effects: NoEffects,
            delegates: vec![],
        }
    }

    /// Cost to activate this ability
    pub fn cost(self, cost: Cost) -> ActivatedAbility<WithCosts, NoEffects> {
        ActivatedAbility {
            costs: WithCosts(vec![cost]),
            choices: AbilityChoices::default(),
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
            choices: self.choices,
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
            choices: self.choices,
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
    choices: AbilityChoices,
    effects: TEffects,
}

impl TriggeredAbility<NoEffects> {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        Self { delegates: vec![], choices: AbilityChoices::default(), effects: NoEffects }
    }

    /// Effects when this ability resolves.
    pub fn effects(
        self,
        effects: impl Fn(&mut GameState, Scope) -> Outcome + 'static + Copy + Send + Sync,
    ) -> TriggeredAbility<WithEffects> {
        TriggeredAbility {
            delegates: self.delegates,
            choices: self.choices,
            effects: WithEffects(Box::new(effects)),
        }
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
            choices: self.choices,
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
            choices: AbilityChoices::default(),
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
