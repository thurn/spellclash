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
use crate::core::numerics::Damage;
use crate::core::primitives::{CardId, HasCardId, ObjectId, PlayerName, Zone};
use crate::printed_cards::printed_card::PrintedCard;

/// Represents the state of a card or token within an ongoing game.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CardState {
    /// Unique identifier for this card in the [Zones] struct.
    pub id: CardId,

    /// Object ID for this card. Cards receive an Object ID when they are
    /// created and then get a new one every time they change zones. In most
    /// typical game situations the rules only 'remember' effects that happen to
    /// a specific object.
    pub object_id: ObjectId,

    /// ID of the printed card for this card, used to populate the result of the
    /// [Self::printed] method after deserialization.
    ///
    /// Note that this is actually the ID of 'Face A' of the card in the MTG
    /// JSON data. Tokens *do* have an associated printed card as well.
    pub printed_card_id: Uuid,

    /// Current game zone for this card. Call [Self::set_zone_internal] instead
    /// of setting this value directly.
    pub zone: Zone,

    /// Whether this card is currently face down or has one of its faces up.
    ///
    /// A card that is not on the battlefield is face-down in the library or
    /// hand and is face-up in the graveyard, on the stack, or in the command
    /// zone. A card in exile can be either face down or face up.
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

    /// Timestamp at which this card arrived in its current zone. Call
    /// [Self::set_zone_internal] instead of setting this value directly.
    pub timestamp: u32,

    /// Stores custom state entries for this card.
    ///
    /// See [CustomCardStateList].
    pub custom_state: CustomCardStateList,

    /// Printed Card associated with this card. Use the [Self::printed] method
    /// instead of accessing this directly.
    ///
    /// This is populated immediately after deserialization with a static
    /// reference to the printed card, it should basically always be fine to
    /// .unwrap() this value.
    #[serde(skip)]
    pub printed_card_reference: Option<&'static PrintedCard>,
}

impl HasCardId for CardState {
    fn card_id(&self) -> CardId {
        self.id
    }
}

impl CardState {
    /// Returns the [PrintedCard] for this card
    pub fn printed(&self) -> &'static PrintedCard {
        self.printed_card_reference.unwrap()
    }

    /// Sets the position of this card.
    pub fn set_zone_internal(&mut self, sorting_key: u32, zone: Zone) {
        self.timestamp = sorting_key;
        self.zone = zone;
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
