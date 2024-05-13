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
    AttackTarget, AttackerId, BlockerId, BlockerMap, CombatState,
};
use data::game_states::game_state::GameState;
use tracing::instrument;
use utils::outcome::Outcome;
use utils::with_error::WithError;
use utils::{fail, outcome, verify};

use crate::queries::{card_queries, players};

#[instrument(err, level = "debug", skip(game))]
pub fn execute(game: &mut GameState, player: PlayerName, action: CombatAction) -> Outcome {
    verify!(game.priority == player, "Player {player:?} does not have priority");
    match action {
        CombatAction::AddActiveAttacker(card_id) => {
            add_active_attacker(game, Source::Game, card_id)
        }
        CombatAction::SetActiveAttackersTarget(target) => {
            set_active_attackers_target(game, Source::Game, target)
        }
        CombatAction::RemoveAttacker(card_id) => remove_attacker(game, Source::Game, card_id),
        CombatAction::ConfirmAttackers => confirm_attackers(game, Source::Game),
        CombatAction::AddActiveBlocker(card_id) => add_active_blocker(game, Source::Game, card_id),
        CombatAction::SetActiveBlockersTarget(attacker) => {
            set_active_blockers_target(game, Source::Game, attacker)
        }
        CombatAction::RemoveBlocker(card_id) => remove_blocker(game, Source::Game, card_id),
        CombatAction::ConfirmBlockers => confirm_blockers(game, Source::Game),
        CombatAction::OrderBlocker { attacker_id, blocker_id, position } => {
            order_blocker(game, Source::Game, attacker_id, blocker_id, position)
        }
        CombatAction::ConfirmBlockerOrder => confirm_blocker_order(game, Source::Game),
    }
}

/// Sets a creature as the current attacker.
///
/// See [CombatAction::SetActiveAttacker].
#[instrument(err, level = "debug", skip(game))]
fn add_active_attacker(game: &mut GameState, source: Source, card_id: AttackerId) -> Outcome {
    let next = players::next_player(game);
    let requires_target = players::player_count(game) > 2
        || game.battlefield(next).iter().any(|&card_id| {
            card_queries::card_types(game, card_id).contains(CardType::Planeswalker)
        })
        || game
            .battlefield(game.turn.active_player)
            .iter()
            .any(|&card_id| card_queries::card_types(game, card_id).contains(CardType::Battle));

    let Some(CombatState::ProposingAttackers { active_attackers, proposed_attacks }) =
        &mut game.combat
    else {
        fail!("Not in the 'ProposingAttackers' state");
    };
    if requires_target {
        // A planeswalker or battle is in play (or there are more than 2 players), must
        // select attack targets.
        active_attackers.insert(card_id);
    } else {
        // Only one attack target, automatically assign it
        proposed_attacks.insert(card_id, AttackTarget::Player(next));
    }
    outcome::OK
}

/// Sets an attack target for the active attacker.
///
/// See [CombatAction::SetActiveAttackersTarget].
#[instrument(err, level = "debug", skip(game))]
fn set_active_attackers_target(
    game: &mut GameState,
    source: Source,
    target: AttackTarget,
) -> Outcome {
    let Some(CombatState::ProposingAttackers { active_attackers, proposed_attacks }) =
        &mut game.combat
    else {
        fail!("Not in the 'ProposingAttackers' state");
    };

    for attacker_id in active_attackers.iter() {
        proposed_attacks.insert(*attacker_id, target);
    }
    outcome::OK
}

/// Removes an attacker proposal.
///
/// See [CombatAction::RemoveAttacker].
#[instrument(err, level = "debug", skip(game))]
fn remove_attacker(game: &mut GameState, source: Source, card_id: AttackerId) -> Outcome {
    let Some(CombatState::ProposingAttackers { active_attackers, proposed_attacks }) =
        &mut game.combat
    else {
        fail!("Not in the 'ProposingAttackers' state");
    };
    active_attackers.remove(&card_id);
    proposed_attacks.remove(&card_id);
    outcome::OK
}

/// Submits the attacker list.
///
/// See [CombatAction::ConfirmAttackers].
#[instrument(err, level = "debug", skip(game))]
fn confirm_attackers(game: &mut GameState, source: Source) -> Outcome {
    let Some(CombatState::ProposingAttackers { proposed_attacks, .. }) = game.combat.take() else {
        fail!("Not in the 'ProposingAttackers' state");
    };
    game.combat = Some(CombatState::ConfirmedAttackers { attackers: proposed_attacks.clone() });
    outcome::OK
}

