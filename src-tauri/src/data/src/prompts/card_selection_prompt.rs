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

use crate::core::primitives::CardId;

/// A prompt for a player to select one or more cards from a set of cards to
/// apply some effect to.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CardSelectionPrompt {
    /// Cards which should be displayed in the browser.
    ///
    /// For example, this would contain cards that should be kept in hand during
    /// the 'discard to hand size' flow.
    pub choices: Vec<CardId>,

    /// If true, the player seeing this prompt can rearrange the cards within
    /// the `target` position.
    pub can_reorder: bool,
}
