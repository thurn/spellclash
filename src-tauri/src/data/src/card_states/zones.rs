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
use log::debug;
use rand::prelude::SliceRandom;
use rand_xoshiro::Xoshiro256StarStar;
use serde::{Deserialize, Serialize};
use slotmap::SlotMap;

use crate::card_definitions::card_name::CardName;
use crate::card_states::card_kind::CardKind;
use crate::card_states::card_reference::CardReference;
use crate::card_states::card_state::{CardFacing, CardState, TappedState};
use crate::card_states::counters::Counters;
use crate::card_states::custom_card_state::CustomCardStateList;
use crate::card_states::stack_ability_state::StackAbilityState;
use crate::core::numerics::Damage;
use crate::core::primitives::{
    AbilityId, CardId, EntityId, HasCardId, HasPlayerName, HasSource, ObjectId, PlayerName,
    StackAbilityId, StackItemId, Zone, ALL_POSSIBLE_PLAYERS,
};
#[allow(unused)] // Used in docs
use crate::game_states::game_state::GameState;
use crate::game_states::game_state::TurnData;
use crate::printed_cards::printed_card_id::PrintedCardId;

pub trait ZoneQueries {
    /// Looks up the state for a card.
    ///
    /// Panics if this Card ID does not exist.
    fn card(&self, id: impl HasCardId) -> &CardState;

    /// Mutable equivalent of [Self::card]
    fn card_mut(&mut self, id: impl HasCardId) -> &mut CardState;

    /// Returns the [CardState] for an [EntityId].
    ///
    /// If this [EntityId] is not the entity id of a card, or if the associated
    /// card no longer has a matching [EntityId] (i.e. because it has changed
    /// zones), None is returned instead.
    fn card_entity(&self, id: EntityId) -> Option<&CardState>;

    /// Mutable equivalent of [Self::card_entity].
    fn card_entity_mut(&mut self, id: EntityId) -> Option<&mut CardState>;

    /// Returns the [CardId] for this [EntityId].
    ///
    /// Panics if this [EntityId] is not valid, e.g. because it is no longer on
    /// the battlefield.
    fn card_entity_id(&self, id: EntityId) -> CardId {
        self.card_entity(id).expect("Card not found").id
    }

    /// Looks up the state for an ability on the stack.
    ///
    /// Panics if this stack ability does not exist.
    fn stack_ability(&self, id: StackAbilityId) -> &StackAbilityState;

    /// Mutable equivalent of [Self::stack_ability].
    fn stack_ability_mut(&mut self, id: StackAbilityId) -> &mut StackAbilityState;

    /// Returns the IDs of cards and card-like objects owned by a player in
    /// their library, in order (`.back()` element in result is top card).
    fn library(&self, player: impl HasPlayerName) -> &VecDeque<CardId>;

    /// Returns the IDs of cards and card-like objects owned by a player in
    /// their hand
    fn hand(&self, player: impl HasPlayerName) -> &HashSet<CardId>;

    /// Returns the IDs of cards and card-like objects owned by a player in
    /// their graveyard, in order (`.back()` element in result is top card).
    fn graveyard(&self, player: impl HasPlayerName) -> &VecDeque<CardId>;

    /// Returns the IDs of cards and card-like objects ***controlled*** by a
    /// player on the battlefield
    fn battlefield(&self, player: impl HasPlayerName) -> &HashSet<CardId>;

    /// Returns the IDs of cards and card-like objects owned by a player on the
    /// battlefield
    fn battlefield_owned(&self, player: impl HasPlayerName) -> &HashSet<CardId>;

    /// Returns the IDs of cards and card-like objects owned by a player in
    /// exile
    fn exile(&self, player: impl HasPlayerName) -> &HashSet<CardId>;

    /// Returns the IDs of all cards and activated or triggered abilities on the
    /// stack (last element in result is top of stack).
    fn stack(&self) -> &[StackItemId];

    /// Returns the IDs of cards and card-like objects owned by a player in the
    /// command zone
    fn command_zone(&self, player: impl HasPlayerName) -> &HashSet<CardId>;

    /// Returns the IDs of cards and card-like objects owned by a player outside
    /// the game
    fn outside_the_game_zone(&self, player: impl HasPlayerName) -> &HashSet<CardId>;
}

/// Stores the state & position of all cards and card-like objects
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Zones {
    /// All cards and card-like objects in the current game
    all_cards: SlotMap<CardId, CardState>,

    /// Triggered or activated abilities which have triggered or are currently
    /// on the stack.
    stack_abilities: SlotMap<StackAbilityId, StackAbilityState>,

    /// Next object id to use for zone moves.
    next_object_id: ObjectId,

    libraries: OrderedZone,
    hands: UnorderedZone,
    graveyards: OrderedZone,
    battlefield_controlled: UnorderedZone,
    battlefield_owned: UnorderedZone,
    exile: UnorderedZone,
    stack: Vec<StackItemId>,
    command_zone: UnorderedZone,
    outside_the_game_zone: UnorderedZone,
}

