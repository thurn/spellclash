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

use crate::core::primitives::CardId;
use crate::game_states::game_state::GameState;

/// A prompt for a player to select one or more cards from a set of cards to
/// apply some effect to.
#[derive(Clone, Debug)]
pub struct CardSelectionPrompt {
    /// Cards which should be displayed in the browser and which have not
    /// been selected by dragging them to the target. Initially, this should
    /// contain all subject cards. As cards are dragged in the UI, they will be
    /// removed from this list and added to [Self::chosen_subjects].
    ///
    /// For example, this would contain cards that should be kept in hand during
    /// the 'discard to hand size' flow.
    pub unchosen_subjects: Vec<CardId>,

    /// Cards which have been selected so far, e.g. the cards that should be
    /// discarded when performing the 'discard to hand size' flow. This
    /// should initially be empty.
    pub chosen_subjects: Vec<CardId>,

    /// If true, the player seeing this prompt can rearrange the cards within
    /// the `target` position. The chosen order will be reflected in the vector
    /// passed to [Self::callback].
    pub can_reorder: bool,

    /// Callback invoked when cards are selected
    pub callback: fn(&mut GameState, Vec<CardId>),
}
