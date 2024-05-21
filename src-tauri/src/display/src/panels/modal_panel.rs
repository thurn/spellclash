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
use serde::{Deserialize, Serialize};
use specta::Type;

use crate::core::game_view::GameButtonView;

/// Rendering options for a modal window which can be displayed on top of other
/// game content
#[derive(Clone, Debug, Serialize, Deserialize, Type)]
pub struct ModalPanel {
    pub title: Option<String>,
    pub on_close: UserAction,
    pub data: PanelData,
}

/// Types of content which can appear in a modal panel
#[derive(Clone, Debug, Serialize, Deserialize, Type)]
pub enum PanelData {
    Debug(DebugPanel),
}

/// Debug options
#[derive(Clone, Debug, Serialize, Deserialize, Type)]
pub struct DebugPanel {
    pub buttons: Vec<GameButtonView>,
}
