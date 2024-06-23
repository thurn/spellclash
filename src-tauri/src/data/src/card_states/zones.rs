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
use std::fmt::Debug;
use std::hash::Hash;

use enumset::EnumSet;
use log::debug;
use rand::prelude::SliceRandom;
use rand_xoshiro::Xoshiro256StarStar;
use serde::{Deserialize, Serialize};
use slotmap::SlotMap;
use utils::outcome;
use utils::outcome::{Outcome, Success};

use crate::card_definitions::card_name::CardName;
use crate::card_states::card_kind::CardKind;
use crate::card_states::card_reference::CardReference;
use crate::card_states::card_state::{CardFacing, CardState, TappedState};
use crate::card_states::counters::Counters;
use crate::card_states::custom_card_state::CustomCardStateList;
use crate::card_states::stack_ability_state::StackAbilityState;
use crate::core::function_types::ScopedCardPredicate;
use crate::core::numerics::Damage;
use crate::core::primitives::{
    AbilityId, CardId, EntityId, HasController, HasPlayerName, HasSource, ObjectId, PermanentId,
    PlayerName, StackAbilityId, StackItemId, Zone, ALL_POSSIBLE_PLAYERS,
};
use crate::delegates::scope::Scope;
#[allow(unused)] // Used in docs
use crate::game_states::game_state::GameState;
use crate::game_states::game_state::TurnData;
use crate::printed_cards::printed_card_id::PrintedCardId;

pub trait ZoneQueries {
    /// Looks up the state for a card.
    ///
    /// Panics if this Card ID does not exist.
    fn card(&self, id: impl ToCardId) -> Option<&CardState>;

    /// Mutable equivalent of [Self::card]
    fn card_mut(&mut self, id: impl ToCardId) -> Option<&mut CardState>;

    /// Returns the [CardState] for a [PermanentId].
    ///
    /// If this [PermanentId] is not valid, e.g. because the permanent it
    /// represents is no longer on the battlefield, returns None.
    fn permanent(&self, id: PermanentId) -> Option<&CardState>;

    /// Mutable equivalent of [Self::permanent]
    fn permanent_mut(&mut self, id: PermanentId) -> Option<&mut CardState>;

    /// Returns the [CardId] for a [PermanentId].
    ///
    /// If this [PermanentId] is not valid, e.g. because the permanent it
    /// represents is no longer on the battlefield, returns None.
    fn card_id_for_permanent(&self, id: PermanentId) -> Option<CardId> {
        self.permanent(id).map(|c| c.id)
    }

    /// Returns the [PermanentId] for a [CardId].
    ///
    /// If this card is not a permanent, returns None.
    fn permanent_id_for_card(&self, card_id: CardId) -> Option<PermanentId> {
        self.card(card_id)?.permanent_id()
    }

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
    fn battlefield(&self, player: impl HasPlayerName) -> &HashSet<PermanentId>;

    /// Returns [CardId]s for cards controlled by a player on the battlefield
    fn battlefield_card_ids(
        &self,
        player: impl HasPlayerName,
    ) -> impl Iterator<Item = CardId> + '_ {
        self.battlefield(player).iter().filter_map(move |&id| self.card_id_for_permanent(id))
    }

    /// Returns the IDs of cards and card-like objects owned by a player on the
    /// battlefield
    fn battlefield_owned(&self, player: impl HasPlayerName) -> &HashSet<PermanentId>;

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

/// Identifies a struct that can be converted into a [CardId].
pub trait ToCardId: Copy + Debug {
    fn card_id(&self, zones: &impl HasZones) -> Option<CardId>;
}

impl ToCardId for CardId {
    fn card_id(&self, _: &impl HasZones) -> Option<CardId> {
        Some(*self)
    }
}

pub trait HasZones {
    fn zones(&self) -> &Zones;
}

pub trait IterMatchingScope<
    TId: ToCardId,
    TScope: Scope + 'static,
    TFn: ScopedCardPredicate<TId, TScope>,
>
{
    fn iter_matching_scoped<'a>(
        &'a self,
        game: &'a GameState,
        scope: TScope,
        predicate: TFn,
    ) -> impl Iterator<Item = TId> + 'a;
}

