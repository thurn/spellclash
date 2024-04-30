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

use crate::core::primitives::{AbilityNumber, Zone};
use crate::delegates::game_delegates::{Delegate, DelegateFn};
use crate::effects::effect::EffectList;
use crate::game_states::game_state::GameState;

pub type EffectFn = fn(&GameState, &mut EffectList);

pub struct NoEffects;

pub struct WithEffects(pub EffectFn);

/// Builder for Spell Abilities.
///
/// Spell abilities are abilities that are followed as instructions while an
/// instant or sorcery spell is resolving. Any text on an instant or sorcery
/// spell is a spell ability unless it's an activated ability, a triggered
/// ability, or a static ability that fits the criteria described in rule
/// 113.6.
///
/// <https://yawgatog.com/resources/magic-rules/#R1133a>
pub struct SpellAbility<TEffects> {
    pub effects: TEffects,

    pub delegates: Vec<Delegate>,
}

impl Default for SpellAbility<NoEffects> {
    fn default() -> Self {
        Self { effects: NoEffects, delegates: vec![] }
    }
}

impl SpellAbility<NoEffects> {
    /// Effects when this spell resolves.
    pub fn effects(self, effects: EffectFn) -> SpellAbility<WithEffects> {
        SpellAbility { effects: WithEffects(effects), delegates: self.delegates }
    }
}

impl SpellAbility<WithEffects> {
    pub fn delegate(mut self, zones: impl Into<EnumSet<Zone>>, delegate: DelegateFn) -> Self {
        self.delegates.push(Delegate { zones: zones.into(), run: delegate });
        self
    }
}

pub struct ActivatedAbility<TEffects> {
    /// Number of this ability within the card. Identifies the ability and is
    /// used to produce rules text.
    pub number: AbilityNumber,

    pub effects: TEffects,

    pub delegates: Vec<Delegate>,
}

#[derive(Debug, Clone)]
pub enum AbilityType {
    Spell,

    /// Activated abilities have a cost and an effect. They are written as
    /// "`[Cost]: [Effect.] [Activation instructions (if any).]`" A player may
    /// activate such an ability whenever they have priority. Doing so puts it
    /// on the stack, where it remains until it's countered, it resolves, or it
    /// otherwise leaves the stack.
    ///
    /// <https://yawgatog.com/resources/magic-rules/#R1133b>
    Activated,

    /// Triggered abilities have a trigger condition and an effect. They are
    /// written as "`[Trigger condition], [effect],`" and include (and usually
    /// begin with) the word "when," "whenever," or "at." Whenever the trigger
    /// event occurs, the ability is put on the stack the next time a player
    /// would receive priority and stays there until it's countered, it
    /// resolves, or it otherwise leaves the stack.
    ///
    /// <https://yawgatog.com/resources/magic-rules/#R1133c>
    Triggered,

    /// Static abilities are written as statements. They're simply true. Static
    /// abilities create continuous effects which are active while the permanent
    /// with the ability is on the battlefield and has the ability, or while the
    /// object with the ability is in the appropriate zone.
    ///
    /// https://yawgatog.com/resources/magic-rules/#R1133d
    Static,
}
