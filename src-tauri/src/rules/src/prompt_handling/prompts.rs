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
use data::delegates::scope::{DelegateScope, Scope};
use data::game_states::game_state::GameState;
use data::player_states::player_state::{PlayerQueries, PlayerType};
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

use crate::action_handlers::prompt_actions;
use crate::action_handlers::prompt_actions::PromptExecutionResult;

/// Sends a new [Prompt] to the player and blocks until they respond with a
/// [PromptResponse].
pub fn send(game: &mut GameState, mut prompt: Prompt) -> PromptResponse {
    let agent_player = game.current_search_agent.unwrap_or(prompt.player);
    if let (Some(agent), Some(prompt_agent)) =
        (game.player(agent_player).agent(), game.player(agent_player).prompt_agent())
    {
        let ongoing = game.current_search_agent.is_some();
        loop {
            let action = if ongoing {
                agent.incremental_prompt_action(game, &prompt, prompt.player)
            } else {
                prompt_agent.top_level_prompt_action(game, &prompt, prompt.player)
            };
            match prompt_actions::execute(prompt, action) {
                PromptExecutionResult::Prompt(p) => {
                    prompt = p;
                }
                PromptExecutionResult::PromptResponse(response) => {
                    return response;
                }
            }
        }
    } else {
        let kind = prompt.prompt_type.kind();
        info!(immediate = true, ?kind, "Sending prompt");
        let (sender, receiver) = oneshot::channel();
        game.updates
            .as_ref()
            .expect("PromptChannel")
            .send(GameUpdate::new(game).prompt(prompt).response_channel(sender))
            .expect("Unable to send prompt, receiver has dropped");
        let result = receiver
            .blocking_recv()
            .expect("Unable to receive prompt response, sender has dropped");
        let result_kind = result.kind();
        info!(?result_kind, "Got prompt response");
        result
    }
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
/// [SelectOrderPrompt].
pub fn select_order(
    game: &mut GameState,
    player: PlayerName,
    description: Text,
    prompt: SelectOrderPrompt,
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

/// Prompt the controller to select a `quantity` from the provided unordered
/// list of cards to move to a new `target` location.
///
/// This is typically used when the cards in question do not already exist in an
/// ordered location, e.g. for selecting cards from hand to place on top of the
/// library.
pub fn select_ordered_from<'a>(
    game: &mut GameState,
    scope: impl Scope,
    text: Text,
    cards: impl IntoIterator<Item = &'a CardId>,
    quantity: usize,
    target: CardOrderLocation,
) -> Vec<CardId> {
    select_order(
        game,
        scope.controller(),
        text,
        SelectOrderPrompt::new(hashmap! {
            CardOrderLocation::Unordered => cards.into_iter().copied().collect(),
            target => vec![]
        })
        .quantity(Quantity::Ordered(quantity)),
    )
    .remove(&target)
    .unwrap_or_default()
}
