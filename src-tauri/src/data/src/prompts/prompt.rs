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

use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use strum::EnumDiscriminants;

use crate::core::primitives::{CardId, EntityId, PlayerName};
use crate::prompts::card_select_and_order_prompt::{CardOrderLocation, CardSelectOrderPrompt};
use crate::prompts::choice_prompt::ChoicePrompt;
use crate::prompts::pick_number_prompt::PickNumberPrompt;
use crate::prompts::play_cards_prompt::PlayCardsPrompt;
use crate::text_strings::Text;

/// Data for showing a prompt to a player.
///
/// Prompts allow players to make a choice within the game interface.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Prompt {
    /// Player who is being prompted
    pub player: PlayerName,
    /// Optionally, a label to display describing the choice being made
    pub label: Option<Text>,
    /// Which type of prompt to show
    pub prompt_type: PromptType,
}

/// Possible types of prompts
#[derive(Clone, Debug, Serialize, Deserialize, EnumDiscriminants)]
#[strum_discriminants(name(PromptTypeKind))]
pub enum PromptType {
    /// A blocking choice for a player to pick one of a list of entities before
    /// any other game action_handlers can occur.
    EntityChoice(ChoicePrompt<EntityId>),

    /// A prompt for a player to select and/or reorder cards
    SelectOrder(CardSelectOrderPrompt),

    /// A prompt for a player to play one or more cards from a set of cards.
    PlayCards(PlayCardsPrompt),

    /// Pick an integer value
    PickNumber(PickNumberPrompt),
}

impl PromptType {
    pub fn kind(&self) -> PromptTypeKind {
        self.into()
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum PromptResponse {
    EntityChoice(EntityId),
    SelectOrder(HashMap<CardOrderLocation, Vec<CardId>>),
    PlayCards(Vec<CardId>),
    PickNumber(u32),
}
