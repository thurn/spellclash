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
    AbilityId, AbilityNumber, CardId, HasCardId, HasSource, PlayerName, Source,
};

/// Identifies the context in which an event delegate is currently executing
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub struct Scope {
    pub controller: PlayerName,
    pub ability_id: AbilityId,
}

impl HasCardId for Scope {
    fn card_id(&self) -> CardId {
        self.ability_id.card_id
    }
}

impl HasSource for Scope {
    fn source(&self) -> Source {
        Source::Ability { controller: self.controller, ability_id: self.ability_id }
    }
}
