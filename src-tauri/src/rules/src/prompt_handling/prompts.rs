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

use data::card_states::zones::ZoneQueries;
use data::core::primitives::{CardId, EntityId, PlayerName};
use data::delegates::scope::Scope;
use data::game_states::game_state::GameState;
use data::prompts::card_select_and_order_prompt::{
    CardOrderLocation, CardSelectOrderPrompt, Quantity,
};
use data::prompts::choice_prompt::{Choice, ChoicePrompt};
use data::prompts::game_update::GameUpdate;
use data::prompts::pick_number_prompt::PickNumberPrompt;
use data::prompts::prompt::{Prompt, PromptResponse, PromptType};
use data::text_strings::Text;
use enumset::EnumSet;
use tokio::sync::oneshot;
use tracing::info;

/// Sends a new [Prompt] to the player and blocks until they respond with a
/// [PromptResponse].
pub fn send(game: &mut GameState, prompt: Prompt) -> PromptResponse {
    let kind = prompt.prompt_type.kind();
    info!(immediate = true, ?kind, "Sending prompt");
    let (sender, receiver) = oneshot::channel();
    game.updates
        .as_ref()
        .expect("PromptChannel")
        .send(GameUpdate::new(game).prompt(prompt).response_channel(sender));
    receiver.blocking_recv().expect("Unable to receive prompt response, sender has dropped")
}

pub fn choose_entity(
    game: &mut GameState,
    player: PlayerName,
    description: Text,
    choices: Vec<Choice<EntityId>>,
) -> EntityId {
    let PromptResponse::EntityChoice(id) = send(game, Prompt {
        player,
        label: Some(description),
        prompt_type: PromptType::EntityChoice(ChoicePrompt { optional: false, choices }),
    }) else {
        panic!("Unexpected prompt response type!");
    };
    id
}

/// Prompt for the [PlayerName] player to select and reorder cards based on a
/// [CardSelectOrderPrompt].
pub fn select_order(
    game: &mut GameState,
    player: PlayerName,
    description: Text,
    prompt: CardSelectOrderPrompt,
) -> HashMap<CardOrderLocation, Vec<CardId>> {
    let PromptResponse::SelectOrder(ids) = send(game, Prompt {
        player,
        label: Some(description),
        prompt_type: PromptType::SelectOrder(prompt),
    }) else {
        panic!("Unexpected prompt response type!");
    };

    ids
}

/// Show a [PickNumberPrompt].
pub fn pick_number(
    game: &mut GameState,
    player: PlayerName,
    description: Text,
    prompt: PickNumberPrompt,
) -> u32 {
    let PromptResponse::PickNumber(number) = send(game, Prompt {
        player,
        label: Some(description),
        prompt_type: PromptType::PickNumber(prompt),
    }) else {
        panic!("Unexpected prompt response type!");
    };
    number
}

/// Prompt to select a [Quantity] of cards from controller's hand and place them
/// on top of the library in a chosen order. Returns the list of selected cards
/// in order.
pub fn hand_to_top_of_library(
    game: &mut GameState,
    scope: Scope,
    quantity: Quantity,
) -> Vec<CardId> {
    select_order(game, scope.controller, Text::HandToTopOfLibraryPrompt, CardSelectOrderPrompt {
        choices: game.hand(scope.controller).iter().copied().collect(),
        locations: EnumSet::only(CardOrderLocation::TopOfLibrary),
        ordered: HashMap::new(),
        quantity,
    })
    .remove(&CardOrderLocation::TopOfLibrary)
    .unwrap_or_default()
}
