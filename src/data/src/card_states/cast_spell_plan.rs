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

use enumset::{EnumSet, EnumSetType};
use serde::{Deserialize, Serialize};

use crate::core::numerics::ManaValue;
use crate::core::primitives::{CardId, ObjectId};
use crate::delegates::scope::AbilityId;
use crate::printed_cards::printed_card::Face;

/// Describes a proposed series of a choices for a user to play a card as part
/// of the "play card" game action.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CastSpellPlan {
    /// How the user will pay mana costs for this spell
    pub mana_payment: ManaPaymentPlan,
    /// Choices the user has selected for this spell
    pub choices: CastSpellChoices,
}

/// Describes a user's proposed plan for paying mana costs for a spell.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ManaPaymentPlan {
    /// Identifies cards the player has chosen to active as basic lands in order
    /// to pay costs for this spell. Lands with basic land subtypes are listed
    /// here because they do not have an explicit activated ability:
    ///
    /// > 305.6. An object with the land card type and a basic land type has the
    /// > intrinsic ability "{T}: Add [mana symbol]," even if the text box
    /// > doesn't actually contain that text or the object has no text box.
    /// <https://yawgatog.com/resources/magic-rules/#R3056>
    pub basic_land_abilities: Vec<CardId>,
    /// Identifies mana abilities the player has chosen to activate in order to
    /// pay costs to cast this spell.
    pub mana_abilities: Vec<AbilityId>,
}

/// Choices a player may make while placing a spell on the stack
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CastSpellChoices {
    /// The face of this card which the player is casting
    pub face: Face,
    /// Targets the player has chosen for this spell
    ///
    /// > 601.2c. The player announces their choice of an appropriate object or
    /// > player for each target the spell requires ... The same target can't be
    /// > chosen multiple times for any one instance of the word "target" on the
    /// > spell.
    /// <https://yawgatog.com/resources/magic-rules/#R6012c>
    pub targets: Vec<ObjectId>,
    /// Modal choices for this spell
    ///
    /// > 601.2b. If the spell is modal, the player announces the mode choice
    /// > (see rule 700.2).
    /// <https://yawgatog.com/resources/magic-rules/#R6012b>
    pub modes: EnumSet<ModalChoice>,
    /// Identifies an ability which provides an alternative cost which will be
    /// used to cast this spell
    pub alternative_cost: Option<AbilityId>,
    /// Identifies abilities adding additional choices the caster has chosen for
    /// this spell, such as optional costs like Kicker.
    ///
    /// > 601.2b. If the spell has alternative or additional costs that will be
    /// > paid as it's being cast such as buyback or kicker costs (see rules
    /// > 118.8 and 118.9), the player announces their intentions to pay any or
    /// > all of those costs (see rule 601.2f). A player can't apply two
    /// > alternative methods of casting or two alternative costs to a single
    /// > spell.
    /// <https://yawgatog.com/resources/magic-rules/#R6012b>
    pub additional_choices: Vec<CastSpellPlanAdditionalChoice>,
    /// The chosen value for an "X" variable in a spell's casting cost
    ///
    /// > 601.2b. If the spell has a variable cost that will be paid as it's
    /// > being cast (such as an {X} in its mana cost; see rule 107.3), the
    /// > player announces the value of that variable. If the value of that
    /// > variable is defined in the text of the spell by a choice that player
    /// > would make later in the announcement or resolution of the spell, that
    /// > player makes that choice at this time instead of that later time.
    /// <https://yawgatog.com/resources/magic-rules/#R6012b>
    pub variable: Option<ManaValue>,
}

impl Default for CastSpellChoices {
    fn default() -> Self {
        Self {
            face: Face::Primary,
            targets: Vec::new(),
            modes: EnumSet::empty(),
            alternative_cost: None,
            additional_choices: Vec::new(),
            variable: None,
        }
    }
}

/// A choice of mode a user makes while casting a spell.
///
/// > 700.2. A spell or ability is modal if it has two or more options in a
/// > bulleted list preceded by instructions for a player to choose a number of
/// > those options, such as "Choose one --." Each of those options is a mode.
/// <https://yawgatog.com/resources/magic-rules/#R7002>
#[derive(Debug, EnumSetType, Serialize, Deserialize)]
pub enum ModalChoice {
    One,
    Two,
    Three,
    Four,
    Fix,
    Six,
}

/// Extra choices a player can make while casting a spell
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum CastSpellPlanAdditionalChoice {
    /// Ability with an additional cost the player has *chosen* to pay for this
    /// spell, such as Kicker. Does not include additional costs the player is
    /// *forced* to pay.
    AdditionalCostChoice(AbilityId),
    /// Splice this spell with the indicated card
    ///
    /// > 601.2b. If the player wishes to splice any cards onto the spell (see
    /// > rule 702.47), they reveal those cards in their hand.
    /// <https://yawgatog.com/resources/magic-rules/#R6012b>
    SpliceWith(CardId),
}
