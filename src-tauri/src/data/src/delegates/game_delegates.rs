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

use crate::core::primitives::{AbilityId, CardId, HasCardId, Zone};
use crate::delegates::card_delegate_list::CardDelegateList;
use crate::delegates::stores_delegates::StoresDelegates;
use crate::game_states::combat_state::AttackTarget;
use crate::game_states::game_state::GameState;

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
pub struct AttackerData {
    pub card_id: CardId,
    pub target: AttackTarget,
}

impl HasCardId for AttackerData {
    fn card_id(&self) -> CardId {
        self.card_id
    }
}

#[derive(Debug, Clone, Default)]
pub struct GameDelegates {
    /// Can the creature in [AttackerData] attack the indicated target?
    pub can_attack: CardDelegateList<GameState, AttackerData, bool>,
}

impl GameDelegates {
    pub fn apply_writes(&mut self, id: AbilityId, zones: EnumSet<Zone>) {
        self.can_attack.apply_writes(id, zones);
    }
}
