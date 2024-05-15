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

use crate::game_states::game_state::GameState;

/// Represents a change to the state of the game which should be translated
/// into a client animation
#[derive(Debug, Clone)]
pub enum GameAnimation {}

/// A step in the animation process
#[derive(Debug, Clone)]
pub struct AnimationStep {
    pub snapshot: GameState,
    pub update: GameAnimation,
}

/// Standard enum used by APIs to configure their update tracking behavior.
#[derive(Debug, Clone, Eq, PartialEq)]
pub enum AnimationState {
    /// Game updates should not be tracked by the receiver
    Ignore,
    /// Game updates should be tracked by the receiver
    Track,
}

/// Tracks game mutations for a game action.
///
/// Some game state changes in the game require custom animations in the UI in
/// order to communicate their effects clearly. In order to implement the
/// animation system, code which mutates game state can also call
/// [GameState::add_animation] and provide a [GameAnimation] to record the
/// action they took. The way this process works is that a snapshot of the game
/// state is stored (to capture any mutations that occurred *before* the
/// animation), and then the update is stored. During the animation process, the
/// stored snapshots and [GameAnimation]s are played back sequentially.
///
/// Many types of state changes are handled automatically by the game state
/// snapshot system, so appending an update is only needed for custom
/// animations. For example the system will correctly detect and animate a card
/// which has moved to a new position.
#[derive(Debug, Clone)]
pub struct AnimationTracker {
    /// Used to globally disable or enable update tracking
    pub state: AnimationState,
    /// List of update steps with snapshots of the game state
    pub steps: Vec<AnimationStep>,
}

impl Default for AnimationTracker {
    fn default() -> Self {
        Self { state: AnimationState::Ignore, steps: vec![] }
    }
}

impl AnimationTracker {
    pub fn new(updates: AnimationState) -> Self {
        Self { state: updates, steps: vec![] }
    }
}
