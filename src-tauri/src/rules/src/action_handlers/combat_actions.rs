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

use std::collections::HashMap;

use data::actions::debug_action::DebugGameAction;
use data::actions::game_action::CombatAction;
#[allow(unused)] // Used in docs
use data::actions::game_action::GameAction;
use data::card_states::zones::ZoneQueries;
use data::core::primitives::{CardId, CardType, PlayerName, Source};
use data::game_states::combat_state::{
    AttackTarget, AttackerId, BlockerId, BlockerMap, CombatState, ProposedAttackers,
};
use data::game_states::game_state::GameState;
use tracing::instrument;
use utils::outcome;
use utils::outcome::Outcome;

use crate::mutations::permanents;
use crate::queries::{card_queries, combat_queries, player_queries};

#[instrument(name = "combat_actions_execute", level = "debug", skip(game))]
pub fn execute(game: &mut GameState, player: PlayerName, action: CombatAction) {
    match action {
        CombatAction::AddSelectedAttacker(card_id) => {
            add_selected_attacker(game, Source::Game, card_id)
        }
        CombatAction::SetSelectedAttackersTarget(target) => {
            set_selected_attackers_target(game, Source::Game, target)
        }
        CombatAction::RemoveAttacker(card_id) => remove_attacker(game, Source::Game, card_id),
        CombatAction::ConfirmAttackers => confirm_attackers(game, Source::Game),
        CombatAction::AddSelectedBlocker(card_id) => {
            add_selected_blocker(game, Source::Game, card_id)
        }
        CombatAction::SetSelectedBlockersTarget(attacker) => {
            set_selected_blockers_target(game, Source::Game, attacker)
        }
        CombatAction::RemoveBlocker(card_id) => remove_blocker(game, Source::Game, card_id),
        CombatAction::ConfirmBlockers => confirm_blockers(game, Source::Game),
        CombatAction::OrderBlocker { attacker_id, blocker_id, position } => {
            order_blocker(game, Source::Game, attacker_id, blocker_id, position)
        }
        CombatAction::ConfirmBlockerOrder => confirm_blocker_order(game, Source::Game),
    };
}

/// Sets a creature as a selected attacker.
///
/// See [CombatAction::AddSelectedAttacker].
#[instrument(level = "debug", skip(game))]
fn add_selected_attacker(game: &mut GameState, source: Source, card_id: AttackerId) {
    let next = player_queries::next_player(game);
    // There is more than one possible attack target because a planeswalker or
    // battle is in play (or there are more than 2 players), must select attack
    // targets.
    let requires_target = combat_queries::attack_targets(game, source).nth(1).is_some();

    let Some(CombatState::ProposingAttackers(attackers)) = &mut game.combat else {
        panic!("Not in the 'ProposingAttackers' state");
    };
    if requires_target {
        attackers.selected_attackers.insert(card_id);
    } else {
        // Only one attack target, automatically assign it
        attackers.proposed_attacks.insert(card_id, AttackTarget::Player(next));
    }
}

/// Sets an attack target for selected attackers.
///
/// See [CombatAction::SetSelectedAttackersTarget].
#[instrument(level = "debug", skip(game))]
fn set_selected_attackers_target(game: &mut GameState, source: Source, target: AttackTarget) {
    let Some(CombatState::ProposingAttackers(attackers)) = &mut game.combat else {
        panic!("Not in the 'ProposingAttackers' state");
    };

    for attacker_id in attackers.selected_attackers.iter() {
        attackers.proposed_attacks.insert(*attacker_id, target);
    }
}

/// Removes an attacker proposal.
///
/// See [CombatAction::RemoveAttacker].
#[instrument(level = "debug", skip(game))]
fn remove_attacker(game: &mut GameState, source: Source, card_id: AttackerId) {
    let Some(CombatState::ProposingAttackers(attackers)) = &mut game.combat else {
        panic!("Not in the 'ProposingAttackers' state");
    };
    attackers.selected_attackers.remove(&card_id);
    attackers.proposed_attacks.remove(card_id);
}

