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

use crate::core::card_tags::CardTag;
use crate::core::numerics::{Power, Toughness};
use crate::core::primitives::{CardId, Color};
use crate::delegates::card_query_delegate_list::CardQueryDelegateList;
use crate::delegates::game_delegate_data::{CanAttackTarget, CanBeBlocked};
use crate::delegates::query_value::{ChangeText, EnumSets, Ints};
use crate::printed_cards::card_subtypes::{CreatureType, LandSubtype};
use crate::queries::card_query::{CardArgumentQuery, CardQuery};
use crate::queries::flag::Flag;

#[derive(Default, Clone)]
pub struct CardQueries {
    /// Queries tags on this card
    pub tags: CardQuery<EnumSets<CardTag>>,

    /// Can this creature attack the indicated target?
    pub can_attack_target: CardArgumentQuery<CanAttackTarget, Flag<CanAttackTarget>>,

    /// Can this creature be blocked by the indicated blocker?
    pub can_be_blocked: CardArgumentQuery<CanBeBlocked, Flag<CanBeBlocked>>,

    /// Queries the colors of a card.
    ///
    /// An empty set represents colorless.
    pub colors: CardQuery<EnumSets<Color>>,

    /// Queries the creature subtypes of a card.
    pub creature_types: CardQuery<EnumSets<CreatureType>>,

    /// Queries the land subtypes of a card.
    pub land_types: CardQuery<EnumSets<LandSubtype>>,

    /// Queries for text-changing effects to change a land subtype in the rules
    /// text of a card
    pub change_land_type_text: CardQuery<ChangeText<LandSubtype>>,

    /// Queries for text-changing effects to change a color in the rules
    /// text of a card
    pub change_color_text: CardQuery<ChangeText<Color>>,

    /// Queries the power value for a card.
    ///
    /// This may be invoked for a card in any zone.
    pub power: CardQuery<Ints<Power>>,

    /// Queries the base power value for a card. This is added to other
    /// modifiers to compute a final power value.
    pub base_power: CardQuery<Ints<Power>>,

    /// Queries the toughness value for a card.
    ///
    /// This may be invoked for a card in any zone.
    pub toughness: CardQuery<Ints<Toughness>>,

    /// Queries the base toughness value for a card. This is added to other
    /// modifiers to compute a final power value.
    pub base_toughness: CardQuery<Ints<Toughness>>,
}

impl Debug for CardQueries {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("CardQueries").finish()
    }
}
