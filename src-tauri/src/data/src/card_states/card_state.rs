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

use std::sync::Arc;

use enumset::EnumSet;
use serde::{Deserialize, Serialize};
use specta::Type;

use crate::card_definitions::card_name::CardName;
use crate::card_states::card_kind::CardKind;
use crate::card_states::counters::Counters;
use crate::card_states::custom_card_state::CustomCardStateList;
#[allow(unused)] // Used in docs
use crate::card_states::zones::Zones;
use crate::card_states::zones::{HasZones, ToCardId};
use crate::core::numerics::Damage;
use crate::core::primitives::{
    AbilityId, CardId, EffectId, EntityId, HasController, HasPlayerName, ObjectId, PermanentId,
    PlayerName, Timestamp, Zone,
};
#[allow(unused)] // Used in docs
use crate::game_states::game_state::{GameState, TurnData};
use crate::printed_cards::printed_card::{Face, PrintedCard, PrintedCardFace};
use crate::printed_cards::printed_card_id::PrintedCardId;
use crate::queries::card_queries::CardQueries;

/// Represents the state of a card or card-like object.
///
/// The term "card" is used broadly here to include:
///
/// - A normal card
/// - A copy of a card on the stack
/// - A token
/// - An emblem
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CardState {
    /// Unique identifier for this card in the [Zones] struct.
    ///
    /// Do not modify this field.
    pub id: CardId,

    /// Object ID for this card. Cards receive an Object ID when they are
    /// created and then get a new one every time they change zones.
    ///
    /// In most typical game situations the rules only 'remember' effects that
    /// happen to a specific object, e.g. if you exile a card and return it to
    /// the battlefield it gets a new object ID and effects targeting it will
    /// end.
    ///
    /// Do not modify this field directly, use the `move_card` module instead.
    pub object_id: ObjectId,

    /// Identifier for this card within the rules of the game. Do not modify
    /// this field.
    ///
    /// See [CardName] for more information.
    pub card_name: CardName,

    /// Identifier for the printed card for this card.
    ///
    /// Do not modify this field.
    pub printed_card_id: PrintedCardId,

    /// The current timestamp for this card's effects.
    ///
    /// Effects produced by a card within a layer are ordered based on the
    /// timestamps of those effects.
    ///
    /// Do not modify this field directly, use the relevant higher-level modules
    /// like `move_card` instead.
    pub timestamp: Timestamp,

    /// Describes which kind of card-like object this is. Do not modify this
    /// field.
    ///
    /// See [CardKind].
    pub kind: CardKind,

    /// The player who this card belongs to, who starts the game with this card
    /// or who creates this token. Do not modify this field.
    ///
    /// See <https://yawgatog.com/resources/magic-rules/#R1083>
    pub owner: PlayerName,

    #[serde(skip)]
    pub queries: CardQueries,

    /// Effects which are applying to this card to change its controller.
    ///
    /// The topmost effect represents the current controller. Use the
    /// [Self::controller] method to compute this.
    ///
    /// Each effect is tagged with the ability ID that created it, and abilities
    /// are responsible for removing effects they create when their durations
    /// expire.
    ///
    /// Do not modify this field directly, use the `change_controller` module
    /// instead.
    pub control_changing_effects: Vec<ControlChangingEffect>,

    /// Current game zone location for this card.
    ///
    /// Do not modify this field directly, use the `move_card` module instead.
    pub zone: Zone,

    /// Whether this card is currently face down or has one of its faces up.
    ///
    /// A card that is not on the battlefield is:
    /// - Face-down in the library or hand
    /// - Face-up in the graveyard, on the stack, or in the command zone.
    /// - Either face up or face down in exile depending on the effect that put
    ///   it there
    pub facing: CardFacing,

    /// The set of faces used to cast this card while it is on the stack.
    ///
    /// This is a single face except in the face of a split card cast with the
    /// "Fuse" ability.
    pub cast_as: EnumSet<Face>,

    /// Whether this card is current tapped.
    ///
    /// A card that is not on the battlefield is always untapped.
    pub tapped_state: TappedState,

    /// Whether this card is phased out.
    pub phasing_state: PhasingState,

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

    /// Targets for this card, selected when it is placed on the stack.
    ///
    /// Cards which are not on the stack cannot have targets.
    pub targets: Vec<EntityId>,

    /// The entity this card is attached to.
    ///
    /// Cards such as Equipment, Auras, and Fortifications can be attached to a
    /// permanent or player. Cards that are not on the battlefield cannot be
    /// attached to each other.
    pub attached_to: Option<EntityId>,

    /// Stores custom state entries for this card.
    ///
    /// See [CustomCardStateList].
    pub custom_state: CustomCardStateList,

    /// Turn on which this card last entered its current zone.
    ///
    /// Do not mutate this field directly, use the `move_card` module instead.
    pub entered_current_zone: TurnData,

    /// Turn on which this card gained its current controller.
    ///
    /// Used to e.g. determine whether creatures can attack in combat. Do not
    /// mutate this field directly, use the `change_controller` module instead.
    pub last_changed_control: TurnData,

    /// Printed Card associated with this card. Use the [Self::printed] method
    /// instead of accessing this directly.
    ///
    /// All cards must have a [PrintedCard] and this is populated immediately
    /// after deserialization. It should basically always be fine to .unwrap()
    /// this value by calling the [Self::printed] method.
    #[serde(skip)]
    pub printed_card_reference: Option<Arc<PrintedCard>>,
}

