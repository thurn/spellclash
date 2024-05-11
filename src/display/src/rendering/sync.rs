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

use data::card_states::card_state::CardState;
use data::core::primitives::{PlayerName, Zone};
use data::game_states::game_state::GameState;
use data::player_states::player_state::PlayerQueries;
use rules::legality::legal_actions;

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
        .filter(|c| !skip_sending_to_client(c))
        .map(|c| card_sync::card_view(builder, &CardViewContext::Game(c.printed(), game, c)))
        .collect::<Vec<_>>();

    builder.push_game_view(GameView {
        viewer: player_view(game, builder.player),
        opponent: player_view(game, match builder.player {
            PlayerName::One => PlayerName::Two,
            PlayerName::Two => PlayerName::One,
            _ => todo!("Not implemented"),
        }),
        cards,
        status_description: format!(
            "{:?}/Turn {}/Player {:?}",
            game.step, game.turn.turn_number, game.turn.active_player
        ),
        state: if game.combat.is_some() {
            GameViewState::CombatActive
        } else {
            GameViewState::None
        },
        can_pass_priority: legal_actions::can_pass_priority(game, builder.player),
        can_undo: game.undo_tracker.enabled && game.undo_tracker.undo.is_some(),
    });
}

fn player_view(game: &GameState, player: PlayerName) -> PlayerView {
    PlayerView { life: game.player(player).life, can_act: game.priority == player }
}

pub fn skip_sending_to_client(card: &CardState) -> bool {
    card.revealed_to.is_empty() && card.zone == Zone::Library
}
