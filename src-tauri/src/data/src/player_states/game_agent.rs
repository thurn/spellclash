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

use std::fmt::Debug;
use std::time::Instant;

use dyn_clone::DynClone;

use crate::actions::game_action::GameAction;
use crate::actions::prompt_action::PromptAction;
use crate::game_states::game_state::GameState;
use crate::prompts::prompt::Prompt;

/// Trait representing an AI agent playing in a game.
///
/// This is very similar to the normal 'Agent' trait, but we separate it out to
/// avoid crate circular dependency problems and add a little bit of
/// game-specific context.
pub trait GameAgent: Debug + DynClone + Send {
    fn pick_action(&self, deadline: Instant, game: &GameState) -> GameAction;

    fn pick_prompt_action(
        &self,
        deadline: Instant,
        game: &GameState,
        prompt: &Prompt,
    ) -> PromptAction;
}

dyn_clone::clone_trait_object!(GameAgent);
