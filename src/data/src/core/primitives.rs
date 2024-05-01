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

use enum_iterator::Sequence;
use enum_map::Enum;
use enumset::EnumSetType;
use serde::{Deserialize, Serialize};
use slotmap::new_key_type;
use uuid::Uuid;

/// The five canonical colors of magic.
#[derive(Debug, Hash, Serialize, Deserialize, EnumSetType, Sequence)]
pub enum Color {
    White,
    Blue,
    Black,
    Red,
    Green,
}

/// Possible colors of mana
#[derive(Debug, Hash, Serialize, Deserialize, EnumSetType, Enum, Sequence)]
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

/// Identifies one of the players in a game
#[derive(Debug, Hash, Serialize, Deserialize, EnumSetType, Ord, PartialOrd, Sequence)]
pub enum PlayerName {
    /// The player who plays first, who is "on the play"
    One,
    /// The player who plays second, who is "on the draw"
    Two,
}

/// Identifies a struct that is 1:1 associated with a given [PlayerName].
pub trait HasOwner {
    fn owner(&self) -> PlayerName;
}

impl HasOwner for PlayerName {
    fn owner(&self) -> PlayerName {
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
    /// - An ability on the stack
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

/// An identifier for an object within a game.
///
/// An object is an ability on the stack, a card, a copy of a card, a token, a
/// spell, a permanent, emblem, or player. Cards receive a new object ID when
/// they change zones.
///
/// Note that 'players' are not considered objects for the purposes of the CR
/// but are treated as such here.
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

pub const PLAYER_ONE_ID: ObjectId = ObjectId(1);
pub const PLAYER_TWO_ID: ObjectId = ObjectId(2);

pub trait HasObjectId {
    fn object_id(&self) -> ObjectId;
}

impl HasObjectId for ObjectId {
    fn object_id(&self) -> ObjectId {
        *self
    }
}

impl HasObjectId for PlayerName {
    fn object_id(&self) -> ObjectId {
        match self {
            PlayerName::One => PLAYER_ONE_ID,
            PlayerName::Two => PLAYER_TWO_ID,
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
#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub struct GameId(pub Uuid);

/// Unique identifier for a user
///
/// A 'user' is an operator of this software outside of the context of any game.
/// A 'player' is a participate within a game who may or may not be a user.
#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub struct UserId(pub Uuid);
