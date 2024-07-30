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

use tokio::sync::{mpsc, oneshot};
use crate::game_states::game_state::GameState;
use crate::prompts::prompt::{Prompt, PromptResponse};
/// Represents a change to the state of the game which should be translated
/// into a client animation
#[derive(Debug, Clone)]
pub enum GameAnimation {}

/// Represents an incremental update to the game state as the rules engine is
/// running.
pub struct GameUpdate {
    /// Snapshot of the game state at the time of the update.
    pub game: GameState,

    /// An animation to perform after displaying this game state
    /// snapshot.
    pub animation: Option<GameAnimation>,

    /// A prompt to display to a named player.
    pub prompt: Option<Prompt>,

    /// A channel on which a [PromptResponse] can be sent to make
    /// some choice for this game state.
    ///
    /// If this is None, no response is expected.
    pub response_channel: Option<oneshot::Sender<PromptResponse>>,
}

impl GameUpdate {
    pub fn new(game: &GameState) -> Self {
        Self { game: game.shallow_clone(), prompt: None, animation: None, response_channel: None }
    }

    pub fn animation(mut self, animation: GameAnimation) -> Self {
        self.animation = Some(animation);
        self
    }

    pub fn prompt(mut self, prompt: Prompt) -> Self {
        self.prompt = Some(prompt);
        self
    }

    pub fn response_channel(mut self, response_channel: oneshot::Sender<PromptResponse>) -> Self {
        self.response_channel = Some(response_channel);
        self
    }
}

/// Channel on which incremental [GameUpdate]s can be sent to the client.
pub type UpdateChannel = mpsc::UnboundedSender<GameUpdate>;
