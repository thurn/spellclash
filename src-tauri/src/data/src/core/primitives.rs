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

use std::str::FromStr;

use derive_more::Display;
use enum_iterator::Sequence;
use enum_map::Enum;
use enumset::{EnumSet, EnumSetType};
use serde::{Deserialize, Serialize};
use slotmap::{new_key_type, Key, KeyData};
use specta::{DataType, Generics, Type, TypeMap};
use strum::EnumString;
use uuid::Uuid;

use crate::card_states::zones::{HasZones, ToCardId, ZoneQueries, Zones};
use crate::game_states::game_state::GameState;

/// Possible colors of cards or effects.
///
/// An empty color set represents colorless.
#[derive(Debug, Hash, Ord, PartialOrd, Serialize, Deserialize, EnumSetType, Enum, Sequence)]
pub enum Color {
    White,
    Blue,
    Black,
    Red,
    Green,
}

impl FromStr for Color {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "W" => Ok(Color::White),
            "U" => Ok(Color::Blue),
            "B" => Ok(Color::Black),
            "R" => Ok(Color::Red),
            "G" => Ok(Color::Green),
            _ => Err(()),
        }
    }
}

/// Possible colors of mana.
#[derive(Debug, Hash, Ord, PartialOrd, Serialize, Deserialize, EnumSetType, Enum, Sequence)]
pub enum ManaColor {
    Colorless,
    White,
    Blue,
    Black,
    Red,
    Green,
}

/// Supertypes for a card.
///
/// See <https://yawgatog.com/resources/magic-rules/#R2054>
#[derive(Debug, Serialize, Deserialize, EnumSetType, EnumString)]
pub enum CardSupertype {
    Basic,
    Legendary,
    Ongoing,
    Snow,
    World,
}

/// Types for a card.
///
/// See <https://yawgatog.com/resources/magic-rules/#R2052>
#[derive(Debug, Serialize, Deserialize, EnumSetType, EnumString)]
pub enum CardType {
    Artifact,
    Battle,
    Conspiracy,
    Creature,
    Dungeon,
    Enchantment,
    Instant,
    Land,
    Phenomenon,
    Plane,
    Planeswalker,
    Scheme,
    Sorcery,
    Tribal,
    Vanguard,
}

impl CardType {
    pub fn is_permanent(&self) -> bool {
        matches!(
            self,
            CardType::Artifact
                | CardType::Battle
                | CardType::Creature
                | CardType::Enchantment
                | CardType::Land
                | CardType::Planeswalker
        )
    }
}

/// Identifies one of the players in a game
#[derive(Debug, Hash, Serialize, Deserialize, EnumSetType, Ord, PartialOrd, Sequence)]
pub enum PlayerName {
    /// The player who plays first, who is "on the play"
    One,

    /// The player who plays second, who is "on the draw"
    Two,

    /// Player 3. Not currently implemented.
    Three,

    /// Player 4. Not currently implemented.
    Four,
}

impl PlayerName {
    pub fn entity_id(&self) -> EntityId {
        EntityId::Player(*self)
    }
}

pub const ALL_POSSIBLE_PLAYERS: EnumSet<PlayerName> = EnumSet::ALL;

/// Identifies a struct that is 1:1 associated with a given [PlayerName].
pub trait HasPlayerName {
    fn player_name(&self) -> PlayerName;
}

impl HasPlayerName for PlayerName {
    fn player_name(&self) -> PlayerName {
        *self
    }
}

/// Identifies a struct that has a controller.
pub trait HasController {
    fn controller(&self) -> PlayerName;
}

new_key_type! {
    /// Identifies a card or card-like object such as:
    ///
    /// - A normal card
    /// - A copy of a card on the stack
    /// - A token
    /// - An emblem
    pub struct CardId;
}

impl CardId {
    /// Converts an opaque number received from [Self::to_ffi_value] into a card
    /// id
    pub fn from_ffi_value(value: u64) -> Self {
        KeyData::from_ffi(value).into()
    }