/// Sets a creature as the current blocker.
///
/// See [CombatAction::SetActiveBlocker].
#[instrument(err, level = "debug", skip(game))]
fn add_active_blocker(game: &mut GameState, source: Source, card_id: BlockerId) -> Outcome {
    let Some(CombatState::ProposingBlockers { attackers, active_blockers, proposed_blocks }) =
        &mut game.combat
    else {
        fail!("Not in the 'ProposingBlockers' state");
    };
    if attackers.len() == 1 {
        // Only one attacker, automatically block it.
        proposed_blocks.insert(card_id, *attackers.keys().next().unwrap());
    } else {
        active_blockers.insert(card_id);
    }
    outcome::OK
}

/// Sets a block target for the active blocker.
///
/// See [CombatAction::SetActiveBlockersTarget].
#[instrument(err, level = "debug", skip(game))]
fn set_active_blockers_target(
    game: &mut GameState,
    source: Source,
    attacker: AttackerId,
) -> Outcome {
    let Some(CombatState::ProposingBlockers { active_blockers, proposed_blocks, .. }) =
        &mut game.combat
    else {
        fail!("Not in the 'ProposingBlockers' state");
    };

    for blocker_id in active_blockers.iter() {
        proposed_blocks.insert(*blocker_id, attacker);
    }
    outcome::OK
}

/// Removes a blocker proposal.
///
/// See [CombatAction::RemoveBlocker].
#[instrument(err, level = "debug", skip(game))]
fn remove_blocker(game: &mut GameState, source: Source, card_id: BlockerId) -> Outcome {
    let Some(CombatState::ProposingBlockers { active_blockers, proposed_blocks, .. }) =
        &mut game.combat
    else {
        fail!("Not in the 'ProposingBlockers' state");
    };
    active_blockers.remove(&card_id);
    proposed_blocks.remove(&card_id);
    outcome::OK
}

/// Submits the blocker list.
///
/// See [CombatAction::ConfirmBlockers].
#[instrument(err, level = "debug", skip(game))]
fn confirm_blockers(game: &mut GameState, source: Source) -> Outcome {
    let Some(CombatState::ProposingBlockers { attackers, proposed_blocks, .. }) =
        game.combat.take()
    else {
        fail!("Not in the 'ProposingBlockers' state");
    };
    let mut attackers_to_blockers = HashMap::new();
    for (&blocker_id, &attacker_id) in &proposed_blocks {
        // TODO: Figure out some kind of default ordering for blockers
        attackers_to_blockers.entry(attacker_id).or_insert_with(Vec::new).push(blocker_id);
    }
    game.combat = Some(CombatState::OrderingBlockers {
        blockers: BlockerMap {
            all_attackers: attackers,
            blocked_attackers: attackers_to_blockers,
            reverse_lookup: proposed_blocks,
        },
    });
    outcome::OK
}

/// Sets the order of a blocker for a creature.
///
/// See [CombatAction::OrderBlocker].
#[instrument(err, level = "debug", skip(game))]
fn order_blocker(
    game: &mut GameState,
    source: Source,
    attacker_id: AttackerId,
    blocker_id: BlockerId,
    position: usize,
) -> Outcome {
    let Some(CombatState::OrderingBlockers { blockers }) = &mut game.combat else {
        fail!("Not in the 'OrderingBlockers' state");
    };
    let entry = blockers
        .blocked_attackers
        .get_mut(&attacker_id)
        .with_error(|| "Attacker not found {attacker_id:?}")?;
    entry.retain(|id| id != &blocker_id);
    entry.insert(position, blocker_id);
    outcome::OK
}

/// Submits the blocker order.
///
/// See [CombatAction::ConfirmBlockerOrder].
#[instrument(err, level = "debug", skip(game))]
fn confirm_blocker_order(game: &mut GameState, source: Source) -> Outcome {
    let Some(CombatState::OrderingBlockers { blockers }) = game.combat.take() else {
        fail!("Not in the 'OrderingBlockers' state");
    };
    game.combat = Some(CombatState::ConfirmedBlockers { blockers });
    outcome::OK
}
