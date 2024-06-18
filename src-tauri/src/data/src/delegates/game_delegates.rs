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
use crate::delegates::stores_delegates::StoresDelegates;
use crate::game_states::combat_state::AttackTarget;
use crate::game_states::game_state::GameState;

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
pub struct AttackerData {
    pub card_id: CardId,
    pub target: AttackTarget,
}

impl AttackerData {
    /// Returns the [PlayerName] of the player being attacked.
    ///
    /// Panics if the attack target is no longer valid (e.g. not on the
    /// battlefield).
    pub fn defending_player(&self, game: &GameState) -> PlayerName {
        match self.target {
            AttackTarget::Player(p) => p,
            AttackTarget::Planeswalker(entity_id) | AttackTarget::Battle(entity_id) => {
                game.card_entity(entity_id).expect("Entity not found").controller
            }
        }
    }
}

impl HasCardId for AttackerData {
    fn card_id(&self) -> CardId {
        self.card_id
    }
}

#[derive(Default, Clone)]
pub struct GameDelegates {
    /// Can the creature in [AttackerData] attack the indicated target?
    pub can_attack: CardDelegateList<GameState, AttackerData, bool>,
}

impl GameDelegates {
    pub fn apply_writes(&mut self, id: AbilityId, zones: EnumSet<Zone>) {
        self.can_attack.apply_writes(id, zones);
    }
}

impl Debug for GameDelegates {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("GameDelegates").finish()
    }
}
