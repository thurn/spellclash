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
use data::card_states::iter_matching::{IterMatching, IterOptional};
use data::card_states::zones::ZoneQueries;
use data::core::card_tags::CardTag;
use data::game_states::combat_state::{
    AttackTarget, AttackerId, BlockerId, BlockerMap, CombatState,
};
use data::game_states::game_state::GameState;
use data::properties::card_property_data::{CanAttackTarget, CanBeBlocked};
use enumset::EnumSet;
use primitives::game_primitives::{CardType, HasController, PermanentId, PlayerName, Source};

use crate::predicates::card_predicates;
use crate::queries::{card_queries, player_queries};

/// Returns true if the card with the provided [AttackerId] has any valid
/// target to attack. Returns None if this attacker no longer exists.
///
/// > 508.1a. The active player chooses which creatures that they control, if
/// > any, will attack. The chosen creatures must be untapped, they can't also
/// > be battles, and each one must either have haste or have been controlled by
/// > the active player continuously since the turn began.
///
/// <https://yawgatog.com/resources/magic-rules/#R5081a>
pub fn can_attack(game: &GameState, source: Source, attacker_id: AttackerId) -> Option<bool> {
    let turn = game.turn;
    let card = game.card(attacker_id)?;
    let types = card_queries::card_types(game, source, card.id)?;
    let mut result = true;
    result &= card.last_changed_control != turn || can_attack_same_turn(game, source, attacker_id)?;
    result &= card.entered_current_zone != turn || can_attack_same_turn(game, source, attacker_id)?;
    result &= card.controller() == turn.active_player;
    result &= card.tapped_state == TappedState::Untapped;
    result &= types.contains(CardType::Creature);
    result &= !types.contains(CardType::Battle);

    attack_targets(game, source).map(|target| CanAttackTarget { attacker_id, target }).any_matching(
        |target| card.properties.can_attack_target.query_with(game, source, &target, result),
    )
}

/// Returns true if the indicated permanent has the 'haste' ability.
pub fn can_attack_same_turn(
    game: &GameState,
    source: Source,
    permanent_id: PermanentId,
) -> Option<bool> {
    game.card(permanent_id)?.properties.can_attack_same_turn.query(game, source, false)
}

/// Returns true if the indicated permanent has the 'flying' ability.
pub fn has_flying(game: &GameState, source: Source, permanent_id: PermanentId) -> Option<bool> {
    Some(
        game.card(permanent_id)?
            .properties
            .tags
            .query(game, source, EnumSet::empty())
            .contains(CardTag::Flying),
    )
}

/// Returns an iterator over all legal attackers for the provided player.
pub fn legal_attackers(
    game: &GameState,
    source: Source,
    player: PlayerName,
) -> impl Iterator<Item = AttackerId> + '_ {
    game.battlefield(player).iter_matching(game, source, can_attack)
}

/// Returns true if the card with the provided [BlockerId] can block legally in
/// the current combat phase. Must be invoked while there are confirmed
/// attackers.
///
/// > 509.1a. The defending player chooses which creatures they control, if any,
/// > will block. The chosen creatures must be untapped and they can't also be
/// > battles.
///
/// <https://yawgatog.com/resources/magic-rules/#R5091a>
#[must_use]
pub fn can_block(game: &GameState, source: Source, blocker_id: BlockerId) -> Option<bool> {
    let blocker = game.card(blocker_id)?;
    let types = card_queries::card_types(game, source, blocker.id)?;

    let mut result = true;
    result &= blocker.controller() != game.turn.active_player;
    result &= blocker.tapped_state != TappedState::Tapped;
    result &= types.contains(CardType::Creature);
    result &= !types.contains(CardType::Battle);
    let attackers = game.combat.as_ref()?.confirmed_attackers()?;

    attackers
        .all()
        .map(|(&attacker_id, &target)| CanBeBlocked { attacker_id, target, blocker_id })
        .any_matching(|target| {
            game.card(target.attacker_id)?
                .properties
                .can_be_blocked
                .query_with(game, source, &target, result)
        })
}

/// Returns an iterator over all legal blockers for the provided player.
pub fn legal_blockers(
    game: &GameState,
    source: Source,
    player: PlayerName,
) -> impl Iterator<Item = BlockerId> + '_ {
    game.battlefield(player).iter_matching(game, source, can_block)
}

/// Returns an iterator over legal targets the active player could attack during
/// combat.
pub fn attack_targets(game: &GameState, source: Source) -> impl Iterator<Item = AttackTarget> + '_ {
    player_queries::inactive_players(game).iter().flat_map(move |player| {
        iter::once(AttackTarget::Player(player)).chain(
            game.battlefield(player)
                .iter_matching(game, source, card_predicates::planeswalker)
                .map(move |id| AttackTarget::Planeswalker(player, id)),
        )
    })
}

/// Possible states for a creature to be in during combat.
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum CombatRole {
    SelectedAttacker,
    ProposedAttacker(AttackTarget),
    Attacker(AttackTarget),
    SelectedBlocker,
    ProposedBlocker(BlockerId),
    Blocking { attacker: AttackerId, order: usize },
}

/// Returns this entity's current [CombatRole], if any.
#[must_use]
pub fn role(game: &GameState, permanent_id: PermanentId) -> Option<CombatRole> {
    match &game.combat {
        None => None,
        Some(CombatState::ProposingAttackers(attackers)) => {
            if attackers.proposed_attacks.contains(permanent_id) {
                Some(CombatRole::ProposedAttacker(
                    attackers.proposed_attacks.get_target(permanent_id)?,
                ))
            } else if attackers.selected_attackers.contains(&permanent_id) {
                Some(CombatRole::SelectedAttacker)
            } else {
                None
            }
        }
        Some(CombatState::ConfirmedAttackers(attackers)) => {
            if attackers.contains(permanent_id) {
                Some(CombatRole::Attacker(attackers.get_target(permanent_id)?))
            } else {
                None
            }
        }
        Some(CombatState::ProposingBlockers(blockers)) => {
            if blockers.proposed_blocks.contains_key(&permanent_id) {
                Some(CombatRole::ProposedBlocker(blockers.proposed_blocks[&permanent_id][0]))
            } else if blockers.selected_blockers.contains(&permanent_id) {
                Some(CombatRole::SelectedBlocker)
            } else if blockers.attackers.contains(permanent_id) {
                Some(CombatRole::Attacker(blockers.attackers.get_target(permanent_id)?))
            } else {
                None
            }
        }
        Some(CombatState::OrderingBlockers(blockers)) => {
            role_in_blocker_map(permanent_id, blockers)
        }
        Some(CombatState::ConfirmedBlockers(blockers)) => {
            role_in_blocker_map(permanent_id, blockers)
        }
    }
}

fn role_in_blocker_map(id: PermanentId, blockers: &BlockerMap) -> Option<CombatRole> {
    if blockers.attackers.contains(id) {
        Some(CombatRole::Attacker(blockers.attackers.get_target(id)?))
    } else if blockers.reverse_lookup.contains_key(&id) {
        let attacker_id = blockers.reverse_lookup[&id][0];
        Some(CombatRole::Blocking {
            attacker: attacker_id,
            order: blockers.blocked_attackers[&attacker_id]
                .iter()
                .position(|&blocker_id| blocker_id == id)
                .expect("Blocker not found in blocked_attackers"),
        })
    } else {
        None
    }
}
