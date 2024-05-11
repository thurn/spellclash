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
use data::actions::game_action::CombatAction;
#[allow(unused)] // Used in docs
use data::actions::game_action::GameAction;
use data::core::primitives::{CardId, PlayerName, Source};
use data::game_states::combat_state::{AttackTarget, AttackerId, BlockerId};
use data::game_states::game_state::GameState;
use tracing::instrument;
use utils::outcome;
use utils::outcome::Outcome;

#[instrument(err, level = "debug", skip(game))]
pub fn execute(game: &mut GameState, player: PlayerName, action: CombatAction) -> Outcome {
    match action {
        CombatAction::SetActiveAttacker(card_id) => {
            set_active_attacker(game, Source::Game, card_id)
        }
        CombatAction::AttackWithActiveAttacker(target) => {
            attack_with_active_attacker(game, Source::Game, target)
        }
        CombatAction::RemoveAttacker(card_id) => remove_attacker(game, Source::Game, card_id),
        CombatAction::ConfirmAttackers => confirm_attackers(game, Source::Game),
        CombatAction::SetActiveBlocker(card_id) => set_active_blocker(game, Source::Game, card_id),
        CombatAction::BlockWithActiveBlocker(attacker) => {
            block_with_active_blocker(game, Source::Game, attacker)
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
/// See [GameAction::SetActiveAttacker].
#[instrument(err, level = "debug", skip(game))]
fn set_active_attacker(game: &mut GameState, source: Source, card_id: BlockerId) -> Outcome {
    outcome::OK
}

/// Sets an attack target for the active attacker.
///
/// See [GameAction::AttackWithActiveAttacker].
#[instrument(err, level = "debug", skip(game))]
fn attack_with_active_attacker(
    game: &mut GameState,
    source: Source,
    target: AttackTarget,
) -> Outcome {
    outcome::OK
}

/// Removes an attacker proposal.
///
/// See [GameAction::RemoveAttacker].
#[instrument(err, level = "debug", skip(game))]
fn remove_attacker(game: &mut GameState, source: Source, card_id: AttackerId) -> Outcome {
    outcome::OK
}

/// Submits the attacker list.
///
/// See [GameAction::ConfirmAttackers].
#[instrument(err, level = "debug", skip(game))]
fn confirm_attackers(game: &mut GameState, source: Source) -> Outcome {
    outcome::OK
}

/// Sets a creature as the current blocker.
///
/// See [GameAction::SetActiveBlocker].
#[instrument(err, level = "debug", skip(game))]
fn set_active_blocker(game: &mut GameState, source: Source, card_id: BlockerId) -> Outcome {
    outcome::OK
}

/// Sets a block target for the active blocker.
///
/// See [GameAction::BlockWithActiveBlocker].
#[instrument(err, level = "debug", skip(game))]
fn block_with_active_blocker(
    game: &mut GameState,
    source: Source,
    attacker: AttackerId,
) -> Outcome {
    outcome::OK
}

/// Removes a blocker proposal.
///
/// See [GameAction::RemoveBlocker].
#[instrument(err, level = "debug", skip(game))]
fn remove_blocker(game: &mut GameState, source: Source, card_id: BlockerId) -> Outcome {
    outcome::OK
}

/// Submits the blocker list.
///
/// See [GameAction::ConfirmBlockers].
#[instrument(err, level = "debug", skip(game))]
fn confirm_blockers(game: &mut GameState, source: Source) -> Outcome {
    outcome::OK
}

/// Sets the order of a blocker for a creature.
///
/// See [GameAction::OrderBlocker].
#[instrument(err, level = "debug", skip(game))]
fn order_blocker(
    game: &mut GameState,
    source: Source,
    attacker_id: AttackerId,
    blocker_id: BlockerId,
    position: usize,
) -> Outcome {
    outcome::OK
}

/// Submits the blocker order.
///
/// See [GameAction::ConfirmBlockerOrder].
#[instrument(err, level = "debug", skip(game))]
fn confirm_blocker_order(game: &mut GameState, source: Source) -> Outcome {
    outcome::OK
}
