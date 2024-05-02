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

use std::collections::{HashSet, VecDeque};

use enumset::EnumSet;
use rand::prelude::SliceRandom;
use rand_xoshiro::Xoshiro256StarStar;
use serde::{Deserialize, Serialize};
use slotmap::SlotMap;

use crate::card_definitions::card_name::CardName;
use crate::card_states::card_kind::CardKind;
use crate::card_states::card_state::{CardFacing, CardState, TappedState};
use crate::card_states::counters::Counters;
use crate::card_states::custom_card_state::CustomCardStateList;
use crate::core::numerics::Damage;
use crate::core::primitives::{CardId, HasCardId, ObjectId, PlayerName, Zone};
#[allow(unused)] // Used in docs
use crate::game_states::game_state::GameState;

pub trait ZoneQueries {
    /// Looks up the state for a card.
    ///
    /// Panics if this Card ID does not exist.
    fn card(&self, id: impl HasCardId) -> &CardState;

    /// Mutable equivalent of [Self::card]
    fn card_mut(&mut self, id: impl HasCardId) -> &mut CardState;

    /// Returns the cards and card-like objects in a player's library, in order
    /// (last element in result is top card of deck).
    fn library(&self, player: PlayerName) -> &VecDeque<CardId>;

    /// Returns the set of cards and card-like objects in a player's hand
    fn hand(&self, player: PlayerName) -> &HashSet<CardId>;

    /// Equivalent function to [Self::hand] which returns an iterator over
    /// [CardState]s in an undefined order.
    fn hand_cards(&self, player: PlayerName) -> impl Iterator<Item = &CardState> {
        self.hand(player).iter().map(move |id| self.card(*id))
    }

    /// Returns the cards and card-like objects in a player's graveyard, in
    /// order (last element in result is top card).
    fn graveyard(&self, player: PlayerName) -> &VecDeque<CardId>;

    /// Equivalent function to [Self::graveyard] which returns an iterator over
    /// [CardState]s in order.
    fn graveyard_cards(&self, player: PlayerName) -> impl Iterator<Item = &CardState> {
        self.graveyard(player).iter().map(move |id| self.card(*id))
    }

    /// Returns the set of cards and card-like objects in a player's
    /// battlefield.
    fn battlefield(&self, player: PlayerName) -> &HashSet<CardId>;

    /// Equivalent function to [Self::battlefield] which returns an iterator
    /// over [CardState]s in an undefined order.
    fn battlefield_cards(&self, player: PlayerName) -> impl Iterator<Item = &CardState> {
        self.battlefield(player).iter().map(move |id| self.card(*id))
    }

    /// Returns the set of cards and card-like objects in a player's exile
    fn exile(&self, player: PlayerName) -> &HashSet<CardId>;

    /// Equivalent function to [Self::exile] which returns an iterator over
    /// [CardState]s in an undefined order.
    fn exile_cards(&self, player: PlayerName) -> impl Iterator<Item = &CardState> {
        self.exile(player).iter().map(move |id| self.card(*id))
    }

    /// Returns the cards and card-like objects currently on the stack (last
    /// element in result is top of stack).
    fn stack(&self) -> &[CardId];

    /// Equivalent function to [Self::stack] which returns an iterator over
    /// [CardState]s in order.
    fn stack_cards(&self) -> impl Iterator<Item = &CardState> {
        self.stack().iter().map(move |id| self.card(*id))
    }

    /// Returns the set of cards and card-like objects in a player's command
    /// zone
    fn command_zone(&self, player: PlayerName) -> &HashSet<CardId>;

    /// Returns the set of cards and card-like objects in a player's 'outside
    /// the game' zone
    fn outside_the_game_zone(&self, player: PlayerName) -> &HashSet<CardId>;
}

/// Stores the state & position of all cards and card-like objects
#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct Zones {
    /// All cards and card-like objects in the current game
    all_cards: SlotMap<CardId, CardState>,

    /// Next object id to use for zone moves.
    next_object_id: ObjectId,

    libraries: OrderedZone,
    hands: UnorderedZone,
    graveyards: OrderedZone,
    battlefield: UnorderedZone,
    exile: UnorderedZone,
    stack: Vec<CardId>,
    command_zone: UnorderedZone,
    outside_the_game_zone: UnorderedZone,
}

impl ZoneQueries for Zones {
    fn card(&self, id: impl HasCardId) -> &CardState {
        &self.all_cards[id.card_id()]
    }

    fn card_mut(&mut self, id: impl HasCardId) -> &mut CardState {
        &mut self.all_cards[id.card_id()]
    }

    fn library(&self, player: PlayerName) -> &VecDeque<CardId> {
        self.libraries.cards(player)
    }

    fn hand(&self, player: PlayerName) -> &HashSet<CardId> {
        self.hands.cards(player)
    }

    fn graveyard(&self, player: PlayerName) -> &VecDeque<CardId> {
        self.graveyards.cards(player)
    }

    fn battlefield(&self, player: PlayerName) -> &HashSet<CardId> {
        self.battlefield.cards(player)
    }

    fn exile(&self, player: PlayerName) -> &HashSet<CardId> {
        self.exile.cards(player)
    }

    fn stack(&self) -> &[CardId] {
        &self.stack
    }

    fn command_zone(&self, player: PlayerName) -> &HashSet<CardId> {
        self.command_zone.cards(player)
    }

    fn outside_the_game_zone(&self, player: PlayerName) -> &HashSet<CardId> {
        self.outside_the_game_zone.cards(player)
    }
}

