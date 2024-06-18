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
use data::core::primitives::{CardId, CardType, EntityId, PlayerName};
use data::delegates::game_delegates::AttackerData;
use data::game_states::combat_state::{AttackTarget, BlockerMap, CombatState, ProposedAttackers};
use data::game_states::game_state::GameState;

use crate::queries::{card_queries, combat_queries, player_queries};

/// Returns true if the card with the provided [CardId] has any valid target to
/// attack.
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
    let result = card.last_changed_control != turn
        && card.entered_current_zone != turn
        && card.controller == turn.active_player
        && card.tapped_state == TappedState::Untapped
        && types.contains(CardType::Creature)
        && !types.contains(CardType::Battle);

    if game.delegates.can_attack.is_empty() {
        // Computing attack targets is expensive (this was a 25% regression to the
        // random playout benchmark), so only do it if we have to.
        result
    } else {
        attack_targets(game).any(|target| {
            game.delegates.can_attack.query(game, &AttackerData { card_id, target }, result)
        })
    }
}

/// Returns an iterator over all legal attackers for the provided player.
pub fn legal_attackers(game: &GameState, player: PlayerName) -> impl Iterator<Item = CardId> + '_ {
    game.battlefield(player).iter().filter(|&&card_id| can_attack(game, card_id)).copied()
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

/// Returns an iterator over all legal blockers for the provided player.
pub fn legal_blockers(game: &GameState, player: PlayerName) -> impl Iterator<Item = CardId> + '_ {
    game.battlefield(player).iter().filter(|&&card_id| can_block(game, card_id)).copied()
}

/// Returns an iterator over legal targets the active player could attack during
/// combat.
pub fn attack_targets(game: &GameState) -> impl Iterator<Item = AttackTarget> + '_ {
    player_queries::inactive_players(game)
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
            game.battlefield(game.active_player())
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
    SelectedAttacker,
    ProposedAttacker(AttackTarget),
    Attacker(AttackTarget),
    SelectedBlocker,
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
            } else if attackers.selected_attackers.contains(&entity) {
                Some(CombatRole::SelectedAttacker)
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
                Some(CombatRole::ProposedBlocker(blockers.proposed_blocks[&entity][0]))
            } else if blockers.selected_blockers.contains(&entity) {
                Some(CombatRole::SelectedBlocker)
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
    if blockers.attackers.contains(entity) {
        Some(CombatRole::Attacker(blockers.attackers.get_target(entity)?))
    } else if blockers.reverse_lookup.contains_key(&entity) {
        let attacker_id = blockers.reverse_lookup[&entity][0];
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