impl CardState {
    /// Returns the [EntityId] for this card.
    ///
    /// Panics if this card was assigned an invalid entity id.
    pub fn entity_id(&self) -> EntityId {
        EntityId::Card(self.id, self.object_id)
    }

    /// Returns this card's [PermanentId] if it is on the battlefield.
    pub fn permanent_id(&self) -> Option<PermanentId> {
        if self.zone == Zone::Battlefield {
            Some(PermanentId::new(self.object_id, self.id))
        } else {
            None
        }
    }
}

impl HasPlayerName for CardState {
    fn player_name(&self) -> PlayerName {
        self.owner
    }
}

impl HasController for CardState {
    /// The player who can currently make decisions about this card.
    ///
    /// For cards which are not currently on the battlefield or on the stack,
    /// this will be the card's owner.
    ///
    /// See <https://yawgatog.com/resources/magic-rules/#R1084>
    fn controller(&self) -> PlayerName {
        self.control_changing_effects.last().map_or(self.owner, |c| c.controller)
    }
}

impl CardState {
    /// Returns the [PrintedCard] for this card.
    pub fn printed(&self) -> &PrintedCard {
        self.printed_card_reference.as_ref().expect("PrintedCard reference not populated!")
    }

    /// Returns the name on the primary face of this card.
    pub fn displayed_name(&self) -> &str {
        &self.printed().face.displayed_name
    }

    /// Returns the [PrintedCardFace] for this card if it is currently face up.
    pub fn face_up_printed_face(&self) -> Option<&PrintedCardFace> {
        match self.facing {
            CardFacing::FaceDown => None,
            CardFacing::FaceUp(face) => Some(self.printed().face(face)),
        }
    }
}

/// Whether a card is tapped or untapped.
///
/// I assume within 10 years WoTC will introduce a third tapped state somehow,
/// so might as well make this an enum.
#[derive(Copy, Clone, Debug, Eq, PartialEq, Serialize, Deserialize, Type)]
#[serde(rename_all = "camelCase")]
pub enum TappedState {
    Untapped,
    Tapped,
}

impl TappedState {
    pub fn is_tapped(self) -> bool {
        self == TappedState::Tapped
    }
}

/// Facing for this card, corresponding to the [PrintedCard] faces.
#[derive(Copy, Clone, Debug, Eq, PartialEq, Serialize, Deserialize, Type)]
#[serde(rename_all = "camelCase")]
pub enum CardFacing {
    FaceDown,

    /// The indicated card face is currently up
    FaceUp(Face),
}

/// Represents an effect which changes the controller of a card.
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct ControlChangingEffect {
    pub effect_id: EffectId,
    pub controller: PlayerName,
}

/// Whether a card is phased out
#[derive(Copy, Clone, Debug, Eq, PartialEq, Serialize, Deserialize, Type)]
#[serde(rename_all = "camelCase")]
pub enum PhasingState {
    PhasedIn,
    PhasedOut,
}
