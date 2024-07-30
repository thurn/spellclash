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

use data::actions::user_action::UserAction;
use data::card_states::card_state::{CardFacing, TappedState};
use data::core::numerics::Damage;
use data::printed_cards::layout::{CardLayout, FaceLayout};
use data::prompts::select_order_prompt::CardOrderLocation;
use primitives::game_primitives::{CardId, StackAbilityId};
use serde::{Deserialize, Serialize};
use specta::Type;

use crate::core::object_position::ObjectPosition;

/// Represents the visual state of a card or ability in a game
#[derive(Clone, Debug, Serialize, Deserialize, Type)]
#[serde(rename_all = "camelCase")]
pub struct CardView {
    /// Identifier for this card
    pub id: ClientCardId,

    /// Position of this card in the UI
    pub position: ObjectPosition,

    /// Card back image
    pub card_back: String,

    /// If this card is revealed to the viewer, contains information on the
    /// revealed face of the card.
    pub revealed: Option<RevealedCardView>,

    /// True if this card is in a hidden zone but known to one or more opponents
    pub revealed_to_opponents: bool,

    /// Face up/face down state for this card
    pub card_facing: CardFacing,

    /// Tapped/untapped state for this card
    pub tapped_state: TappedState,

    /// Damage marked on this card
    ///
    /// Note that the rules engine internally uses 64-bit integers, but in the
    /// display layer we use floats for JavaScript compatibility.
    pub damage: f64,

    /// Optionally, a position at which to create this card.
    ///
    /// If this card does not already exist, it will be created at this position
    /// before being animated to [Self::position].
    pub create_position: Option<ObjectPosition>,

    /// Optionally, a position at which to destroy this card.
    ///
    /// If provided, the card will be animated to this position before being
    /// destroyed.
    pub destroy_position: Option<ObjectPosition>,
}

/// Identifies a card in client code
///
/// Serialized u64, represented as string because JavaScript is a silly
/// language.
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize, Type)]
#[serde(rename_all = "camelCase")]
pub enum ClientCardId {
    CardId(String),
    StackAbilityId(String),
}

impl ClientCardId {
    pub fn new(card_id: CardId) -> Self {
        Self::CardId(card_id.to_ffi_value().to_string())
    }

    pub fn new_for_stack_ability(stack_ability_id: StackAbilityId) -> Self {
        Self::StackAbilityId(stack_ability_id.to_ffi_value().to_string())
    }

    pub fn to_card_id(&self) -> CardId {
        let ClientCardId::CardId(id) = self else {
            panic!("Expected card ID");
        };
        CardId::from_ffi_value(id.parse().expect("Invalid client card ID"))
    }

    pub fn to_stack_ability_id(&self) -> StackAbilityId {
        let ClientCardId::StackAbilityId(id) = self else {
            panic!("Expected stack ability ID");
        };
        StackAbilityId::from_ffi_value(id.parse().expect("Invalid client stack ability ID"))
    }
}

/// Visual state of a revealed card
#[derive(Clone, Debug, Serialize, Deserialize, Type)]
#[serde(rename_all = "camelCase")]
pub struct RevealedCardView {
    /// Image URL for this card
    ///
    /// For double-faced cards, this is the image of the face which is currently
    /// face-up. For other kinds of multi-faced cards, this is an image
    /// containing both faces.
    pub image: String,

    /// Primary face of this card
    pub face: RevealedCardFace,

    /// Visual status of this card
    pub status: Option<RevealedCardStatus>,

    /// True if this card represents an ability
    pub is_ability: bool,

    /// Action to take when this card is clicked, if any.
    pub click_action: Option<UserAction>,

    /// True if this card can be dragged by the player.
    ///
    /// The set of valid drag targets is set on the GameView. All draggable
    /// cards can be dragged to and reordered within any valid target.
    pub can_drag: bool,

    /// Secondary or additional face of this card, if any
    pub face_b: Option<RevealedCardFace>,

    /// Visual style of this card, how the faces are displayed
    pub layout: CardLayout,
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize, Type)]
#[serde(rename_all = "camelCase")]
pub enum RevealedCardStatus {
    Selected,
    CanSelect,
    CanPlay,
    Attacking(String),
    Blocking(String),
}

/// Visual state of a revealed card face
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize, Type)]
#[serde(rename_all = "camelCase")]
pub struct RevealedCardFace {
    /// Name of this face
    pub name: String,

    /// Visual style of specifically this face
    pub layout: FaceLayout,

    /// Rules text_strings for this face, if any.
    pub rules_text: Option<String>,
}
