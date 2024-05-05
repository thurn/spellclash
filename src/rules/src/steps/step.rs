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

use data::core::primitives::PlayerName;
use data::game_states::game_state::GameState;
use data::game_states::game_step::GamePhaseStep;
use utils::outcome;
use utils::outcome::Outcome;

use crate::queries::players;

/// Advances the game state to the indicated `step`.
///
/// Primary entry point for the game state machine. Performs all actions for
/// exiting the previous [GamePhaseStep] and then performs actions which occur
/// at the start of this step. Increments the turn number and active player when
/// transitioning to the Untap step.
pub fn advance(game: &mut GameState) -> Outcome {
    let step = enum_iterator::next(&game.step).unwrap_or(GamePhaseStep::Untap);
    match step {
        GamePhaseStep::Untap => untap(game),
        GamePhaseStep::Upkeep => upkeep(game),
        GamePhaseStep::Draw => draw(game),
        GamePhaseStep::PreCombatMain => pre_combat_main(game),
        GamePhaseStep::BeginCombat => begin_combat(game),
        GamePhaseStep::DeclareAttackers => declare_attackers(game),
        GamePhaseStep::DeclareBlockers => declare_blockers(game),
        GamePhaseStep::FirstStrikeDamage => first_strike_damage(game),
        GamePhaseStep::CombatDamage => combat_damage(game),
        GamePhaseStep::EndCombat => end_combat(game),
        GamePhaseStep::PostCombatMain => post_combat_main(game),
        GamePhaseStep::EndStep => end_step(game),
        GamePhaseStep::Cleanup => cleanup(game),
    }
}

fn begin_step(game: &mut GameState, step: GamePhaseStep) -> Outcome {
    game.clear_passed();
    game.step = step;
    game.priority = game.current_turn.active_player;
    outcome::OK
}

fn untap(game: &mut GameState) -> Outcome {
    begin_step(game, GamePhaseStep::Untap)?;
    let next = players::next_player_after(game, game.current_turn.active_player);
    if next == PlayerName::One {
        game.current_turn.turn_number += 1;
    }
    game.current_turn.active_player = next;

    // > No player receives priority during the untap step, so no spells can be cast
    // > or
    // > resolve and no abilities can be activated or resolve. Any ability that
    // > triggers
    // > during this step will be held until the next time a player would receive
    // > priority,
    // > which is usually during the upkeep step. (See rule 503, "Upkeep Step.")
    // <https://yawgatog.com/resources/magic-rules/#R5024>
    advance(game)
}

fn upkeep(game: &mut GameState) -> Outcome {
    begin_step(game, GamePhaseStep::Upkeep)
}

fn draw(game: &mut GameState) -> Outcome {
    begin_step(game, GamePhaseStep::Draw)
}

fn pre_combat_main(game: &mut GameState) -> Outcome {
    begin_step(game, GamePhaseStep::PreCombatMain)
}

fn begin_combat(game: &mut GameState) -> Outcome {
    begin_step(game, GamePhaseStep::BeginCombat)
}

fn declare_attackers(game: &mut GameState) -> Outcome {
    begin_step(game, GamePhaseStep::DeclareAttackers)
}

fn declare_blockers(game: &mut GameState) -> Outcome {
    begin_step(game, GamePhaseStep::DeclareBlockers)
}

fn first_strike_damage(game: &mut GameState) -> Outcome {
    begin_step(game, GamePhaseStep::FirstStrikeDamage)
}

fn combat_damage(game: &mut GameState) -> Outcome {
    begin_step(game, GamePhaseStep::CombatDamage)
}

fn end_combat(game: &mut GameState) -> Outcome {
    begin_step(game, GamePhaseStep::EndCombat)
}

fn post_combat_main(game: &mut GameState) -> Outcome {
    begin_step(game, GamePhaseStep::PostCombatMain)
}

fn end_step(game: &mut GameState) -> Outcome {
    begin_step(game, GamePhaseStep::EndStep)
}

fn cleanup(game: &mut GameState) -> Outcome {
    begin_step(game, GamePhaseStep::Cleanup)
}