    /// Returns an opaque number which can later be converted back into a card
    /// id
    pub fn to_ffi_value(&self) -> u64 {
        self.data().as_ffi()
    }
}

new_key_type! {
    /// Identifies a triggered or activated ability on the stack.
    pub struct StackAbilityId;
}

impl StackAbilityId {
    pub fn from_ffi_value(value: u64) -> Self {
        KeyData::from_ffi(value).into()
    }

    pub fn to_ffi_value(&self) -> u64 {
        self.data().as_ffi()
    }
}

/// An identifier for any game entity.
///
/// This is generally anything which can be a target: a player, card while it
/// exists in a specific zone, or ability on the stack.
#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash, Ord, PartialOrd, Serialize, Deserialize)]
pub enum EntityId {
    Player(PlayerName),
    Card(CardId, ObjectId),
    StackAbility(StackAbilityId),
}

pub const PLAYER_ONE_ID: EntityId = EntityId::Player(PlayerName::One);
pub const PLAYER_TWO_ID: EntityId = EntityId::Player(PlayerName::Two);
pub const PLAYER_THREE_ID: EntityId = EntityId::Player(PlayerName::Three);
pub const PLAYER_FOUR_ID: EntityId = EntityId::Player(PlayerName::Four);

impl ToCardId for EntityId {
    fn to_card_id(&self, zones: &impl HasZones) -> Option<CardId> {
        match self {
            EntityId::Card(card_id, object_id) => {
                if zones.zones().card(*card_id)?.object_id == *object_id {
                    Some(*card_id)
                } else {
                    None
                }
            }
            _ => None,
        }
    }
}

/// An identifier for a card or ability while it is in a given zone. A new
/// object ID is assigned each time a card changes zones, meaning that it can be
/// used for targeting effects that end when the card changes zones.
///
/// > 109.1. An object is an ability on the stack, a card, a copy of a card, a
/// > token, a spell, a permanent, or an emblem.
///
/// See <https://yawgatog.com/resources/magic-rules/#R1091>
#[derive(
    Debug, Clone, Copy, Default, Eq, PartialEq, Hash, Ord, PartialOrd, Serialize, Deserialize,
)]
pub struct ObjectId(pub u64);

impl ObjectId {
    /// Returns this ObjectId as a float, for use as a sorting key in the
    /// display layer.
    ///
    /// It's fine that we lose some precision here since it's only a visual
    /// effect.
    pub fn as_sorting_key(&self) -> f64 {
        self.0 as f64
    }
}

/// Unique identifier for a permanent on the battlefield
#[derive(
    Debug, Clone, Copy, Default, Eq, PartialEq, Hash, Ord, PartialOrd, Serialize, Deserialize,
)]
pub struct PermanentId {
    object_id: ObjectId,
    internal_card_id: CardId,
}

impl PermanentId {
    pub fn new(object_id: ObjectId, internal_card_id: CardId) -> Self {
        Self { object_id, internal_card_id }
    }
}

impl From<PermanentId> for EntityId {
    fn from(value: PermanentId) -> Self {
        EntityId::Card(value.internal_card_id, value.object_id)
    }
}

impl TryFrom<EntityId> for PermanentId {
    type Error = ();

    fn try_from(value: EntityId) -> Result<Self, Self::Error> {
        match value {
            EntityId::Card(card_id, object_id) => Ok(Self::new(object_id, card_id)),
            _ => Err(()),
        }
    }
}

impl ToCardId for PermanentId {
    fn to_card_id(&self, zones: &impl HasZones) -> Option<CardId> {
        if zones.zones().card(self.internal_card_id)?.object_id == self.object_id {
            Some(self.internal_card_id)
        } else {
            None
        }
    }
}

/// A unique identifier for an effect.
///
/// Each instance of an effect function resolving as a spell ability, activated
/// ability, or triggered ability gets its own ID.
#[derive(
    Debug, Clone, Copy, Default, Eq, PartialEq, Hash, Ord, PartialOrd, Serialize, Deserialize,
)]
pub struct EffectId(pub u64);

