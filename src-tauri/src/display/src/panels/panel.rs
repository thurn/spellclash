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

use data::core::panel_address::{GamePanelAddress, UserPanelAddress};
use data::game_states::game_state::GameState;
use primitives::game_primitives::PlayerName;

use crate::panels::debug_panel;
use crate::panels::modal_panel::ModalPanel;

pub fn build_game_panel(
    game: &GameState,
    player: PlayerName,
    address: GamePanelAddress,
) -> ModalPanel {
    match address {
        GamePanelAddress::GameDebugPanel => debug_panel::render(game, player),
    }
}

pub fn build_user_panel(
    _game: &GameState,
    _player: PlayerName,
    address: UserPanelAddress,
) -> ModalPanel {
    match address {}
}
