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

use std::fmt::{Display, Formatter};

use enumset::{EnumSet, EnumSetType};
use primitives::game_primitives::{AbilityId, CardId, Color, EntityId, PermanentId, PlayerName};

use crate::core::numerics::ManaValue;
use crate::printed_cards::printed_card::Face;
use crate::text_strings::Text;

/// Describes a proposed series of a choices for a user to play a card as part
/// of the "play card" game action.
#[derive(Debug, Clone)]
pub struct PlayCardPlan {
    /// Choices made as part of playing a card prior to target selection &
    /// paying mana costs
    pub choices: PlayCardChoices,

    /// Targets the player has chosen for this spell
    ///
    /// > 601.2c. The player announces their choice of an appropriate object or
    /// > player for each target the spell requires ... The same target can't be
    /// > chosen multiple times for any one instance of the word "target" on the
    /// > spell.
    /// <https://yawgatog.com/resources/magic-rules/#R6012c>
    pub targets: Vec<EntityId>,

    /// How the user will pay mana costs for this card if it is a spell
    pub mana_payment: ManaPaymentPlan,
}

impl PlayCardPlan {
    pub fn new(controller: PlayerName, play_as: PlayAs) -> Self {
        Self {
            choices: PlayCardChoices {
                controller,
                play_as,
                modes: Vec::new(),
                alternative_cost: None,
                additional_choices: Vec::new(),
                variable: None,
            },
            targets: Vec::new(),
            mana_payment: ManaPaymentPlan::default(),
        }
    }
}

/// Choices made as part of playing a card prior to target selection & paying
/// mana costs.
#[derive(Debug, Clone)]
pub struct PlayCardChoices {
    /// The player who is playing this card
    pub controller: PlayerName,

    /// The face or faces of this card which the player is casting and the
    /// timing restriction used for playing this card.
    ///
    /// This will be a single face for most cards, but split cards with the
    /// "Fuse" ability can be cast using multiple faces at once.
    pub play_as: PlayAs,

    /// Modal choices selected for this spell
    ///
    /// > 601.2b. If the spell is modal, the player announces the mode choice
    /// > (see rule 700.2).
    /// <https://yawgatog.com/resources/magic-rules/#R6012b>
    pub modes: Vec<ModalChoice>,

    /// Identifies an ability which provides an alternative cost or casting
    /// method which will be used to cast this spell
    ///
    /// > A player can't apply two alternative methods of casting or two
    /// > alternative costs to a single spell.
    ///
    /// <https://yawgatog.com/resources/magic-rules/#R6012b>
    pub alternative_cost: Option<AbilityId>,

    /// Identifies abilities adding additional choices the caster has chosen for
    /// this spell, such as optional costs like Kicker.
    ///
    /// > 601.2b. If the spell has alternative or additional costs that will be
    /// > paid as it's being cast such as buyback or kicker costs (see rules
    /// > 118.8 and 118.9), the player announces their intentions to pay any or
    /// > all of those costs (see rule 601.2f).
    ///
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

/// Describes a user's proposed plan for paying mana costs for a spell.
#[derive(Debug, Clone, Default)]
pub struct ManaPaymentPlan {
    /// Identifies cards the player has chosen to active via their basic land
    /// abilities in order to pay costs for this spell. Lands with basic
    /// land subtypes are listed here instead of in [Self::mana_abilities]
    /// because they do not have an explicit activated ability.
    ///
    /// > 305.6. An object with the land card type and a basic land type has the
    /// > intrinsic ability "{T}: Add [mana symbol]," even if the text box
    /// > doesn't actually contain that text or the object has no text box.
    /// <https://yawgatog.com/resources/magic-rules/#R3056>
    pub basic_land_abilities_to_activate: Vec<PermanentId>,
    /// Identifies mana abilities the player has chosen to activate in order to
    /// pay costs to cast this spell.
    pub mana_abilities: Vec<AbilityId>,
}

/// Describes how a face of card can be played.
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum PlayCardTiming {
    Sorcery,
    Instant,
    Land,
}

/// Describes how a face of card can be played.
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct PlayAs {
    /// Set of faces being played
    pub faces: EnumSet<Face>,

    /// Timing restriction on playing this card
    pub timing: PlayCardTiming,
}

impl PlayAs {
    /// Returns the single face being played, or panics if more than one face is
    /// being played.
    pub fn single_face(&self) -> Face {
        self.faces.iter().next().expect("Expected exactly one face")
    }
}

/// A choice of mode a user makes while casting a spell. Index position within
/// the choice list.
///
/// > 700.2. A spell or ability is modal if it has two or more options in a
/// > bulleted list preceded by instructions for a player to choose a number of
/// > those options, such as "Choose one --." Each of those options is a mode.
/// <https://yawgatog.com/resources/magic-rules/#R7002>
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct ModalChoice(pub usize);

impl Display for ModalChoice {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0 + 1)
    }
}

impl From<ModalChoice> for Text {
    fn from(value: ModalChoice) -> Self {
        Text::ModalChoice(value)
    }
}

/// Extra choices a player can make while casting a spell
#[derive(Debug, Clone)]
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