/// Identifies an ability within the set of abilities of a card.
///
/// Abilities are written in oracle text separated by a newline character, or
/// via spaces in the case of keywords. This number is the (0-indexed) position
/// of the ability within that text. One ability definition should be provided
/// for each clause that appears in card text, and this number is used to
/// produce the displayed text for that ability.
#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash, Ord, PartialOrd, Serialize, Deserialize)]
pub struct AbilityNumber(pub usize);

/// Identifies an ability of a card.
///
/// Each ability which appears in a card's oracle text gets its own Ability ID
/// and Ability Number, as described by [AbilityNumber]. Activated or triggered
/// abilities on the stack also have a [StackAbilityId] to identify their
/// current state.
#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash, Ord, PartialOrd, Serialize, Deserialize)]
pub struct AbilityId {
    pub card_id: CardId,
    pub number: AbilityNumber,
}

impl ToCardId for AbilityId {
    fn to_card_id(&self, _: &impl HasZones) -> Option<CardId> {
        Some(self.card_id)
    }
}

/// Identifies a card or an activated or triggered ability on the stack.
#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash, Ord, PartialOrd, Serialize, Deserialize)]
pub enum StackItemId {
    Card(CardId),
    StackAbility(StackAbilityId),
}

impl StackItemId {
    pub fn card_id(&self) -> Option<CardId> {
        match self {
            StackItemId::Card(card_id) => Some(*card_id),
            StackItemId::StackAbility(_) => None,
        }
    }

    pub fn stack_ability_id(&self) -> Option<StackAbilityId> {
        match self {
            StackItemId::Card(_) => None,
            StackItemId::StackAbility(stack_ability_id) => Some(*stack_ability_id),
        }
    }
}

/// A zone is a place where objects can be during the game.
///
/// See <https://yawgatog.com/resources/magic-rules/#R4001>
#[derive(Debug, Serialize, Deserialize, Hash, EnumSetType)]
pub enum Zone {
    Hand,
    Graveyard,
    Library,
    Battlefield,
    Stack,
    Exiled,
    Command,
    OutsideTheGame,
}

pub const ALL_ZONES: EnumSet<Zone> = EnumSet::ALL;

impl Zone {
    /// Is this zone a public zone?
    ///
    /// See <https://yawgatog.com/resources/magic-rules/#R4002>
    pub fn is_public(&self) -> bool {
        match self {
            Zone::Hand => false,
            Zone::Graveyard => true,
            Zone::Library => false,
            Zone::Battlefield => true,
            Zone::Stack => true,
            Zone::Exiled => true,
            Zone::Command => true,
            Zone::OutsideTheGame => false,
        }
    }
}

/// Unique identifier for a game
#[derive(Debug, Display, Clone, Copy, Eq, PartialEq, Hash, Serialize, Deserialize, Type)]
pub struct GameId(pub Uuid);

impl FromStr for GameId {
    type Err = uuid::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(GameId(Uuid::try_parse(s)?))
    }
}

/// Unique identifier for a user
///
/// A 'user' is an operator of this software outside of the context of any game.
/// A 'player' is a participate within a game who may or may not be a user.
#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash, Serialize, Deserialize, Type)]
pub struct UserId(pub Uuid);

/// Describes the source of some game mutation.
#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub enum Source {
    /// Mutation caused by the rules of the game, e.g. drawing a card for turn.
    Game,

    /// Mutation caused by an ability
    Ability { controller: PlayerName, ability_id: AbilityId },
}

impl Source {
    pub fn is_game_source(&self) -> bool {
        *self == Source::Game
    }

    pub fn is_ability_source(&self) -> bool {
        matches!(self, Source::Ability { .. })
    }
}

/// Marker trait for objects which have a source
pub trait HasSource {
    fn source(&self) -> Source;
}

impl HasSource for Source {
    fn source(&self) -> Source {
        *self
    }
}
