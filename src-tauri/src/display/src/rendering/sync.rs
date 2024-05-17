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

use data::actions::debug_action::DebugGameAction;
use data::actions::game_action::{CombatAction, GameAction};
use data::actions::user_action::UserAction;
use data::card_states::card_state::CardState;
use data::card_states::zones::ZoneQueries;
use data::core::primitives::{PlayerName, Zone};
use data::game_states::combat_state::CombatState;
use data::game_states::game_state::GameState;
use data::game_states::game_step::GamePhaseStep;
use data::player_states::player_state::PlayerQueries;
use rules::legality::legal_actions;

use crate::core::game_view::{GameButtonView, GameView, GameViewState, PlayerView};
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
        viewer: player_view(game, builder.display_as_player()),
        opponent: player_view(game, match builder.display_as_player() {
            PlayerName::One => PlayerName::Two,
            PlayerName::Two => PlayerName::One,
            _ => todo!("Not implemented"),
        }),
        cards,
        status_description: format!(
            "{:?}\nTurn {}\nPlayer {:?}",
            game.step, game.turn.turn_number, game.turn.active_player
        ),
        state: if game.combat.is_some() {
            GameViewState::CombatActive
        } else {
            GameViewState::None
        },
        top_buttons: top_game_buttons(game, builder.act_as_player(game)),
        bottom_buttons: bottom_game_buttons(game, builder.act_as_player(game)),
    });
}

fn player_view(game: &GameState, player: PlayerName) -> PlayerView {
    PlayerView {
        life: game.player(player).life,
        can_act: legal_actions::next_to_act(game) == player,
    }
}

fn skip_sending_to_client(card: &CardState) -> bool {
    card.revealed_to.is_empty() && card.zone == Zone::Library
}

fn top_game_buttons(game: &GameState, _player: PlayerName) -> Vec<GameButtonView> {
    let mut result = vec![GameButtonView::new_default("Leave Game", UserAction::LeaveGameAction)];
    if game.undo_tracker.enabled && !game.undo_tracker.undo.is_empty() {
        result.push(GameButtonView::new_default("Undo", DebugGameAction::Undo));
    }
    result
}

fn bottom_game_buttons(game: &GameState, player: PlayerName) -> Vec<GameButtonView> {
    let mut result = vec![];
    if legal_actions::can_take_action(game, player, GameAction::PassPriority) {
        if game.stack().is_empty() {
            let next = match game.step {
                GamePhaseStep::Upkeep => "To Draw",
                GamePhaseStep::Draw => "To Main",
                GamePhaseStep::PreCombatMain => "To Combat",
                GamePhaseStep::BeginCombat => "To Attackers",
                GamePhaseStep::DeclareAttackers => "To Blockers",
                GamePhaseStep::DeclareBlockers => "To Damage",
                GamePhaseStep::FirstStrikeDamage => "To Damage",
                GamePhaseStep::CombatDamage => "End Combat",
                GamePhaseStep::EndCombat => "End Combat",
                GamePhaseStep::PostCombatMain => "End Turn",
                GamePhaseStep::EndStep => "Next Turn",
                _ => "Continue",
            };
            result.push(GameButtonView::new_primary(next, GameAction::PassPriority));
        } else {
            result.push(GameButtonView::new_primary("Resolve", GameAction::PassPriority));
        }
    }
    if legal_actions::can_take_action(game, player, CombatAction::ConfirmAttackers) {
        if let Some(CombatState::ProposingAttackers(attackers)) = &game.combat {
            let count = attackers.proposed_attacks.len();
            result.push(GameButtonView::new_primary(
                format!("{} Attacker{}", count, if count == 1 { "" } else { "s" }),
                CombatAction::ConfirmAttackers,
            ));
        }
    }
    if legal_actions::can_take_action(game, player, CombatAction::ConfirmBlockers) {
        if let Some(CombatState::ProposingBlockers(blockers)) = &game.combat {
            let count = blockers.proposed_blocks.len();
            result.push(GameButtonView::new_primary(
                format!("{} Blocker{}", count, if count == 1 { "" } else { "s" }),
                CombatAction::ConfirmBlockers,
            ));
        }
    }
    if legal_actions::can_take_action(game, player, CombatAction::ConfirmBlockerOrder) {
        result
            .push(GameButtonView::new_primary("Confirm Order", CombatAction::ConfirmBlockerOrder));
    }

    result
}
