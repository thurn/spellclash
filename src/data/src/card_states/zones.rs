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
use serde::{Deserialize, Serialize};
use slotmap::SlotMap;

use crate::card_definitions::card_name::CardName;
use crate::card_states::card_state::{CardFacing, CardState, TappedState};
use crate::card_states::counters::Counters;
use crate::card_states::custom_card_state::CustomCardStateList;
use crate::card_states::stack_ability::{StackAbility, StackAbilityId};
use crate::core::numerics::Damage;
use crate::core::primitives::{CardId, ObjectId, PlayerName, Zone};

/// Stores the state of all cards and all abilities currently on the stack.
#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct Zones {
    /// All cards, copies of cards on the stack, tokens, and emblems in the
    /// current game.
    all_cards: SlotMap<CardId, CardState>,

    /// Abilities currently on the stack
    stack_abilities: SlotMap<StackAbilityId, StackAbility>,

    /// Next object id to use for zone moves.
    next_object_id: ObjectId,
}

impl Zones {
    /// Creates a new named card, owned & controlled by the `owner` player in
    /// the provided `zone`.
    ///
    /// The card is created in a face-down state and is not visible to any
    /// player. The card is assigned a [CardId] and [ObjectId] on creation.
    pub fn create_hidden_card(
        &mut self,
        name: CardName,
        owner: PlayerName,
        zone: Zone,
    ) -> &CardState {
        let object_id = self.create_object_id();
        let id = self.all_cards.insert(CardState {
            card_id: CardId::default(),
            object_id: ObjectId::default(),
            card_name: name,
            owner,
            controller: owner,
            zone,
            facing: CardFacing::FaceDown,
            tapped_state: TappedState::Untapped,
            revealed_to: EnumSet::empty(),
            counters: Counters::default(),
            damage: Damage(0),
            targets: vec![],
            attached_to: None,
            custom_state: CustomCardStateList::default(),
            printed_card_reference: None,
        });
        let card = &mut self.all_cards[id];
        card.card_id = id;
        card.object_id = object_id;
        card
    }

    fn create_object_id(&mut self) -> ObjectId {
        let result = self.next_object_id;
        self.next_object_id = ObjectId(result.0 + 1);
        result
    }
}
