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

use data::actions::game_action::{CombatAction, GameAction};
use data::card_states::zones::ZoneQueries;
use data::core::primitives::PlayerName;
use data::game_states::combat_state::CombatState;
use data::game_states::game_state::GameState;

#[allow(unused)] // Used in docs
use crate::legality::legal_actions;
use crate::queries::combat_queries;

/// Appends all legal combat actions for the named player to the provided
/// vector.
///
/// The provided player is assumed to have already been validated by
/// [legal_actions::next_to_act], i.e. we can assume this is the attacking
/// player during 'declare attacks' and the defending player during 'declare
/// blocks'.
pub fn append(game: &GameState, player: PlayerName, actions: &mut Vec<GameAction>) {
    match &game.combat {
        None => {}
        Some(CombatState::ProposingAttackers { proposed_attacks, active_attackers }) => {
            extend_actions(
                actions,
                game.battlefield(player)
                    .iter()
                    .filter(|&&card_id| combat_queries::can_attack(game, card_id))
                    .map(|&card_id| CombatAction::AddActiveAttacker(game.card(card_id).entity_id)),
            );
            if !active_attackers.is_empty() {
                extend_actions(
                    actions,
                    combat_queries::attack_targets(game)
                        .map(CombatAction::SetActiveAttackersTarget),
                );
            }
            extend_actions(
                actions,
                active_attackers
                    .iter()
                    .map(|&attacker_id| CombatAction::RemoveAttacker(attacker_id)),
            );
            actions.push(CombatAction::ConfirmAttackers.into());
        }
        Some(CombatState::ConfirmedAttackers { attackers }) => {}
        Some(CombatState::ProposingBlockers {
            attackers,
            active_blockers,
            proposed_blocks,
            ..
        }) => {
            extend_actions(
                actions,
                game.battlefield(player)
                    .iter()
                    .filter(|&&card_id| combat_queries::can_block(game, card_id))
                    .map(|&card_id| CombatAction::AddActiveBlocker(game.card(card_id).entity_id)),
            );
            if !active_blockers.is_empty() {
                extend_actions(
                    actions,
                    attackers
                        .keys()
                        .map(|&attacker_id| CombatAction::SetActiveBlockersTarget(attacker_id)),
                );
            }
            extend_actions(
                actions,
                active_blockers.iter().map(|&blocker_id| CombatAction::RemoveBlocker(blocker_id)),
            );
            actions.push(CombatAction::ConfirmBlockers.into());
        }
        Some(CombatState::OrderingBlockers { blockers }) => {
            // TODO: Add ordering actions
            actions.push(CombatAction::ConfirmBlockerOrder.into());
        }
        Some(CombatState::ConfirmedBlockers { blockers }) => {}
    }
}

fn extend_actions(
    actions: &mut Vec<GameAction>,
    combat_action: impl Iterator<Item = CombatAction>,
) {
    actions.extend(combat_action.map(GameAction::CombatAction));
}
