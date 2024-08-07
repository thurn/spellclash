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

use primitives::game_primitives::AbilityNumber;

use crate::card_definitions::ability_definition::Ability;
use crate::card_definitions::card_name::CardName;
#[allow(unused)] // Used in docs
use crate::printed_cards::printed_card::PrintedCard;

/// Contains the game rules definition for a card.
///
/// This is combined with the card's Oracle information from the [PrintedCard]
/// struct to determine overall card behavior. Cards are implemented as zero
/// argument functions which return an instance of this struct.
pub struct CardDefinition {
    /// Name of this card, used to connect the card to its [PrintedCard]
    /// equivalent.
    name: CardName,
    /// Abilities of this card, which describe how it modifies game rules & game
    /// state.
    abilities: Vec<Box<dyn Ability>>,
}

impl CardDefinition {
    pub fn new(name: CardName) -> Self {
        Self { name, abilities: vec![] }
    }

    pub fn card_name(&self) -> CardName {
        self.name
    }

    /// Adds a new ability to this card definition
    ///
    /// Each clause of the card's oracle text_strings should correspond to one
    /// ability in sequence.
    pub fn ability(mut self, ability: impl Ability + 'static) -> Self {
        self.abilities.push(Box::new(ability));
        self
    }

    /// Iterates over all abilities of this card with their [AbilityNumber]s.
    pub fn iterate_abilities(&self) -> impl Iterator<Item = (AbilityNumber, &dyn Ability)> {
        self.abilities.iter().enumerate().map(|(i, a)| (AbilityNumber(i), a.as_ref()))
    }

    /// Looks up an ability by its [AbilityNumber].
    pub fn get_ability(&self, ability_number: AbilityNumber) -> &dyn Ability {
        self.abilities[ability_number.0].as_ref()
    }
}
