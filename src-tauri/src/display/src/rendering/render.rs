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

use data::game_states::game_state::{GameState, GameStatus};
use primitives::game_primitives::PlayerName;

use crate::commands::command::{Command, DisplayGameMessageCommand};
use crate::core::display_state::DisplayState;
use crate::core::game_message::GameMessage;
use crate::core::response_builder::{AllowActions, ResponseBuilder, ResponseState};
use crate::rendering::sync;

/// Returns a series of [Command]s which fully describe the current state of the
/// provided game
pub fn connect(game: &GameState, player: PlayerName, display_state: &DisplayState) -> Vec<Command> {
    let mut builder = ResponseBuilder::new(player, ResponseState {
        animate: false,
        is_final_update: true,
        display_state,
        reveal_all_cards: game.configuration.debug.reveal_all_cards,
        act_as_player: game.configuration.debug.act_as_player,
        allow_actions: AllowActions::Yes,
    });
    sync::run(&mut builder, game);

    if let GameStatus::GameOver { winners } = game.status {
        builder.commands.push(Command::DisplayGameMessage(DisplayGameMessageCommand {
            message: if winners.contains(builder.display_as_player()) {
                GameMessage::Victory
            } else {
                GameMessage::Defeat
            },
        }))
    }
    builder.commands
}

/// Returns a series of commands which contain animations for recent changes to
/// game states, followed by a snapshot of the current game state in the same
/// manner as returned by [connect].
pub fn render_updates(
    game: &GameState,
    player: PlayerName,
    display_state: &DisplayState,
    allow_actions: AllowActions,
) -> Vec<Command> {
    let mut builder = ResponseBuilder::new(player, ResponseState {
        animate: true,
        is_final_update: false,
        display_state,
        reveal_all_cards: game.configuration.debug.reveal_all_cards,
        act_as_player: game.configuration.debug.act_as_player,
        allow_actions,
    });

    builder.response_state.is_final_update = true;
    sync::run(&mut builder, game);

    if let GameStatus::GameOver { winners } = game.status {
        builder.commands.push(Command::DisplayGameMessage(DisplayGameMessageCommand {
            message: if winners.contains(builder.display_as_player()) {
                GameMessage::Victory
            } else {
                GameMessage::Defeat
            },
        }))
    }

    builder.commands
}