impl Default for Zones {
    fn default() -> Self {
        Self {
            all_cards: Default::default(),
            stack_abilities: Default::default(),
            next_object_id: ObjectId(100),
            libraries: Default::default(),
            hands: Default::default(),
            graveyards: Default::default(),
            battlefield_controlled: Default::default(),
            battlefield_owned: Default::default(),
            exile: Default::default(),
            stack: Default::default(),
            command_zone: Default::default(),
            outside_the_game_zone: Default::default(),
        }
    }
}

impl ZoneQueries for Zones {
    fn card(&self, id: impl HasCardId) -> &CardState {
        &self.all_cards[id.card_id()]
    }

    fn card_mut(&mut self, id: impl HasCardId) -> &mut CardState {
        &mut self.all_cards[id.card_id()]
    }

    fn card_entity(&self, id: EntityId) -> Option<&CardState> {
        match id {
            EntityId::Card(card_id, _) => {
                let card = self.card(card_id);
                if card.entity_id == id {
                    Some(card)
                } else {
                    None
                }
            }
            _ => None,
        }
    }

    fn card_entity_mut(&mut self, id: EntityId) -> Option<&mut CardState> {
        match id {
            EntityId::Card(card_id, _) => {
                let card = self.card_mut(card_id);
                if card.entity_id == id {
                    Some(card)
                } else {
                    None
                }
            }
            _ => None,
        }
    }

    fn stack_ability(&self, id: StackAbilityId) -> &StackAbilityState {
        &self.stack_abilities[id]
    }

    fn stack_ability_mut(&mut self, id: StackAbilityId) -> &mut StackAbilityState {
        &mut self.stack_abilities[id]
    }

    fn library(&self, player: impl HasPlayerName) -> &VecDeque<CardId> {
        self.libraries.cards(player.player_name())
    }

    fn hand(&self, player: impl HasPlayerName) -> &HashSet<CardId> {
        self.hands.cards(player.player_name())
    }

    fn graveyard(&self, player: impl HasPlayerName) -> &VecDeque<CardId> {
        self.graveyards.cards(player.player_name())
    }

    fn battlefield(&self, player: impl HasPlayerName) -> &HashSet<CardId> {
        self.battlefield_controlled.cards(player.player_name())
    }

    fn battlefield_owned(&self, player: impl HasPlayerName) -> &HashSet<CardId> {
        self.battlefield_owned.cards(player.player_name())
    }

    fn exile(&self, player: impl HasPlayerName) -> &HashSet<CardId> {
        self.exile.cards(player.player_name())
    }

    fn stack(&self) -> &[StackItemId] {
        &self.stack
    }

    fn command_zone(&self, player: impl HasPlayerName) -> &HashSet<CardId> {
        self.command_zone.cards(player.player_name())
    }

    fn outside_the_game_zone(&self, player: impl HasPlayerName) -> &HashSet<CardId> {
        self.outside_the_game_zone.cards(player.player_name())
    }
}

impl Zones {
    /// Returns all currently known cards in an undefined order
    pub fn all_cards(&self) -> impl Iterator<Item = &CardState> {
        self.all_cards.values()
    }

    /// Mutable version of [Self::all_cards]
    pub fn all_cards_mut(&mut self) -> impl Iterator<Item = &mut CardState> {
        self.all_cards.values_mut()
    }

    /// Returns all currently known stack abilities in an undefined order
    pub fn all_stack_abilities(&self) -> impl Iterator<Item = &StackAbilityState> {
        self.stack_abilities.values()
    }

    /// Mutable version of [Self::all_stack_abilities]
    pub fn all_stack_abilities_mut(&mut self) -> impl Iterator<Item = &mut StackAbilityState> {
        self.stack_abilities.values_mut()
    }

    /// Creates a new named card, owned & controlled by the `owner` player in
    /// the player's library.
    ///
    /// The card is created in a face-down state and is not visible to any
    /// player. The card is assigned a [CardId] and [EntityId] on creation.
    pub fn create_card_in_library(
        &mut self,
        reference: CardReference,
        kind: CardKind,
        owner: PlayerName,
        current_turn: TurnData,
    ) -> &CardState {
        let id = self.all_cards.insert(CardState {
            id: CardId::default(),
            entity_id: EntityId::Card(CardId::default(), ObjectId(0)),
            card_name: reference.printed_card_reference.name,
            printed_card_id: reference.identifier,
            kind,
            owner,
            controller: owner,
            zone: Zone::Library,
            facing: CardFacing::FaceDown,
            cast_as: EnumSet::empty(),
            tapped_state: TappedState::Untapped,
            revealed_to: EnumSet::empty(),
            counters: Counters::default(),
            damage: 0,
            targets: vec![],
            attached_to: None,
            custom_state: CustomCardStateList::default(),
            entered_current_zone: current_turn,
            last_changed_control: current_turn,
            printed_card_reference: Some(reference.printed_card_reference),
        });

        self.add_to_zone(owner, id, Zone::Library);
        let entity_id = self.new_card_entity_id(id);

        let card = &mut self.all_cards[id];
        card.id = id;
        card.entity_id = entity_id;
        card
    }

