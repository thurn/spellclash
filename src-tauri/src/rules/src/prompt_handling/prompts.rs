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
use data::prompts::choice_prompt::{Choice, ChoicePrompt};
use data::prompts::game_update::GameUpdate;
use data::prompts::pick_number_prompt::PickNumberPrompt;
use data::prompts::prompt::{Prompt, PromptResponse, PromptType};
use data::prompts::select_order_prompt::{CardOrderLocation, Quantity, SelectOrderPrompt};
use data::text_strings::Text;
use enumset::EnumSet;
use maplit::hashmap;
use tokio::sync::oneshot;
use tracing::info;
use utils::outcome::PromptResult;

/// Sends a new [Prompt] to the player and blocks until they respond with a
/// [PromptResponse].
pub fn send(game: &mut GameState, prompt: Prompt) -> PromptResult<PromptResponse> {
    let kind = prompt.prompt_type.kind();
    info!(immediate = true, ?kind, "Sending prompt");
    let (sender, receiver) = oneshot::channel();
    game.updates
        .as_ref()
        .expect("PromptChannel")
        .send(GameUpdate::new(game).prompt(prompt).response_channel(sender));
    let result =
        receiver.blocking_recv().expect("Unable to receive prompt response, sender has dropped");
    let result_kind = result.kind();
    info!(?result_kind, "Got prompt response");
    Ok(result)
}

pub fn choose_entity(
    game: &mut GameState,
    player: PlayerName,
    description: Text,
    choices: Vec<Choice<EntityId>>,
) -> PromptResult<EntityId> {
    let PromptResponse::EntityChoice(id) = send(game, Prompt {
        player,
        label: Some(description),
        prompt_type: PromptType::EntityChoice(ChoicePrompt { optional: false, choices }),
    })?
    else {
        panic!("Unexpected prompt response type!");
    };
    Ok(id)
}

/// Prompt for the [PlayerName] player to select and reorder cards based on a
/// [SelectOrderPrompt].
pub fn select_order(
    game: &mut GameState,
    player: PlayerName,
    description: Text,
    prompt: SelectOrderPrompt,
) -> PromptResult<HashMap<CardOrderLocation, Vec<CardId>>> {
    let PromptResponse::SelectOrder(ids) = send(game, Prompt {
        player,
        label: Some(description),
        prompt_type: PromptType::SelectOrder(prompt),
    })?
    else {
        panic!("Unexpected prompt response type!");
    };

    Ok(ids)
}

/// Show a [PickNumberPrompt].
pub fn pick_number(
    game: &mut GameState,
    player: PlayerName,
    description: Text,
    prompt: PickNumberPrompt,
) -> PromptResult<u32> {
    let PromptResponse::PickNumber(number) = send(game, Prompt {
        player,
        label: Some(description),
        prompt_type: PromptType::PickNumber(prompt),
    })?
    else {
        panic!("Unexpected prompt response type!");
    };
    Ok(number)
}

/// Prompt the controller to select a `quantity` from the provided unordered
/// list of cards to move to a new `target` location.
///
/// This is typically used when the cards in question do not already exist in an
/// ordered location, e.g. for selecting cards from hand to place on top of the
/// library.
pub fn select_ordered_from<'a>(
    game: &mut GameState,
    scope: Scope,
    text: Text,
    cards: impl IntoIterator<Item = &'a CardId>,
    quantity: usize,
    target: CardOrderLocation,
) -> PromptResult<Vec<CardId>> {
    Ok(select_order(
        game,
        scope.controller,
        text,
        SelectOrderPrompt::new(hashmap! {
            CardOrderLocation::Unordered => cards.into_iter().copied().collect(),
            target => vec![]
        })
        .quantity(Quantity::Ordered(quantity)),
    )?
    .remove(&target)
    .unwrap_or_default())
}
