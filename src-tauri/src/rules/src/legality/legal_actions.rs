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

use std::iter;

use data::actions::game_action::GameAction;
use data::card_states::zones::ZoneQueries;
use data::core::primitives::{CardId, PlayerName, Source};
use data::game_states::combat_state::{CombatState, CombatStateKind};
use data::game_states::game_state::{GameState, GameStatus};
use data::printed_cards::printed_card::Face;
use tracing::instrument;

use crate::legality::{can_pay_mana_cost, legal_combat_actions, legal_prompt_actions};
use crate::play_cards::{pick_face_to_play, play_card};

/// List of all legal actions the named player can take in the
/// current game state.
#[instrument(name = "legal_actions_compute", level = "trace", skip(game))]
pub fn compute(game: &GameState, player: PlayerName) -> Vec<GameAction> {
    let mut result = vec![];
    if game.status != GameStatus::Playing {
        return result;
    }

    if let Some(p) = &game.prompts.current_prompt {
        return legal_prompt_actions::compute(game, player, p);
    }

    if next_to_act(game) != player {
        return result;
    }

    if can_pass_priority(game, player) {
        result.push(GameAction::PassPriority);
    }

    for &card_id in game.hand(player) {
        if play_card::can_play_card(game, player, Source::Game, card_id) {
            result.push(GameAction::ProposePlayingCard(card_id));
        }
    }

    legal_combat_actions::append(game, player, &mut result);
    result
}

/// Returns true if the [PlayerName] player can currently legally take the
/// provided [GameAction].
#[instrument(level = "trace", skip(game, game_action))]
pub fn can_take_action(game: &GameState, player: PlayerName, game_action: &GameAction) -> bool {
    compute(game, player).iter().any(|action| action == game_action)
}

/// Returns the name of the player who is currently allowed to take an action.
///
/// If the game has not yet started, this will be player one. If the game
/// has ended, this will be the player who held priority at the end of the
/// game.
pub fn next_to_act(game: &GameState) -> PlayerName {
    if let Some(p) = &game.prompts.current_prompt {
        return p.player;
    }

    match game.combat.as_ref() {
        Some(CombatState::ProposingAttackers(_)) => game.turn.active_player,
        Some(CombatState::ConfirmedAttackers(_)) => game.priority,
        Some(CombatState::ProposingBlockers(blockers)) => blockers.defender,
        Some(CombatState::OrderingBlockers(_)) => game.turn.active_player,
        Some(CombatState::ConfirmedBlockers(_)) => game.priority,
        None => game.priority,
    }
}

/// Returns true if any player can currently take the action to pass
/// priority.
pub fn can_any_player_pass_priority(game: &GameState) -> bool {
    match game.combat.as_ref().map(|c| c.kind()) {
        Some(CombatStateKind::ProposingAttackers)
        | Some(CombatStateKind::ProposingBlockers)
        | Some(CombatStateKind::OrderingBlockers) => return false,
        _ => {}
    }
    true
}

/// Returns true if the named player can currently take the action to pass
/// priority.
pub fn can_pass_priority(game: &GameState, player: PlayerName) -> bool {
    can_any_player_pass_priority(game) && game.priority == player
}