    /// Creates a new triggered ability.
    ///
    /// The ability is owned & controlled by the `owner` player and has the
    /// provided targets. The resulting ability is *not* placed on the stack
    /// immediately, this is handled the next time a player would receive
    /// priority.
    pub fn create_triggered_ability(
        &mut self,
        oracle_ability_id: AbilityId,
        owner: PlayerName,
        targets: Vec<EntityId>,
    ) -> &StackAbilityState {
        let id = self.stack_abilities.insert(StackAbilityState {
            id: StackAbilityId::default(),
            oracle_ability_id,
            placed_on_stack: false,
            owner,
            controller: owner,
            targets,
        });

        let ability = &mut self.stack_abilities[id];
        ability.id = id;
        ability
    }

    /// Moves a card to a new zone, updates indices, and assigns a new
    /// [EntityId] to it.
    ///
    /// The card is added as the top card of the target zone if it is ordered.
    ///
    /// Panics if this card was not found in its previous zone.
    pub fn move_card(&mut self, id: impl HasCardId, zone: Zone) {
        let card_id = id.card_id();
        let old_zone = self.card_mut(card_id).zone;
        let owner = self.card_mut(card_id).owner;
        self.remove_from_zone(owner, card_id, old_zone);
        self.add_to_zone(owner, card_id, zone);
        let entity_id = self.new_card_entity_id(card_id);
        let card = self.card_mut(card_id);
        card.zone = zone;
        card.entity_id = entity_id;
    }

    /// Adds a list of items to the top of the stack in the given order.
    pub fn add_abilities_to_stack(&mut self, mut ids: Vec<StackItemId>) {
        self.stack.append(&mut ids);
    }

    /// Changes the controller for a card.
    ///
    /// Panics if this card was not found in the `battlefield_controlled` set.
    pub fn change_controller(
        &mut self,
        _source: impl HasSource,
        id: impl HasCardId,
        controller: PlayerName,
        current_turn: TurnData,
    ) {
        let card_id = id.card_id();
        let card = self.card_mut(card_id);
        card.last_changed_control = current_turn;
        let old_controller = card.controller;
        card.controller = controller;
        if card.zone == Zone::Battlefield && old_controller != controller {
            self.battlefield_controlled.remove(card_id, old_controller);
            self.battlefield_controlled.cards_mut(controller).insert(card_id);
        }
    }

    /// Shuffles the order of cards in a player's library
    pub fn shuffle_library(&mut self, player: impl HasPlayerName, rng: &mut Xoshiro256StarStar) {
        self.libraries.cards_mut(player.player_name()).make_contiguous().shuffle(rng);
    }

    pub fn update_debug_info(&mut self) {
        self.hands.update_debug_info(&self.all_cards);
        self.battlefield_controlled.update_debug_info(&self.all_cards);
        self.battlefield_owned.update_debug_info(&self.all_cards);
        self.graveyards.update_debug_info(&self.all_cards);
        self.exile.update_debug_info(&self.all_cards);
        self.command_zone.update_debug_info(&self.all_cards);
        self.outside_the_game_zone.update_debug_info(&self.all_cards);
    }

