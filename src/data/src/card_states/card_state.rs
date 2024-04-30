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
use uuid::Uuid;

use crate::card_states::counters::Counters;
use crate::card_states::custom_card_state::CustomCardStateList;
use crate::card_states::zone_object::ZoneObjectTrait;
#[allow(unused)] // Used in docs
use crate::card_states::zones::Zones;
use crate::core::numerics::{Damage, Timestamp};
use crate::core::primitives::{
    CardId, HasCardId, HasController, HasObjectId, HasOwner, HasTimestamp, ObjectId, PlayerName,
    Zone,
};
use crate::printed_cards::printed_card::PrintedCard;

/// Represents the state of a card, copy of a card on the stack, token, or
/// emblem within an ongoing game.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CardState {
    /// Unique identifier for this card in the [Zones] struct.
    pub card_id: CardId,

    /// Object ID for this card. Cards receive an Object ID when they are
    /// created and then get a new one every time they change zones.
    ///
    /// In most typical game situations the rules only 'remember' effects that
    /// happen to a specific object, e.g. if you exile a card and return it to
    /// the battlefield it gets a new object ID and effects targeting it will
    /// end.
    pub object_id: ObjectId,

    /// ID of the printed card for this card, used to populate the result of the
    /// [Self::printed] method after deserialization.
    ///
    /// Note that this is actually the ID of 'Face A' of the card in the MTG
    /// JSON data. Tokens *do* have an associated printed card as well.
    pub printed_card_id: Uuid,

    /// The player who this card belongs to, who starts the game with this card
    /// or who creates this token.
    ///
    /// See <https://yawgatog.com/resources/magic-rules/#R1083>
    pub owner: PlayerName,

    /// The player who can currently make decisions about this card.
    ///
    /// For cards which are not currently on the battlefield or on the stack,
    /// this will be the card's owner.
    ///
    /// See <https://yawgatog.com/resources/magic-rules/#R1084>
    pub controller: PlayerName,

    /// Current game zone location for this card.
    ///
    /// Please update [Self::timestamp] and [Self::object_id] whenever this
    /// changes.
    pub zone: Zone,

    /// Whether this card is currently face down or has one of its faces up.
    ///
    /// A card that is not on the battlefield is:
    /// - Face-down in the library or hand
    /// - Face-up in the graveyard, on the stack, or in the command zone.
    /// - Either face up or face down in exile depending on the effect that put
    ///   it there
    pub facing: CardFacing,

    /// Whether this card is current tapped.
    ///
    /// A card that is not on the battlefield is always untapped.
    pub tapped_state: TappedState,

    /// Players who this card has been revealed to.
    pub revealed_to: EnumSet<PlayerName>,

    /// Counters on this card.
    ///
    /// A card that is not on the battlefield *typically* does not have counters
    /// on it, but see e.g. 'Skullbriar, the Walking Grave' or 'Lightning
    /// Storm'.
    pub counters: Counters,

    /// Damage marked on this card.
    ///
    /// A card that is not on the battlefield always has 0 damage.
    pub damage: Damage,

    /// Timestamp at which this card arrived in its current zone.
    pub timestamp: Timestamp,

    /// Targets for this card, selected when it is placed on the stack.
    ///
    /// Cards which are not on the stack cannot have targets.
    pub targets: Vec<ObjectId>,

    /// The object this card is attached to.
    ///
    /// Cards such as Equipment, Auras, and Fortifications can be attached to a
    /// permanent or player. Cards that are not on the battlefield cannot be
    /// attached to each other.
    pub attached_to: Option<ObjectId>,

    /// Stores custom state entries for this card.
    ///
    /// See [CustomCardStateList].
    pub custom_state: CustomCardStateList,

    /// Printed Card associated with this card. Use the [Self::printed] method
    /// instead of accessing this directly.
    ///
    /// All cards must have a [PrintedCard] and this is populated immediately
    /// after deserialization with a static reference. It should basically
    /// always be fine to .unwrap() this value by calling the
    /// [Self::printed] method.
    #[serde(skip)]
    pub printed_card_reference: Option<&'static PrintedCard>,
}

impl HasCardId for CardState {
    fn card_id(&self) -> CardId {
        self.card_id
    }
}

impl HasObjectId for CardState {
    fn object_id(&self) -> ObjectId {
        self.object_id
    }
}

impl HasOwner for CardState {
    fn owner(&self) -> PlayerName {
        self.owner
    }
}

impl HasController for CardState {
    fn controller(&self) -> PlayerName {
        self.controller
    }
}

impl HasTimestamp for CardState {
    fn timestamp(&self) -> Timestamp {
        self.timestamp
    }
}

impl ZoneObjectTrait for CardState {
    fn targets(&self) -> &[ObjectId] {
        &self.targets
    }
}

impl CardState {
    /// Returns the [PrintedCard] for this card
    pub fn printed(&self) -> &'static PrintedCard {
        self.printed_card_reference.unwrap()
    }
}

/// Whether a card is tapped or untapped.
///
/// I assume within 10 years WoTC will introduce a third tapped state somehow,
/// so might as well make this an enum.
#[derive(Copy, Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub enum TappedState {
    Untapped,
    Tapped,
}

/// Facing for this card, corresponding to the [PrintedCard] faces.
#[derive(Copy, Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub enum CardFacing {
    FaceDown,
    FaceUp,
    FaceBUp,
}
