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

use crate::core::primitives::PlayerName;
use crate::prompts::card_selection_prompt::CardSelectionPrompt;
use crate::prompts::choice_prompt::ChoicePrompt;
use crate::prompts::play_cards_prompt::PlayCardsPrompt;
use crate::text_strings::Text;

/// Data for showing a prompt to a player.
///
/// Prompts allow players to make a choice within the game interface.
#[derive(Clone, Debug)]
pub struct Prompt {
    /// Player who is being prompted
    pub player: PlayerName,
    /// Optionally, a label to display describing the choice being made
    pub label: Option<Text>,
    /// Which type of prompt to show
    pub prompt_type: PromptType,
}

/// Possible types of prompts
#[derive(Clone, Debug)]
pub enum PromptType {
    /// A blocking choice for a player to pick one of a list of options before
    /// any other game actions can occur.
    Choice(ChoicePrompt),

    /// A prompt for a player to select one or more cards from a set of cards to
    /// apply some effect to.
    SelectCards(CardSelectionPrompt),

    /// A prompt for a player to play one or more cards from a set of cards.
    PlayCards(PlayCardsPrompt),
}
