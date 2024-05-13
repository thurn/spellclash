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
use slotmap::new_key_type;
use uuid::Uuid;

/// The five canonical colors of magic.
#[derive(Debug, Hash, Ord, PartialOrd, Serialize, Deserialize, EnumSetType, Sequence)]
pub enum Color {
    White,
    Blue,
    Black,
    Red,
    Green,
}

/// Possible colors of mana
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
#[derive(Debug, Serialize, Deserialize, EnumSetType)]
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
#[derive(Debug, Serialize, Deserialize, EnumSetType)]
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

/// Identifies a struct that is 1:1 associated with a given [CardId].
pub trait HasCardId {
    fn card_id(&self) -> CardId;
}

impl HasCardId for CardId {
    fn card_id(&self) -> CardId {
        // I know this is the same as Into, I just find it less annoying to have
        // explicit types :)
        *self
    }
}

impl<T: HasCardId> HasCardId for &T {
    fn card_id(&self) -> CardId {
        (*self).card_id()
    }
}

new_key_type! {
    /// Identifies a triggered or activated ability on the stack.
    pub struct StackAbilityId;
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
    pub fn as_u64(&self) -> u64 {
        self.0
    }
}

pub const PLAYER_ONE_ID: EntityId = EntityId::Player(PlayerName::One);
pub const PLAYER_TWO_ID: EntityId = EntityId::Player(PlayerName::Two);
pub const PLAYER_THREE_ID: EntityId = EntityId::Player(PlayerName::Three);
pub const PLAYER_FOUR_ID: EntityId = EntityId::Player(PlayerName::Four);

pub trait HasEntityId {
    fn entity_id(&self) -> EntityId;
}

impl HasEntityId for EntityId {
    fn entity_id(&self) -> EntityId {
        *self
    }
}

impl HasEntityId for PlayerName {
    fn entity_id(&self) -> EntityId {
        match self {
            PlayerName::One => PLAYER_ONE_ID,
            PlayerName::Two => PLAYER_TWO_ID,
            PlayerName::Three => PLAYER_THREE_ID,
            PlayerName::Four => PLAYER_FOUR_ID,
        }
    }
}

/// Identifies an ability of a card.
///
/// Abilities are always written in oracle text_strings separated by a newline
/// character. This number is the (0-indexed) position of the ability within
/// that text_strings. One ability definition should be provided for each clause
/// that appears in card text_strings, and this number is used to produce the
/// displayed text_strings for that ability.
#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash, Ord, PartialOrd, Serialize, Deserialize)]
pub struct AbilityNumber(pub usize);

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
#[derive(Debug, Display, Clone, Copy, Eq, PartialEq, Hash, Serialize, Deserialize)]
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
#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub struct UserId(pub Uuid);

/// Describes the source of some game mutation.
#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub enum Source {
    /// Mutation caused by the rules of the game, e.g. drawing a card for turn.
    Game,

    /// Mutation caused by an ability
    Ability { controller: PlayerName, card_id: CardId, ability_number: AbilityNumber },
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
