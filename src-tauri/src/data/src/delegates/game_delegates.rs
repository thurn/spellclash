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

use crate::card_states::zones::{HasZones, ToCardId, ZoneQueries, Zones};
use crate::core::numerics::{Power, Toughness};
use crate::core::primitives::{AbilityId, CardId, PermanentId, PlayerName, Zone};
use crate::delegates::card_delegate_list::CardDelegateList;
use crate::delegates::event_delegate_list::EventDelegateList;
use crate::delegates::flag::Flag;
use crate::delegates::stores_delegates::StoresDelegates;
use crate::game_states::combat_state::{AttackTarget, AttackerId};
use crate::game_states::game_state::GameState;

#[derive(Debug, Clone, Copy)]
pub struct CanAttackTarget {
    pub attacker_id: AttackerId,
    pub target: AttackTarget,
}

impl ToCardId for CanAttackTarget {
    fn to_card_id(&self, zones: &impl HasZones) -> Option<CardId> {
        self.attacker_id.to_card_id(zones)
    }
}

#[derive(Debug, Clone, Copy)]
pub struct PermanentControllerChangedEvent {
    pub permanent_id: PermanentId,
    pub old_controller: PlayerName,
    pub new_controller: PlayerName,
}

#[derive(Default, Clone)]
pub struct GameDelegates {
    /// Invoked every time game state-triggered abilities are checked.
    pub state_triggered_ability: EventDelegateList<GameState, ()>,

    /// Invoked when the controller of a permanent changes.
    pub permanent_controller_changed: EventDelegateList<GameState, PermanentControllerChangedEvent>,

    /// Can a creature attack the indicated target?
    pub can_attack_target: CardDelegateList<GameState, CanAttackTarget, Flag>,

    /// Does this permanent have haste?
    pub has_haste: CardDelegateList<GameState, PermanentId, Flag>,

    /// Queries the power value for a card.
    ///
    /// This may be invoked for a card in any zone.
    pub power: CardDelegateList<GameState, CardId, Power>,

    /// Queries the toughness value for a card.
    ///
    /// This may be invoked for a card in any zone.
    pub toughness: CardDelegateList<GameState, CardId, Toughness>,
}

impl GameDelegates {
    pub fn apply_writes(&mut self, id: AbilityId, zones: EnumSet<Zone>) {
        self.state_triggered_ability.apply_writes(id, zones);
        self.permanent_controller_changed.apply_writes(id, zones);
        self.can_attack_target.apply_writes(id, zones);
        self.has_haste.apply_writes(id, zones);
        self.power.apply_writes(id, zones);
        self.toughness.apply_writes(id, zones);
    }
}

impl Debug for GameDelegates {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("GameDelegates").finish()
    }
}
