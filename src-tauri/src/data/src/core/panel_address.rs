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

use serde::{Deserialize, Serialize};

use crate::core::primitives::PlayerName;

/// Identifier for a piece of UI which can be contextually displayed.
///
/// Panels are divided up into types based on which data needs to be pulled from
/// the database in order to render this panel.
#[derive(Copy, Clone, Eq, PartialEq, Debug, Serialize, Deserialize)]
pub enum PanelAddress {
    GamePanel(GamePanelAddress),
    UserPanel(UserPanelAddress),
}

#[derive(Copy, Clone, Eq, PartialEq, Debug, Serialize, Deserialize)]
pub enum GamePanelAddress {
    GameDebugPanel,
}

impl From<GamePanelAddress> for PanelAddress {
    fn from(address: GamePanelAddress) -> Self {
        PanelAddress::GamePanel(address)
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Debug, Serialize, Deserialize)]
pub enum UserPanelAddress {}

impl From<UserPanelAddress> for PanelAddress {
    fn from(address: UserPanelAddress) -> Self {
        PanelAddress::UserPanel(address)
    }
}
