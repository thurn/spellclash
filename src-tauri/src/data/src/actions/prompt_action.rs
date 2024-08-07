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

use primitives::game_primitives::{CardId, EntityId};
use serde::{Deserialize, Serialize};

use crate::actions::user_action::UserAction;
use crate::prompts::select_order_prompt::CardOrderLocation;

/// Action to respond to a prompt within an ongoing game
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug, Ord, PartialOrd, Serialize, Deserialize)]
pub enum PromptAction {
    /// Pick an integer for a number selection prompt
    PickNumber(u32),

    /// Sets the order of a card in a card select & order prompt.
    ///
    /// The provided index is interpreted relative to other cards already in
    /// this location. The card currently occupying this location will be pushed
    /// towards the end of the list (right).
    SelectOrder(CardId, CardOrderLocation, usize),

    /// Pick a single entity from a set of choices.
    SelectEntity(EntityId),

    /// Confirm selected card choices on a card selection prompt
    SubmitCardSelection,

    /// Pick a choice at a given index in a multiple choice prompt
    SelectChoice(usize),
}

impl From<PromptAction> for UserAction {
    fn from(value: PromptAction) -> Self {
        UserAction::PromptAction(value)
    }
}
