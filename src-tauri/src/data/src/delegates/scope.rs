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

use serde::{Deserialize, Serialize};

use crate::card_states::zones::{HasZones, ToCardId, Zones};
use crate::core::primitives::{
    AbilityId, AbilityNumber, CardId, EffectId, HasSource, PlayerName, Source, Timestamp,
};
use crate::game_states::game_state::TurnData;

/// Identifies the context in which an effect function or event delegate is
/// currently executing
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub struct Scope {
    /// The controller for this ability or the card that created this ability.
    ///
    /// In an effect function, this is the controller of the effect on the
    /// stack. In a delegate callback, this is always the current controller of
    /// the card.
    ///
    /// This is *usually* what you want, but note that you can get into trouble
    /// relying on this in an activated or triggered ability that needs to
    /// create its own delegate callbacks, since those callbacks will see their
    /// card's controller, *not* the controller of the ability that
    /// created them.
    pub controller: PlayerName,

    /// The identifier for the ability definition that is executing.
    pub ability_id: AbilityId,

    /// The current timestamp for the EffectId (if this is the scope for an
    /// effect) or the card owning this ability (if this is the scope for a
    /// delegate).
    pub timestamp: Timestamp,
}

/// Execution context for an effect function
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub struct EffectContext {
    /// Context in which this effect function is executing
    pub scope: Scope,

    /// Unique identifier for this instance of the effect function resolving.
    pub effect_id: EffectId,

    /// Current game turn
    pub turn: TurnData,
}

impl From<Scope> for Timestamp {
    fn from(scope: Scope) -> Timestamp {
        scope.timestamp
    }
}

impl From<EffectContext> for Timestamp {
    fn from(context: EffectContext) -> Timestamp {
        context.scope.timestamp
    }
}

impl EffectContext {
    pub fn controller(&self) -> PlayerName {
        self.scope.controller
    }

    pub fn ability_id(&self) -> AbilityId {
        self.scope.ability_id
    }
}

impl ToCardId for Scope {
    fn to_card_id(&self, zones: &impl HasZones) -> Option<CardId> {
        self.ability_id.to_card_id(zones)
    }
}

impl ToCardId for EffectContext {
    fn to_card_id(&self, zones: &impl HasZones) -> Option<CardId> {
        self.scope.to_card_id(zones)
    }
}

impl HasSource for Scope {
    fn source(&self) -> Source {
        Source::Ability { controller: self.controller, ability_id: self.ability_id }
    }
}

impl HasSource for EffectContext {
    fn source(&self) -> Source {
        self.scope.source()
    }
}
