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

use crate::core::primitives::{AbilityNumber, CardId, HasCardId, HasSource, PlayerName, Source};

/// Identifies an ability of a card within a game
#[derive(Copy, Clone, Debug, Serialize, Deserialize)]
pub struct AbilityId {
    pub controller: PlayerName,
    pub number: AbilityNumber,
    pub card_id: CardId,
}

impl HasCardId for AbilityId {
    fn card_id(&self) -> CardId {
        self.card_id
    }
}

impl HasSource for AbilityId {
    fn source(&self) -> Source {
        Source::Ability {
            controller: self.controller,
            card_id: self.card_id,
            ability_number: self.number,
        }
    }
}
