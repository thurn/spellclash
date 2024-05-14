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

use data::card_states::card_state::TappedState;
use data::card_states::zones::ZoneQueries;
use data::core::primitives::{CardId, CardType, EntityId};
use data::game_states::combat_state::{AttackTarget, BlockerMap, CombatState, ProposedAttackers};
use data::game_states::game_state::GameState;

use crate::queries::{card_queries, players};

/// Returns true if the card with the provided [CardId] can attack in the
/// current combat phase.
///
/// > 508.1a. The active player chooses which creatures that they control, if
/// > any, will attack. The chosen creatures must be untapped, they can't also
/// > be battles, and each one must either have haste or have been controlled by
/// > the active player continuously since the turn began.
/// <https://yawgatog.com/resources/magic-rules/#R5081a>
pub fn can_attack(game: &GameState, card_id: CardId) -> bool {
    let turn = game.turn;
    let card = game.card(card_id);
    let types = card_queries::card_types(game, card_id);
    card.last_changed_control != turn
        && card.entered_current_zone != turn
        && card.controller == turn.active_player
        && card.tapped_state != TappedState::Tapped
        && types.contains(CardType::Creature)
        && !types.contains(CardType::Battle)
}

/// Returns true if the card with the provided [CardId] can block in the current
/// combat phase.
///
/// > 509.1a. The defending player chooses which creatures they control, if any,
/// > will block. The chosen creatures must be untapped and they can't also be
/// > battles.
/// <https://yawgatog.com/resources/magic-rules/#R5091a>
pub fn can_block(game: &GameState, card_id: CardId) -> bool {
    let card = game.card(card_id);
    let types = card_queries::card_types(game, card_id);
    card.controller != game.turn.active_player
        && card.tapped_state != TappedState::Tapped
        && types.contains(CardType::Creature)
        && !types.contains(CardType::Battle)
}

/// Returns an iterator over legal targets the active player could attack during
/// combat.
pub fn attack_targets(game: &GameState) -> impl Iterator<Item = AttackTarget> + '_ {
    players::inactive_players(game)
        .iter()
        .flat_map(|player| {
            iter::once(AttackTarget::Player(player)).chain(
                game.battlefield(player)
                    .iter()
                    .filter(|&&card_id| {
                        card_queries::card_types(game, card_id).contains(CardType::Planeswalker)
                    })
                    .map(|&card_id| AttackTarget::Planeswalker(game.card(card_id).entity_id)),
            )
        })
        .chain(
            game.battlefield(game.turn.active_player)
                .iter()
                .filter(|&&card_id| {
                    card_queries::card_types(game, card_id).contains(CardType::Battle)
                })
                .map(|&card_id| AttackTarget::Battle(game.card(card_id).entity_id)),
        )
}

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
        Some(CombatState::ProposingAttackers(attackers)) => {
            if attackers.proposed_attacks.contains(entity) {
                Some(CombatRole::ProposedAttacker(attackers.proposed_attacks.get_target(entity)?))
            } else if attackers.active_attackers.contains(&entity) {
                Some(CombatRole::ActiveAttacker)
            } else {
                None
            }
        }
        Some(CombatState::ConfirmedAttackers(attackers)) => {
            if attackers.contains(entity) {
                Some(CombatRole::Attacker(attackers.get_target(entity)?))
            } else {
                None
            }
        }
        Some(CombatState::ProposingBlockers(blockers)) => {
            if blockers.proposed_blocks.contains_key(&entity) {
                Some(CombatRole::ProposedBlocker(blockers.proposed_blocks[&entity]))
            } else if blockers.active_blockers.contains(&entity) {
                Some(CombatRole::ActiveBlocker)
            } else if blockers.attackers.contains(entity) {
                Some(CombatRole::Attacker(blockers.attackers.get_target(entity)?))
            } else {
                None
            }
        }
        Some(CombatState::OrderingBlockers(blockers)) => role_in_blocker_map(entity, blockers),
        Some(CombatState::ConfirmedBlockers(blockers)) => role_in_blocker_map(entity, blockers),
    }
}

fn role_in_blocker_map(entity: EntityId, blockers: &BlockerMap) -> Option<CombatRole> {
    if blockers.all_attackers.contains(entity) {
        Some(CombatRole::Attacker(blockers.all_attackers.get_target(entity)?))
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
