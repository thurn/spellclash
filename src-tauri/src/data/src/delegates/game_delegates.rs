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
use crate::delegates::card_query_delegate_list::CardQueryDelegateList;
use crate::delegates::delegate_arguments::{
    CanAttackTarget, CanBeBlocked, PermanentControllerChangedEvent,
};
use crate::delegates::delegate_data::{EnumSets, Flag, Ints};
use crate::delegates::event_delegate_list::EventDelegateList;
use crate::delegates::stores_delegates::StoresDelegates;
use crate::printed_cards::card_subtypes::CreatureSubtype;

#[derive(Default, Clone)]
pub struct GameDelegates {
    /// Invoked every time game state-triggered abilities are checked.
    pub state_triggered_ability: EventDelegateList<()>,

    /// Invoked when the controller of a permanent changes.
    pub permanent_controller_changed: EventDelegateList<PermanentControllerChangedEvent>,

    /// Can a creature attack the indicated target?
    pub can_attack_target: CardQueryDelegateList<CanAttackTarget, Flag>,

    /// Can this creature be blocked by the indicated blocker?
    pub can_be_blocked: CardQueryDelegateList<CanBeBlocked, Flag>,

    /// Does this permanent have haste?
    pub has_haste: CardQueryDelegateList<PermanentId, Flag>,

    /// Does this permanent have flying?
    pub has_flying: CardQueryDelegateList<PermanentId, Flag>,

    /// Queries the power value for a card.
    ///
    /// This may be invoked for a card in any zone.
    pub power: CardQueryDelegateList<CardId, Ints<Power>>,

    /// Queries the base power value for a card. This is added to other
    /// modifiers to compute a final power value.
    pub base_power: CardQueryDelegateList<CardId, Ints<Power>>,

    /// Queries the toughness value for a card.
    ///
    /// This may be invoked for a card in any zone.
    pub toughness: CardQueryDelegateList<CardId, Ints<Toughness>>,

    /// Queries the base toughness value for a card. This is added to other
    /// modifiers to compute a final power value.
    pub base_toughness: CardQueryDelegateList<CardId, Ints<Toughness>>,

    /// Queries the colors of a card.
    ///
    /// An empty set represents colorless.
    pub colors: CardQueryDelegateList<CardId, EnumSets<Color>>,

    /// Queries the creature subtypes of a card.
    pub creature_subtypes: CardQueryDelegateList<CardId, EnumSets<CreatureSubtype>>,
}

impl Debug for GameDelegates {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("GameDelegates").finish()
    }
}
