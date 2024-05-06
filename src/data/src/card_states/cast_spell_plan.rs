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

/// Describes a proposed series of a choices for a user to play a card as part
/// of the "play card" game action.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CastSpellPlan {
    /// Identifies mana abilities the player has chosen to activate in order to
    /// pay costs to cast this spell.
    pub mana_abilities: Vec<AbilityId>,
    /// Identifies non-mana abilities the player has chosen to activate in order
    /// to pay costs to cast this spell.
    pub non_mana_cost_abilities: Vec<AbilityId>,
    /// Targets the player has chosen for this spell
    pub targets: Vec<ObjectId>,
    /// Modal choices for this spell
    pub modes: EnumSet<ModalChoice>,
    /// Identifies an ability which provides an alternative cost which will be
    /// used to cast this spell
    pub alternative_cost: Option<AbilityId>,
    /// Identifies abilities adding additional choices the caster has chosen for
    /// this spell, such as optional costs like Kicker.
    pub additional: Vec<CastSpellPlanAdditional>,
    /// The chosen value for an "X" variable in a spell's casting cost
    pub variable: Option<ManaValue>,
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

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum CastSpellPlanAdditional {
    /// Ability with an additional cost the player has *chosen* to pay for this
    /// spell, such as Kicker. Does not include additional costs the player is
    /// *forced* to pay.
    AdditionalCostChoice(AbilityId),
    /// Splice this spell with the indicated card
    SpliceWith(CardId),
}
