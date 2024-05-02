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

use crate::delegates::ability_context::AbilityContext;
use crate::game_states::game_state::GameState;
use crate::prompts::prompt::Prompt;

pub type PromptFn = fn(&GameState, AbilityContext) -> Prompt;

/// Stores [Prompt]s to show to players to make game choices.
///
/// Prompts are generated via callback function immediately before display, in
/// order to ensure they use the most current information when shown.
///
/// Prompts cannot be serialized, so game serialization only happens when no
/// prompts are active.
#[derive(Default, Clone, Debug)]
pub struct PromptStack {
    /// Prompt currently being displayed to a player
    pub current: Option<Prompt>,

    /// Stack of functions to produce future prompts to show.
    pub stack: Vec<PromptFn>,
}

impl PromptStack {
    /// Adds a new prompt generator function to the stack
    pub fn push(&mut self, function: PromptFn) {
        self.stack.push(function);
    }
}
