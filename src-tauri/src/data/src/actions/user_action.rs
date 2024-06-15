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
use specta::{DataType, EnumType, Generics, Type, TypeMap};

use crate::actions::game_action::GameAction;
use crate::actions::new_game_action::NewGameAction;
use crate::actions::prompt_action::PromptAction;
use crate::core::panel_address::PanelAddress;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum UserAction {
    NewGameAction(NewGameAction),
    GameAction(GameAction),
    PromptAction(PromptAction),
    LeaveGameAction,
    QuitGameAction,
    OpenPanel(PanelAddress),
    ClosePanel,
    PanelTransition(PanelTransition),
}

#[derive(Serialize, Deserialize, Type)]
pub struct UserActionType {
    pub opaque_type_marker: (),
}

impl Type for UserAction {
    fn inline(type_map: &mut TypeMap, generics: Generics) -> DataType {
        DataType::Unknown
    }
}

/// Transition between panel states.
///
/// Closes the currently displayed game panel, if any, then optionally performs
/// a [GameAction] and/or opens a new panel.
#[derive(Clone, Copy, Eq, PartialEq, Debug, Serialize, Deserialize)]
pub struct PanelTransition {
    /// Optionally, a [GameAction] to perform when performing this transition.
    pub action: Option<GameAction>,

    /// Optionally, a new panel to open when performing this transition.
    pub open: Option<PanelAddress>,
}
