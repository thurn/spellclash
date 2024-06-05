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

use data::card_states::zones::ZoneQueries;
use data::core::primitives::{CardId, EntityId, PlayerName};
use data::delegates::scope::Scope;
use data::game_states::game_state::GameState;
use data::prompts::card_selection_prompt::{CardSelectionPrompt, Quantity};
use data::prompts::choice_prompt::{Choice, ChoicePrompt};
use data::prompts::game_update::GameUpdate;
use data::prompts::pick_number_prompt::PickNumberPrompt;
use data::prompts::prompt::{Prompt, PromptResponse, PromptType};
use data::text_strings::Text;
use tokio::sync::oneshot;

/// Sends a new [Prompt] to the player and blocks until they respond with a
/// [PromptResponse].
pub fn send(game: &mut GameState, prompt: Prompt) -> PromptResponse {
    let (sender, receiver) = oneshot::channel();
    game.updates
        .as_ref()
        .expect("PromptChannel")
        .send(GameUpdate::new(game).response_channel(sender));
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

pub fn select_cards(
    game: &mut GameState,
    player: PlayerName,
    description: Text,
    prompt: CardSelectionPrompt,
) -> Vec<CardId> {
    let PromptResponse::SelectCards(ids) = send(game, Prompt {
        player,
        label: Some(description),
        prompt_type: PromptType::SelectCards(prompt),
    }) else {
        panic!("Unexpected prompt response type!");
    };

    ids.clone()
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

/// Prompt to select a quantity of cards from controller's hand.
///
/// Allows reordering.
pub fn select_in_hand(
    game: &mut GameState,
    scope: Scope,
    quantity: Quantity,
    text: Text,
) -> Vec<CardId> {
    select_cards(
        game,
        scope.controller,
        text,
        CardSelectionPrompt::new(game.hand(scope.controller).iter().copied().collect())
            .can_reorder(true)
            .quantity(quantity),
    )
}