impl<TId: ToCardId, TScope: Scope + 'static, TFn: ScopedCardPredicate<TId, TScope>>
    IterMatchingScope<TId, TScope, TFn> for HashSet<TId>
{
    fn iter_matching_scoped<'a>(
        &'a self,
        game: &'a GameState,
        scope: TScope,
        predicate: TFn,
    ) -> impl Iterator<Item = TId> + 'a {
        self.iter().filter(move |&&id| predicate(game, scope, id) == Some(true)).copied()
    }
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
    hands: UnorderedZone<CardId>,
    graveyards: OrderedZone,
    battlefield_controlled: UnorderedZone<PermanentId>,
    battlefield_owned: UnorderedZone<PermanentId>,
    exile: UnorderedZone<CardId>,
    stack: Vec<StackItemId>,
    command_zone: UnorderedZone<CardId>,
    outside_the_game_zone: UnorderedZone<CardId>,
}

impl HasZones for Zones {
    fn zones(&self) -> &Zones {
        self
    }
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
    fn card(&self, id: impl ToCardId) -> Option<&CardState> {
        self.all_cards.get(id.card_id(self)?)
    }

    fn card_mut(&mut self, id: impl ToCardId) -> Option<&mut CardState> {
        self.all_cards.get_mut(id.card_id(self)?)
    }

    fn permanent(&self, id: PermanentId) -> Option<&CardState> {
        let card = self.all_cards.get(id.internal_card_id)?;
        if card.object_id() == id.object_id {
            Some(card)
        } else {
            None
        }
    }

    fn permanent_mut(&mut self, id: PermanentId) -> Option<&mut CardState> {
        let card = self.all_cards.get_mut(id.internal_card_id)?;
        if card.object_id() == id.object_id {
            Some(card)
        } else {
            None
        }
    }

    fn card_id_for_permanent(&self, id: PermanentId) -> Option<CardId> {
        self.permanent(id).map(|c| c.id)
    }