    fn remove_from_zone(&mut self, owner: PlayerName, card_id: CardId, zone: Zone) {
        match zone {
            Zone::Hand => self.hands.remove(card_id, owner),
            Zone::Graveyard => self.graveyards.remove(card_id, owner),
            Zone::Library => self.libraries.remove(card_id, owner),
            Zone::Battlefield => {
                self.battlefield_owned.remove(card_id, owner);
                if !self.battlefield_controlled.cards_mut(owner).remove(&card_id) {
                    let mut removed = false;
                    for player in enum_iterator::all::<PlayerName>() {
                        removed |= self.battlefield_controlled.cards_mut(player).remove(&card_id);
                    }
                    if !removed {
                        panic!("Card not found {card_id:?} in controller set");
                    }
                }
            }
            Zone::Stack => {
                if let Some((i, _)) = self
                    .stack
                    .iter()
                    .enumerate()
                    .rev()
                    .find(|(_, id)| id.card_id() == Some(card_id))
                {
                    self.stack.remove(i);
                } else {
                    panic!("Card not found {card_id:?}");
                }
            }
            Zone::Exiled => self.exile.remove(card_id, owner),
            Zone::Command => self.command_zone.remove(card_id, owner),
            Zone::OutsideTheGame => self.outside_the_game_zone.remove(card_id, owner),
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
                self.battlefield_owned.cards_mut(owner).insert(card_id);
                self.battlefield_controlled.cards_mut(owner).insert(card_id);
            }
            Zone::Exiled => {
                self.exile.cards_mut(owner).insert(card_id);
            }
            Zone::Stack => self.stack.push(StackItemId::Card(card_id)),
            Zone::Command => {
                self.command_zone.cards_mut(owner).insert(card_id);
            }
            Zone::OutsideTheGame => {
                self.outside_the_game_zone.cards_mut(owner).insert(card_id);
            }
        }
    }

    fn new_card_entity_id(&mut self, card_id: CardId) -> EntityId {
        let result = self.next_object_id;
        self.next_object_id = ObjectId(result.0 + 1);
        EntityId::Card(card_id, result)
    }
}

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
struct UnorderedZone {
    player1: HashSet<CardId>,
    player2: HashSet<CardId>,
    p1_debug: HashSet<String>,
    p2_debug: HashSet<String>,
}

impl UnorderedZone {
    pub fn cards(&self, player_name: PlayerName) -> &HashSet<CardId> {
        match player_name {
            PlayerName::One => &self.player1,
            PlayerName::Two => &self.player2,
            _ => todo!("Not implemented"),
        }
    }

    pub fn cards_mut(&mut self, player_name: PlayerName) -> &mut HashSet<CardId> {
        match player_name {
            PlayerName::One => &mut self.player1,
            PlayerName::Two => &mut self.player2,
            _ => todo!("Not implemented"),
        }
    }

    /// Removes a card from this zone.
    ///
    /// Panics if this card is not present in this zone owned by `owner`.
    pub fn remove(&mut self, card_id: CardId, owner: PlayerName) {
        let removed = self.cards_mut(owner).remove(&card_id);
        if !removed {
            panic!("Card {card_id:?} not found");
        }
    }

    pub fn update_debug_info(&mut self, all_cards: &SlotMap<CardId, CardState>) {
        self.p1_debug = self
            .player1
            .iter()
            .map(|&id| {
                all_cards
                    .get(id)
                    .map(|c| c.displayed_name().to_string())
                    .unwrap_or_else(|| "Unknown".to_string())
            })
            .collect();
        self.p2_debug = self
            .player2
            .iter()
            .map(|&id| {
                all_cards
                    .get(id)
                    .map(|c| c.displayed_name().to_string())
                    .unwrap_or_else(|| "Unknown".to_string())
            })
            .collect();
    }
}

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
struct OrderedZone {
    player1: VecDeque<CardId>,
    player2: VecDeque<CardId>,
    p1_debug: Vec<String>,
    p2_debug: Vec<String>,
}

impl OrderedZone {
    pub fn cards(&self, player_name: PlayerName) -> &VecDeque<CardId> {
        match player_name {
            PlayerName::One => &self.player1,
            PlayerName::Two => &self.player2,
            _ => todo!("Not implemented"),
        }
    }

    pub fn cards_mut(&mut self, player_name: PlayerName) -> &mut VecDeque<CardId> {
        match player_name {
            PlayerName::One => &mut self.player1,
            PlayerName::Two => &mut self.player2,
            _ => todo!("Not implemented"),
        }
    }

    /// Removes a card from this zone.
    ///
    /// The search is started from the top card in the zone. Panics if this
    /// card is not present in this zone owned by `owner`.
    pub fn remove(&mut self, card_id: CardId, owner: PlayerName) {
        if let Some((i, _)) =
            self.cards_mut(owner).iter().enumerate().rev().find(|(_, &id)| id == card_id)
        {
            self.cards_mut(owner).remove(i).expect("Index {i:?} not found");
        } else {
            panic!("Card not found {card_id:?}");
        }
    }

    pub fn update_debug_info(&mut self, all_cards: &SlotMap<CardId, CardState>) {
        self.p1_debug = self
            .player1
            .iter()
            .map(|&id| {
                all_cards
                    .get(id)
                    .map(|c| c.displayed_name().to_string())
                    .unwrap_or_else(|| "Unknown".to_string())
            })
            .collect();
        self.p2_debug = self
            .player2
            .iter()
            .map(|&id| {
                all_cards
                    .get(id)
                    .map(|c| c.displayed_name().to_string())
                    .unwrap_or_else(|| "Unknown".to_string())
            })
            .collect();
    }
}
