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

use data::core::primitives::PlayerName;
use data::game_states::game_state::GameState;

use crate::core::game_view::{GameView, GameViewState, PlayerView};
use crate::core::response_builder::ResponseBuilder;
use crate::rendering::card_sync;
use crate::rendering::card_view_context::CardViewContext;

/// Converts a [GameState] into a series of commands inside the provided
/// [ResponseBuilder] describing the visual game state.
pub fn run(builder: &mut ResponseBuilder, game: &GameState) {
    let cards = game
        .zones
        .all_cards()
        .map(|c| card_sync::card_view(builder, &CardViewContext::Game(c.printed(), game, c)))
        .collect::<Vec<_>>();

    builder.push_game_view(GameView {
        viewer: player_view(game, builder.player),
        opponent: player_view(game, match builder.player {
            PlayerName::One => PlayerName::Two,
            PlayerName::Two => PlayerName::One,
        }),
        cards,
        state: if game.combat.currently_active {
            GameViewState::CombatActive
        } else {
            GameViewState::None
        },
    });
}

fn player_view(game: &GameState, player: PlayerName) -> PlayerView {
    PlayerView { life: game.players.get(player).life, can_act: game.priority == player }
}
