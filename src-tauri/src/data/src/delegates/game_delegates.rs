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

use crate::card_states::zones::HasZones;
use crate::core::numerics::{Power, Toughness};
use crate::core::primitives::{AbilityId, CardId, Color, PermanentId, Zone};
use crate::delegates::event_delegate_list::EventDelegateList;
use crate::delegates::game_delegate_data::{
    CanAttackTarget, CanBeBlocked, PermanentControllerChangedEvent, WillEnterBattlefieldEvent,
};
use crate::delegates::query_value::{ChangeText, EnumSets, Ints};
use crate::delegates::stores_delegates::StoresDelegates;
use crate::printed_cards::card_subtypes::{CreatureType, LandType};

#[derive(Default, Clone)]
pub struct GameDelegates {
    /// Invoked every time game state-triggered abilities are checked.
    pub state_triggered_ability: EventDelegateList<()>,

    /// A permanent is about to enter the battlefield.
    pub will_enter_battlefield: EventDelegateList<WillEnterBattlefieldEvent>,

    /// Invoked when the controller of a permanent changes.
    pub permanent_controller_changed: EventDelegateList<PermanentControllerChangedEvent>,
}

impl Debug for GameDelegates {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("GameDelegates").finish()
    }
}