impl Zones {
    /// Returns all currently known cards in an undefined order
    pub fn all_cards(&self) -> impl Iterator<Item = &CardState> {
        self.all_cards.values()
    }

    /// Creates a new named card, owned & controlled by the `owner` player in
    /// the provided `zone`.
    ///
    /// The card is created in a face-down state and is not visible to any
    /// player. The card is assigned a [CardId] and [ObjectId] on creation.
    pub fn create_hidden_card(
        &mut self,
        name: CardName,
        kind: CardKind,
        owner: PlayerName,
        zone: Zone,
    ) -> &CardState {
        let object_id = self.new_object_id();
        let id = self.all_cards.insert(CardState {
            card_id: CardId::default(),
            object_id,
            card_name: name,
            kind,
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

        self.add_to_zone(owner, id, zone);

        let card = &mut self.all_cards[id];
        card.card_id = id;
        card
    }

    /// Moves a card to a new zone and updates indices.
    pub fn move_card(&mut self, card_id: CardId, zone: Zone) {
        let old_zone = self.card_mut(card_id).zone;
        let owner = self.card_mut(card_id).owner;
        self.remove_from_zone(owner, card_id, old_zone);
        self.add_to_zone(owner, card_id, zone);
        self.card_mut(card_id).zone = zone;
        self.card_mut(card_id).object_id = self.new_object_id();
    }

    /// Shuffles the order of cards in a player's library
    pub fn shuffle_library(&mut self, player: PlayerName, rng: &mut Xoshiro256StarStar) {
        self.libraries.cards_mut(player).make_contiguous().shuffle(rng);
    }

    fn remove_from_zone(&mut self, owner: PlayerName, card_id: CardId, zone: Zone) {
        match zone {
            Zone::Hand => {
                self.hands.remove(card_id, owner);
            }
            Zone::Graveyard => {
                self.graveyards.remove(card_id, owner);
            }
            Zone::Library => {
                self.libraries.remove(card_id, owner);
            }
            Zone::Battlefield => {
                self.battlefield.remove(card_id, owner);
            }
            Zone::Stack => {
                if let Some(p) = self.stack.iter().rev().position(|&id| id == card_id) {
                    self.stack.remove(p);
                }
            }
            Zone::Exiled => {
                self.exile.remove(card_id, owner);
            }
            Zone::Command => {
                self.command_zone.remove(card_id, owner);
            }
            Zone::OutsideTheGame => {
                self.outside_the_game_zone.remove(card_id, owner);
            }
        }
    }

    fn add_to_zone(&mut self, owner: PlayerName, card_id: CardId, zone: Zone) {
        match zone {
            Zone::Library => self.libraries.cards_mut(owner).push_back(card_id),
            Zone::Hand => {
                self.hands.cards_mut(owner).insert(card_id);
            }
            Zone::Graveyard => self.graveyards.cards_mut(owner).push_back(card_id),
            Zone::Battlefield => {
                self.battlefield.cards_mut(owner).insert(card_id);
            }
            Zone::Exiled => {
                self.exile.cards_mut(owner).insert(card_id);
            }
            Zone::Stack => self.stack.push(card_id),
            Zone::Command => {
                self.command_zone.cards_mut(owner).insert(card_id);
            }
            Zone::OutsideTheGame => {
                self.outside_the_game_zone.cards_mut(owner).insert(card_id);
            }
        }
    }

    fn new_object_id(&mut self) -> ObjectId {
        let result = self.next_object_id;
        self.next_object_id = ObjectId(result.0 + 1);
        result
    }
}

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
struct UnorderedZone {
    player1: HashSet<CardId>,
    player2: HashSet<CardId>,
}

impl UnorderedZone {
    pub fn cards(&self, player_name: PlayerName) -> &HashSet<CardId> {
        match player_name {
            PlayerName::One => &self.player1,
            PlayerName::Two => &self.player2,
        }
    }

    pub fn cards_mut(&mut self, player_name: PlayerName) -> &mut HashSet<CardId> {
        match player_name {
            PlayerName::One => &mut self.player1,
            PlayerName::Two => &mut self.player2,
        }
    }

    /// Removes a card from this zone.
    ///
    /// Panics if this card is not present in this zone owned by `owner`.
    pub fn remove(&mut self, card_id: CardId, owner: PlayerName) {
        let removed = self.cards_mut(owner).remove(&card_id);
        assert!(removed, "Card {card_id:?} not found")
    }
}

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
struct OrderedZone {
    player1: VecDeque<CardId>,
    player2: VecDeque<CardId>,
}

impl OrderedZone {
    pub fn cards(&self, player_name: PlayerName) -> &VecDeque<CardId> {
        match player_name {
            PlayerName::One => &self.player1,
            PlayerName::Two => &self.player2,
        }
    }

    pub fn cards_mut(&mut self, player_name: PlayerName) -> &mut VecDeque<CardId> {
        match player_name {
            PlayerName::One => &mut self.player1,
            PlayerName::Two => &mut self.player2,
        }
    }

    /// Removes a card from this zone.
    ///
    /// The search is started from the top card in the zone. Panics if this
    /// card is not present in this zone owned by `owner`.
    pub fn remove(&mut self, card_id: CardId, owner: PlayerName) {
        if let Some(p) = self.cards_mut(owner).iter().rev().position(|&id| id == card_id) {
            self.cards_mut(owner).remove(p);
        } else {
            panic!("Card not found {card_id:?}")
        }
    }
}
