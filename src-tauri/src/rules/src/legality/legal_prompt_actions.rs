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

use data::actions::game_action::GameAction;
use data::actions::prompt_action::PromptAction;
use data::core::primitives::PlayerName;
use data::game_states::game_state::GameState;
use data::prompts::prompt::{Prompt, PromptType};

/// Returns the list of legal actions the [PlayerName] player can take in
/// response to the provided [Prompt].
pub fn compute(game: &GameState, player: PlayerName, prompt: &Prompt) -> Vec<GameAction> {
    if player != prompt.player {
        return vec![];
    }

    match &prompt.prompt_type {
        PromptType::EntityChoice(_) => {
            todo!("Implement EntityChoice")
        }
        PromptType::SelectCards(_) => {
            todo!("Implement SelectCards")
        }
        PromptType::PlayCards(_) => {
            todo!("Implement PlayCards")
        }
        PromptType::PickNumber(pick) => (pick.minimum..=pick.maximum)
            .map(|n| GameAction::PromptAction(PromptAction::PickNumber(n)))
            .collect(),
    }
}