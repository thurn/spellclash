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
use data::core::primitives::PlayerName;
use data::game_states::game_state::GameState;

use crate::core::game_view::GameButtonView;
use crate::panels::modal_panel::{DebugPanel, ModalPanel, PanelData};

pub fn render(_game: &GameState, _player: PlayerName) -> ModalPanel {
    ModalPanel {
        title: Some("Debug".to_string()),
        on_close: UserAction::ClosePanel,
        data: PanelData::Debug(DebugPanel {
            buttons: vec![GameButtonView::new_primary("Close", UserAction::ClosePanel)],
        }),
    }
}
