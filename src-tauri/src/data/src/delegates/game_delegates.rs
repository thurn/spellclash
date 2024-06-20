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

use std::fmt::{Debug, Formatter};

use enumset::EnumSet;

use crate::card_states::zones::ZoneQueries;
use crate::core::primitives::{AbilityId, CardId, HasCardId, PlayerName, Zone};
use crate::delegates::card_delegate_list::CardDelegateList;
use crate::delegates::event_delegate_list::EventDelegateList;
use crate::delegates::flag::Flag;
use crate::delegates::stores_delegates::StoresDelegates;
use crate::game_states::combat_state::AttackTarget;
use crate::game_states::game_state::GameState;

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
pub struct CanAttackTarget {
    pub card_id: CardId,
    pub target: AttackTarget,
}

impl HasCardId for CanAttackTarget {
    fn card_id(&self) -> CardId {
        self.card_id
    }
}

#[derive(Default, Clone)]
pub struct GameDelegates {
    /// Invoked every time game state changes.
    ///
    /// Warning: This is extremely expensive!
    pub state_triggered_abilities: EventDelegateList<GameState, ()>,

    /// Can a creature attack the indicated target?
    pub can_attack_target: CardDelegateList<GameState, CanAttackTarget, Flag>,
}

impl GameDelegates {
    pub fn apply_writes(&mut self, id: AbilityId, zones: EnumSet<Zone>) {
        self.state_triggered_abilities.apply_writes(id, zones);
        self.can_attack_target.apply_writes(id, zones);
    }
}

impl Debug for GameDelegates {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("GameDelegates").finish()
    }
}