    fn card_entity(&self, id: EntityId) -> Option<&CardState> {
        match id {
            EntityId::Card(card_id, _) => {
                let card = self.card(card_id)?;
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
                let card = self.card_mut(card_id)?;
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

    fn battlefield(&self, player: impl HasPlayerName) -> &HashSet<PermanentId> {
        self.battlefield_controlled.cards(player.player_name())
    }

    fn battlefield_owned(&self, player: impl HasPlayerName) -> &HashSet<PermanentId> {
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
            control_changing_effects: vec![],
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
        ability_id: AbilityId,
        owner: PlayerName,
        targets: Vec<EntityId>,
    ) -> &StackAbilityState {
        let object_id = self.new_object_id();
        let id = self.stack_abilities.insert(StackAbilityState {
            id: StackAbilityId::default(),
            ability_id,
            object_id,
            placed_on_stack: false,
            owner,
            controller: owner,
            targets,
        });

        let ability = &mut self.stack_abilities[id];
        ability.id = id;
        ability
    }

    /// Remove the [StackAbilityState] with the given ID, if it exists.
    ///
    /// This updates the underlying data store *and* the list of items on the
    /// stack.
    pub fn remove_stack_ability(&mut self, stack_ability_id: StackAbilityId) {
        if let Some((i, _)) = self
            .stack
            .iter()
            .enumerate()
            .rev()
            .find(|(_, &id)| id == StackItemId::StackAbility(stack_ability_id))
        {
            self.stack.remove(i);
        }
        self.stack_abilities.remove(stack_ability_id);
    }

    /// Moves a card to a new zone, updates indices, and assigns a new
    /// [EntityId] to it.
    ///
    /// The card is added as the top card of the target zone if it is ordered.
    ///
    /// Returns None if this card was not found in the previous zone.
    pub fn move_card(&mut self, id: impl ToCardId, zone: Zone) -> Outcome {
        let card = self.card_mut(id)?;
        let card_id = card.id;
        let old_zone = card.zone;
        let owner = card.owner;
        self.remove_from_zone(owner, card_id, old_zone);
        let entity_id = self.new_card_entity_id(card_id);
        let card = self.card_mut(card_id).expect("Card not found");
        card.zone = zone;
        card.entity_id = entity_id;
        self.add_to_zone(owner, card_id, zone);
        outcome::OK
    }

    /// Adds a list of items to the top of the stack in the given order.
    pub fn add_abilities_to_stack(&mut self, mut ids: Vec<StackItemId>) {
        self.stack.append(&mut ids);
    }

    /// Returns an iterator over IDs of cards controlled by the [PlayerName]
    /// player in the indicated [Zone].
    pub fn cards_in_zone(
        &self,
        zone: Zone,
        player: PlayerName,
    ) -> Box<dyn Iterator<Item = CardId> + '_> {
        match zone {
            Zone::Hand => Box::new(self.hand(player).iter().copied()),
            Zone::Graveyard => Box::new(self.graveyard(player).iter().copied()),
            Zone::Library => Box::new(self.library(player).iter().copied()),
            Zone::Battlefield => Box::new(
                self.battlefield(player)
                    .iter()
                    .map(|&id| self.card_id_for_permanent(id).expect("Permanent not found")),
            ),
            Zone::Stack => Box::new(self.stack.iter().filter_map(move |id| {
                let id = id.card_id()?;
                if self.card(id)?.controller() == player {
                    Some(id)
                } else {
                    None
                }
            })),
            Zone::Exiled => Box::new(self.exile(player).iter().copied()),
            Zone::Command => Box::new(self.command_zone(player).iter().copied()),
            Zone::OutsideTheGame => Box::new(self.outside_the_game_zone(player).iter().copied()),
        }
    }

    /// Iterator over all abilities currently on the stack.
    pub fn abilities_on_stack(&self) -> impl Iterator<Item = StackAbilityId> + '_ {
        self.stack.iter().filter_map(move |id| id.stack_ability_id())
    }

    /// Marks a controller change for a card.
    ///
    /// Do not invoke this method directly, use the `change_controller` module
    /// instead.
    ///
    /// Returns None if this card does not exist.
    pub fn on_controller_changed(
        &mut self,
        id: impl ToCardId,
        old_controller: PlayerName,
        new_controller: PlayerName,
        current_turn: TurnData,
    ) -> Option<()> {
        let card = self.card(id)?;

        if let Some(permanent_id) = card.permanent_id() {
            if card.zone == Zone::Battlefield && old_controller != new_controller {
                self.battlefield_controlled.remove(permanent_id, old_controller);
                self.battlefield_controlled.cards_mut(new_controller).insert(permanent_id);
            }
        }

        Some(())
    }

    /// Shuffles the order of cards in a player's library
    pub fn shuffle_library(&mut self, player: impl HasPlayerName, rng: &mut Xoshiro256StarStar) {
        self.libraries.cards_mut(player.player_name()).make_contiguous().shuffle(rng);
    }

    fn remove_from_zone(&mut self, owner: PlayerName, card_id: CardId, zone: Zone) {
        match zone {
            Zone::Hand => self.hands.remove(card_id, owner),
            Zone::Graveyard => self.graveyards.remove(card_id, owner),
            Zone::Library => self.libraries.remove(card_id, owner),
            Zone::Battlefield => {
                let Some(permanent_id) = self.permanent_id_for_card(card_id) else {
                    panic!("Card has no permanent id {card_id:?}")
                };
                self.battlefield_owned.remove(permanent_id, owner);
                if !self.battlefield_controlled.cards_mut(owner).remove(&permanent_id) {
                    let mut removed = false;
                    for player in enum_iterator::all::<PlayerName>() {
                        removed |=
                            self.battlefield_controlled.cards_mut(player).remove(&permanent_id);
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
                let Some(permanent_id) = self.permanent_id_for_card(card_id) else {
                    panic!("Card has no permanent id {card_id:?}");
                };
                self.battlefield_owned.cards_mut(owner).insert(permanent_id);
                self.battlefield_controlled.cards_mut(owner).insert(permanent_id);
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

    fn new_object_id(&mut self) -> ObjectId {
        let result = self.next_object_id;
        self.next_object_id = ObjectId(result.0 + 1);
        result
    }

    fn new_card_entity_id(&mut self, card_id: CardId) -> EntityId {
        EntityId::Card(card_id, self.new_object_id())
    }
}

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
struct UnorderedZone<T: ToCardId + Hash + Eq + PartialEq + Debug> {
    player1: HashSet<T>,
    player2: HashSet<T>,
}

impl<T: ToCardId + Hash + Eq + PartialEq + Debug> UnorderedZone<T> {
    pub fn cards(&self, player_name: PlayerName) -> &HashSet<T> {
        match player_name {
            PlayerName::One => &self.player1,
            PlayerName::Two => &self.player2,
            _ => todo!("Not implemented"),
        }
    }

    pub fn cards_mut(&mut self, player_name: PlayerName) -> &mut HashSet<T> {
        match player_name {
            PlayerName::One => &mut self.player1,
            PlayerName::Two => &mut self.player2,
            _ => todo!("Not implemented"),
        }
    }

    /// Removes a card from this zone.
    ///
    /// Panics if this card is not present in this zone owned by `owner`.
    pub fn remove(&mut self, card_id: T, owner: PlayerName) {
        let removed = self.cards_mut(owner).remove(&card_id);
        if !removed {
            panic!("Card {card_id:?} not found");
        }
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
}
