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

use std::fmt::{Debug, Formatter};

use primitives::game_primitives::Color;

use crate::core::card_tags::CardTag;
use crate::core::numerics::{Power, Toughness};
use crate::printed_cards::card_subtypes::{CreatureType, LandType};
use crate::properties::card_property::CardProperty;
use crate::properties::card_property_data::{CanAttackTarget, CanBeBlocked};
use crate::properties::flag::Flag;
use crate::properties::property_value::{ChangeText, EnumSets, Ints};

#[derive(Default, Clone)]
pub struct CardProperties {
    /// Queries tags on this card
    pub tags: CardProperty<EnumSets<CardTag>>,

    /// Can this creature attack the indicated target?
    pub can_attack_target: CardProperty<Flag<CanAttackTarget>>,

    /// Can this creature be blocked by the indicated blocker?
    pub can_be_blocked: CardProperty<Flag<CanBeBlocked>>,

    /// 'Haste' effect. Can this creature attack on the same turn it is played,
    /// or immediately after switching controllers?
    pub can_attack_same_turn: CardProperty<Flag<()>>,

    /// Queries the colors of a card.
    ///
    /// An empty set represents colorless.
    pub colors: CardProperty<EnumSets<Color>>,

    /// Queries the creature subtypes of a card.
    pub creature_types: CardProperty<EnumSets<CreatureType>>,

    /// Queries the land subtypes of a card.
    pub land_types: CardProperty<EnumSets<LandType>>,

    /// Queries for text-changing effects to change a land subtype in the rules
    /// text of a card
    pub change_land_type_text: CardProperty<ChangeText<LandType>>,

    /// Queries for text-changing effects to change a color in the rules
    /// text of a card
    pub change_color_text: CardProperty<ChangeText<Color>>,

    /// Queries the power value for a card.
    ///
    /// This may be invoked for a card in any zone.
    pub power: CardProperty<Ints<Power>>,

    /// Queries the base power value for a card. This is added to other
    /// modifiers to compute a final power value.
    pub base_power: CardProperty<Ints<Power>>,

    /// Queries the toughness value for a card.
    ///
    /// This may be invoked for a card in any zone.
    pub toughness: CardProperty<Ints<Toughness>>,

    /// Queries the base toughness value for a card. This is added to other
    /// modifiers to compute a final power value.
    pub base_toughness: CardProperty<Ints<Toughness>>,
}

impl Debug for CardProperties {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("CardQueries").finish()
    }
}
