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

use serde::Deserialize;
use slotmap::__impl::Serialize;
use primitives::game_primitives::{GameId, UserId};
/// Holds state for a user.
///
/// A 'user' is an operator of this software outside of the context of any game.
/// A 'player' is a participate within a game who may or may not be a user.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserState {
    /// ID for this user
    pub id: UserId,
    /// Current game activity of this user
    pub activity: UserActivity,
}

/// Represents the current game activity a user is participating in
#[derive(Debug, Copy, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub enum UserActivity {
    /// No current activity.
    Menu,

    /// Playing in the identified game
    Playing(GameId),
}
