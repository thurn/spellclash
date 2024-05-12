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

use data::core::primitives::EntityId;
use data::game_states::combat_state::{AttackTarget, BlockerMap, CombatState};
use data::game_states::game_state::GameState;

/// Possible states for a creature to be in during combat.
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum CombatRole {
    ActiveAttacker,
    ProposedAttacker(AttackTarget),
    Attacker(AttackTarget),
    ActiveBlocker,
    ProposedBlocker(EntityId),
    Blocking { attacker: EntityId, order: usize },
}

/// Returns this entity's current [CombatRole], if any.
pub fn role(game: &GameState, entity: EntityId) -> Option<CombatRole> {
    match &game.combat {
        None => None,
        Some(CombatState::ProposingAttackers { proposed_attacks, active_attackers }) => {
            if proposed_attacks.contains_key(&entity) {
                Some(CombatRole::ProposedAttacker(proposed_attacks[&entity]))
            } else if active_attackers.contains(&entity) {
                Some(CombatRole::ActiveAttacker)
            } else {
                None
            }
        }
        Some(CombatState::ConfirmedAttackers { attackers }) => {
            if attackers.contains_key(&entity) {
                Some(CombatRole::Attacker(attackers[&entity]))
            } else {
                None
            }
        }
        Some(CombatState::ProposingBlockers { attackers, active_blockers, proposed_blocks }) => {
            if proposed_blocks.contains_key(&entity) {
                Some(CombatRole::ProposedBlocker(proposed_blocks[&entity]))
            } else if active_blockers.contains(&entity) {
                Some(CombatRole::ActiveBlocker)
            } else if attackers.contains_key(&entity) {
                Some(CombatRole::Attacker(attackers[&entity]))
            } else {
                None
            }
        }
        Some(CombatState::OrderingBlockers { blockers }) => role_in_blocker_map(entity, blockers),
        Some(CombatState::ConfirmedBlockers { blockers }) => role_in_blocker_map(entity, blockers),
    }
}

fn role_in_blocker_map(entity: EntityId, blockers: &BlockerMap) -> Option<CombatRole> {
    if blockers.all_attackers.contains_key(&entity) {
        Some(CombatRole::Attacker(blockers.all_attackers[&entity]))
    } else if blockers.reverse_lookup.contains_key(&entity) {
        let attacker_id = blockers.reverse_lookup[&entity];
        Some(CombatRole::Blocking {
            attacker: attacker_id,
            order: blockers.blocked_attackers[&attacker_id]
                .iter()
                .position(|&id| id == entity)
                .expect("Blocker not found in blocked_attackers"),
        })
    } else {
        None
    }
}
