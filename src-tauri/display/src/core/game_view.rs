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

use data::actions::user_action::UserAction;
use data::core::numerics::LifeValue;

use crate::core::card_view::CardView;

/// Represents the visual state of an ongoing game
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct GameView {
    /// Player who is operating the client
    pub viewer: PlayerView,

    /// Opponent of viewer
    pub opponent: PlayerView,

    /// Visual state of cards in the game
    pub cards: Vec<CardView>,

    /// Describes the status of the game, e.g. which phase & step the game is in
    pub status_description: String,

    /// High level visual game state
    pub state: GameViewState,

    /// Top user interaction options
    pub top_buttons: Vec<GameButton>,

    /// Bottom user interaction options
    pub bottom_buttons: Vec<GameButton>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct GameButton {
    pub label: String,
    pub action: UserAction,
}

impl GameButton {
    pub fn new(label: impl Into<String>, action: impl Into<UserAction>) -> Self {
        Self { label: label.into(), action: action.into() }
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Copy)]
pub enum GameViewState {
    None,

    /// There is an ongoing combat phase
    CombatActive,
}

/// Identifies a player in the context of the user interface.
#[derive(Clone, Debug, Eq, PartialEq, Copy, Hash)]
pub enum DisplayPlayer {
    /// Player who is currently operating the client
    Viewer,

    /// Opponent of viewer
    Opponent,
}

/// Represents the visual state of a player in a game
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PlayerView {
    /// Current life total
    pub life: LifeValue,

    /// Can this player currently take a game action?
    pub can_act: bool,
}
