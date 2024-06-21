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

use crate::core::primitives::{
    AbilityId, AbilityNumber, CardId, EffectId, HasCardId, HasSource, PlayerName, Source,
};

/// Identifies the context in which an event function or event delegate is
/// currently executing
pub trait Scope: HasCardId {
    /// The controller for this ability or the card that created this ability.
    ///
    /// In an effect function, this is the controller of the effect on the
    /// stack. In a delegate callback, this is the controller of the card.
    ///
    /// This is *usually* what you want, but note that you can get into trouble
    /// relying on this in an activated or triggered ability that needs to
    /// create its own delegate callbacks, since those callbacks will see their
    /// card's controller, *not* the controller of the ability that
    /// created them.
    fn controller(&self) -> PlayerName;

    /// The identifier for the ability definition that is executing.
    fn ability_id(&self) -> AbilityId;
}

/// Execution context for a delegate callback
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub struct DelegateScope {
    pub controller: PlayerName,
    pub ability_id: AbilityId,
}

/// Execution context for an effect function
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub struct EffectScope {
    pub controller: PlayerName,
    pub ability_id: AbilityId,

    /// Unique identifier for this instance of the effect function resolving.
    pub effect_id: EffectId,
}

impl Scope for DelegateScope {
    fn controller(&self) -> PlayerName {
        self.controller
    }

    fn ability_id(&self) -> AbilityId {
        self.ability_id
    }
}

impl HasCardId for DelegateScope {
    fn card_id(&self) -> CardId {
        self.ability_id.card_id
    }
}

impl Scope for EffectScope {
    fn controller(&self) -> PlayerName {
        self.controller
    }

    fn ability_id(&self) -> AbilityId {
        self.ability_id
    }
}

impl HasCardId for EffectScope {
    fn card_id(&self) -> CardId {
        self.ability_id.card_id
    }
}

impl<T: Scope> HasSource for T {
    fn source(&self) -> Source {
        Source::Ability { controller: self.controller(), ability_id: self.ability_id() }
    }
}
