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

use serde::{Deserialize, Serialize};

use crate::actions::game_action::GameAction;
use crate::actions::user_action::UserAction;

/// Action to respond to a prompt within an ongoing game
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug, Serialize, Deserialize)]
pub enum PromptAction {
    /// Pick an integer for a number selection prompt
    PickNumber(u32),

    /// Move a card from the provided 'source' index in a card selection prompt
    /// to the 'target' index in the selected card list, or to the end of the
    /// selected card list if no target is provided.
    SelectCard { source: usize, target: Option<usize> },

    /// Moves one of the selected cards in a card selection prompt from the
    /// 'source' index in the selected card list to the 'target' index.
    SetSelectionOrder { source: usize, target: usize },

    /// Confirm selected card choices on a card selection prompt
    SubmitCardSelection,
}

impl From<PromptAction> for UserAction {
    fn from(value: PromptAction) -> Self {
        GameAction::PromptAction(value).into()
    }
}