/// Submits the attacker list.
///
/// See [CombatAction::ConfirmAttackers].
#[instrument(level = "debug", skip(game))]
fn confirm_attackers(game: &mut GameState, source: Source) {
    let Some(CombatState::ProposingAttackers(attackers)) = game.combat.take() else {
        panic!("Not in the 'ProposingAttackers' state");
    };
    for attacker in attackers.proposed_attacks.all() {
        permanents::tap(game, Source::Game, attacker);
    }
    game.combat = Some(CombatState::ConfirmedAttackers(attackers.proposed_attacks));
}

/// Sets a creature as a selected blocker.
///
/// See [CombatAction::AddSelectedBlocker].
#[instrument(level = "debug", skip(game))]
fn add_selected_blocker(game: &mut GameState, source: Source, card_id: BlockerId) {
    let Some(CombatState::ProposingBlockers(blockers)) = &mut game.combat else {
        panic!("Not in the 'ProposingBlockers' state");
    };
    if let Some(id) = blockers.attackers.exactly_one() {
        // Only one attacker, automatically block it.
        blockers.proposed_blocks.insert(card_id, vec![id]);
    } else {
        blockers.selected_blockers.insert(card_id);
    }
}

/// Sets a block target for the selected blockers.
///
/// See [CombatAction::SetSelectedBlockersTarget].
#[instrument(level = "debug", skip(game))]
fn set_selected_blockers_target(game: &mut GameState, source: Source, attacker: AttackerId) {
    let Some(CombatState::ProposingBlockers(blockers)) = &mut game.combat else {
        panic!("Not in the 'ProposingBlockers' state");
    };

    for blocker_id in blockers.selected_blockers.iter() {
        blockers.proposed_blocks.insert(*blocker_id, vec![attacker]);
    }
}

/// Removes a blocker proposal.
///
/// See [CombatAction::RemoveBlocker].
#[instrument(level = "debug", skip(game))]
fn remove_blocker(game: &mut GameState, source: Source, card_id: BlockerId) {
    let Some(CombatState::ProposingBlockers(blockers)) = &mut game.combat else {
        panic!("Not in the 'ProposingBlockers' state");
    };
    blockers.selected_blockers.remove(&card_id);
    blockers.proposed_blocks.remove(&card_id);
}

/// Submits the blocker list.
///
/// See [CombatAction::ConfirmBlockers].
#[instrument(level = "debug", skip(game))]
fn confirm_blockers(game: &mut GameState, source: Source) {
    let Some(CombatState::ProposingBlockers(blockers)) = game.combat.take() else {
        panic!("Not in the 'ProposingBlockers' state");
    };
    let mut attackers_to_blockers = HashMap::new();
    for (&blocker_id, attackers) in &blockers.proposed_blocks {
        if attackers.len() != 1 {
            todo!("Implement support for blocking multiple attackers");
        }
        // TODO: Figure out some kind of default ordering for blockers
        attackers_to_blockers.entry(attackers[0]).or_insert_with(Vec::new).push(blocker_id);
    }
    game.combat = Some(CombatState::OrderingBlockers(BlockerMap {
        attackers: blockers.attackers,
        blocked_attackers: attackers_to_blockers,
        reverse_lookup: blockers.proposed_blocks,
    }));
}

/// Sets the order of a blocker for a creature.
///
/// See [CombatAction::OrderBlocker].
#[instrument(level = "debug", skip(game))]
fn order_blocker(
    game: &mut GameState,
    source: Source,
    attacker_id: AttackerId,
    blocker_id: BlockerId,
    position: usize,
) {
    let Some(CombatState::OrderingBlockers(blockers)) = &mut game.combat else {
        panic!("Not in the 'OrderingBlockers' state");
    };
    let entry = blockers
        .blocked_attackers
        .get_mut(&attacker_id)
        .unwrap_or_else(|| panic!("Attacker not found {attacker_id:?}"));
    entry.retain(|id| id != &blocker_id);
    entry.insert(position, blocker_id);
}

/// Submits the blocker order.
///
/// See [CombatAction::ConfirmBlockerOrder].
#[instrument(level = "debug", skip(game))]
fn confirm_blocker_order(game: &mut GameState, source: Source) {
    let Some(CombatState::OrderingBlockers(blockers)) = game.combat.take() else {
        panic!("Not in the 'OrderingBlockers' state");
    };
    game.combat = Some(CombatState::ConfirmedBlockers(blockers));
}
