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

use std::collections::BTreeMap;

use primitives::game_primitives::{CardId, EntityId, PlayerName};
use serde::{Deserialize, Serialize};
use serde_with::serde_as;
use strum::EnumDiscriminants;

use crate::prompts::entity_choice_prompt::EntityChoicePrompt;
use crate::prompts::multiple_choice_prompt::MultipleChoicePromptTrait;
use crate::prompts::pick_number_prompt::PickNumberPrompt;
use crate::prompts::play_cards_prompt::PlayCardsPrompt;
use crate::prompts::select_order_prompt::{CardOrderLocation, SelectOrderPrompt};
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
#[derive(Clone, Debug, EnumDiscriminants)]
#[strum_discriminants(name(PromptTypeKind))]
pub enum PromptType {
    EntityChoice(EntityChoicePrompt<EntityId>),
    SelectOrder(SelectOrderPrompt),
    PlayCards(PlayCardsPrompt),
    PickNumber(PickNumberPrompt),
    MultipleChoice(Box<dyn MultipleChoicePromptTrait>),
}

impl PromptType {
    pub fn kind(&self) -> PromptTypeKind {
        self.into()
    }
}

#[serde_as]
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SelectedOrder {
    #[serde_as(as = "Vec<(_, _)>")]
    pub order: BTreeMap<CardOrderLocation, Vec<CardId>>,
}

impl SelectedOrder {
    pub fn new(order: BTreeMap<CardOrderLocation, Vec<CardId>>) -> Self {
        Self { order }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, EnumDiscriminants)]
#[strum_discriminants(name(PromptResponseKind))]
pub enum PromptResponse {
    EntityChoice(EntityId),
    SelectOrder(SelectedOrder),
    PlayCards(Vec<CardId>),
    PickNumber(u32),
    MultipleChoice(usize),
}

impl PromptResponse {
    pub fn kind(&self) -> PromptResponseKind {
        self.into()
    }
}
