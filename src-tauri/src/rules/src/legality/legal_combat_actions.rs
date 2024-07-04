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
use data::core::primitives::{PlayerName, Source};
use data::delegates::delegate_arguments::CanAttackTarget;
use data::game_states::combat_state::{CombatState, ProposedAttackers};
use data::game_states::game_state::GameState;

#[allow(unused)] // Used in docs
use crate::legality::legal_actions;
use crate::legality::legal_actions::LegalActions;
use crate::queries::combat_queries;

/// True if the player is currently making combat decisions and cannot thus take
/// other game actions.
pub fn in_combat_prompt(game: &GameState, player: PlayerName) -> bool {
    matches!(
        &game.combat,
        Some(CombatState::ProposingAttackers(_))
            | Some(CombatState::ProposingBlockers(_))
            | Some(CombatState::OrderingBlockers(_))
    )
}

/// Appends all legal combat actions for the named player to the provided
/// vector.
///
/// The provided player is assumed to have already been validated by
/// [legal_actions::next_to_act], i.e. we can assume this is the attacking
/// player during 'declare attacks' and the defending player during 'declare
/// blocks'.
pub fn append(
    game: &GameState,
    player: PlayerName,
    actions: &mut Vec<GameAction>,
    options: LegalActions,
) {
    match &game.combat {
        None => {}
        Some(CombatState::ProposingAttackers(ProposedAttackers {
            proposed_attacks,
            selected_attackers,
        })) => {
            extend_actions(
                actions,
                combat_queries::legal_attackers(game, Source::Game, player)
                    .filter(|id| {
                        !selected_attackers.contains(id) && !proposed_attacks.contains(*id)
                    })
                    .map(CombatAction::AddSelectedAttacker),
            );
            if !selected_attackers.is_empty() {
                extend_actions(
                    actions,
                    combat_queries::attack_targets(game, Source::Game)
                        .filter(|target| {
                            // Only include targets that all selected attackers can legally attack.
                            game.delegates.can_attack_target.query_all(
                                game,
                                Source::Game,
                                selected_attackers.iter().map(|&attacker| CanAttackTarget {
                                    attacker_id: attacker,
                                    target: *target,
                                }),
                                true,
                            )
                        })
                        .map(CombatAction::SetSelectedAttackersTarget),
                );
            }
            if options.for_human_player {
                extend_actions(
                    actions,
                    selected_attackers
                        .iter()
                        .copied()
                        .chain(proposed_attacks.all())
                        .map(CombatAction::RemoveAttacker),
                );
            }
            actions.push(CombatAction::ConfirmAttackers.into());
        }
        Some(CombatState::ConfirmedAttackers(attackers)) => {}
        Some(CombatState::ProposingBlockers(blockers)) => {
            extend_actions(
                actions,
                combat_queries::legal_blockers(game, Source::Game, player)
                    .filter(|id| {
                        !blockers.selected_blockers.contains(id)
                            && !blockers.proposed_blocks.contains_key(id)
                    })
                    .map(CombatAction::AddSelectedBlocker),
            );
            if !blockers.selected_blockers.is_empty() {
                extend_actions(
                    actions,
                    blockers.attackers.all().map(CombatAction::SetSelectedBlockersTarget),
                );
            }
            if options.for_human_player {
                extend_actions(
                    actions,
                    blockers
                        .selected_blockers
                        .iter()
                        .chain(blockers.proposed_blocks.keys())
                        .copied()
                        .map(CombatAction::RemoveBlocker),
                );
            }
            actions.push(CombatAction::ConfirmBlockers.into());
        }
        Some(CombatState::OrderingBlockers(blockers)) => {
            // TODO: Add ordering actions
            actions.push(CombatAction::ConfirmBlockerOrder.into());
        }
        Some(CombatState::ConfirmedBlockers(blockers)) => {}
    }
}

fn extend_actions(
    actions: &mut Vec<GameAction>,
    combat_action: impl Iterator<Item = CombatAction>,
) {
    actions.extend(combat_action.map(GameAction::CombatAction));
}
