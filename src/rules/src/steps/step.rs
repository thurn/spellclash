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

use std::collections::{HashMap, HashSet};

use data::card_states::zones::ZoneQueries;
use data::core::primitives::{CardType, PlayerName, Source};
use data::game_states::combat_state::{
    AttackerMap, CombatState, ProposedAttackers, ProposedBlockers,
};
use data::game_states::game_state::GameState;
use data::game_states::game_step::GamePhaseStep;
use enumset::EnumSet;
use utils::outcome::Outcome;
use utils::{fail, outcome};

use crate::mutations::{library, permanents};
use crate::queries::{card_queries, players};

/// Advances the game state to the indicated `step`.
///
/// Primary entry point for the game state machine. Performs all action_handlers
/// for exiting the previous [GamePhaseStep] and then performs action_handlers
/// which occur at the start of this step. Increments the turn number and active
/// player when transitioning to the Untap step.
pub fn advance(game: &mut GameState) -> Outcome {
    advance_internal(game, &StepConfig::default())
}

fn advance_internal(game: &mut GameState, config: &StepConfig) -> Outcome {
    let step = enum_iterator::next(&game.step).unwrap_or(GamePhaseStep::Untap);
    match step {
        GamePhaseStep::Untap => untap(game, config),
        GamePhaseStep::Upkeep => upkeep(game, config),
        GamePhaseStep::Draw => draw(game, config),
        GamePhaseStep::PreCombatMain => pre_combat_main(game, config),
        GamePhaseStep::BeginCombat => begin_combat(game, config),
        GamePhaseStep::DeclareAttackers => declare_attackers(game, config),
        GamePhaseStep::DeclareBlockers => declare_blockers(game, config),
        GamePhaseStep::FirstStrikeDamage => first_strike_damage(game, config),
        GamePhaseStep::CombatDamage => combat_damage(game, config),
        GamePhaseStep::EndCombat => end_combat(game, config),
        GamePhaseStep::PostCombatMain => post_combat_main(game, config),
        GamePhaseStep::EndStep => end_step(game, config),
        GamePhaseStep::Cleanup => cleanup(game, config),
    }
}

#[derive(Debug, Clone)]
struct StepConfig {
    /// Steps to always skip
    skip_steps: EnumSet<GamePhaseStep>,
    /// Steps to skip until the end of this turn
    skip_this_turn: EnumSet<GamePhaseStep>,
}

impl StepConfig {
    pub fn with_skip_this_turn(&self, set: EnumSet<GamePhaseStep>) -> Self {
        let mut result = self.clone();
        result.skip_this_turn = set;
        result
    }
}

impl Default for StepConfig {
    fn default() -> Self {
        Self {
            skip_steps: GamePhaseStep::Upkeep
                | GamePhaseStep::Draw
                | GamePhaseStep::BeginCombat
                | GamePhaseStep::FirstStrikeDamage
                | GamePhaseStep::CombatDamage
                | GamePhaseStep::EndCombat,
            skip_this_turn: EnumSet::empty(),
        }
    }
}

fn begin_step(game: &mut GameState, step: GamePhaseStep) -> Outcome {
    game.step = step;
    game.priority = game.turn.active_player;
    outcome::OK
}

fn is_skipped(config: &StepConfig, step: GamePhaseStep) -> bool {
    config.skip_steps.contains(step) || config.skip_this_turn.contains(step)
}

fn advance_if_skipped(game: &mut GameState, config: &StepConfig, step: GamePhaseStep) -> Outcome {
    if is_skipped(config, step) {
        advance_internal(game, config)
    } else {
        outcome::OK
    }
}

fn untap(game: &mut GameState, config: &StepConfig) -> Outcome {
    begin_step(game, GamePhaseStep::Untap)?;
    let next = players::next_player_after(game, game.turn.active_player);
    if next == PlayerName::One {
        game.turn.turn_number += 1;
    }
    game.turn.active_player = next;

    // > 502.3. Third, the active player determines which permanents they control
    // > will untap. Then they untap them all simultaneously. This turn-based action
    // > doesn't use the stack. Normally, all of a player's permanents untap, but
    // > effects can keep one or more of a player's permanents from untapping.
    // <https://yawgatog.com/resources/magic-rules/#R5023>
    let to_untap = game.battlefield(next).clone();
    for &card_id in &to_untap {
        permanents::untap(game, Source::Game, card_id)?;
    }

    // > 502.4. No player receives priority during the untap step, so no spells can
    // > be cast or resolve and no abilities can be activated or resolve. Any
    // > ability that triggers during this step will be held until the next time a
    // > player would receive priority, which is usually during the upkeep step.
    // > (See rule 503, "Upkeep Step.")
    // <https://yawgatog.com/resources/magic-rules/#R5024>
    advance_internal(game, config)
}

fn upkeep(game: &mut GameState, config: &StepConfig) -> Outcome {
    begin_step(game, GamePhaseStep::Upkeep)?;
    advance_if_skipped(game, config, GamePhaseStep::Upkeep)
}

fn draw(game: &mut GameState, config: &StepConfig) -> Outcome {
    begin_step(game, GamePhaseStep::Draw)?;

    // > 504.1. First, the active player draws a card. This turn-based action
    // doesn't use the stack.
    // <https://yawgatog.com/resources/magic-rules/#R5041>
    library::draw(game, Source::Game, game.turn.active_player)?;

    // > 504.2. Second, the active player gets priority. (See rule 117, "Timing
    // > and Priority.")
    // <https://yawgatog.com/resources/magic-rules/#R5042>

    advance_if_skipped(game, config, GamePhaseStep::Draw)
}

fn pre_combat_main(game: &mut GameState, config: &StepConfig) -> Outcome {
    begin_step(game, GamePhaseStep::PreCombatMain)?;
    advance_if_skipped(game, config, GamePhaseStep::PreCombatMain)
}

fn begin_combat(game: &mut GameState, config: &StepConfig) -> Outcome {
    begin_step(game, GamePhaseStep::BeginCombat)?;
    let config = if !game
        .battlefield(game.turn.active_player)
        .iter()
        .any(|&card_id| card_queries::card_types(game, card_id).contains(CardType::Creature))
    {
        config.with_skip_this_turn(
            GamePhaseStep::BeginCombat
                | GamePhaseStep::DeclareAttackers
                | GamePhaseStep::DeclareBlockers
                | GamePhaseStep::FirstStrikeDamage
                | GamePhaseStep::CombatDamage
                | GamePhaseStep::EndCombat,
        )
    } else {
        config.clone()
    };
    advance_if_skipped(game, &config, GamePhaseStep::BeginCombat)
}

fn declare_attackers(game: &mut GameState, config: &StepConfig) -> Outcome {
    begin_step(game, GamePhaseStep::DeclareAttackers)?;
    if is_skipped(config, GamePhaseStep::DeclareAttackers) {
        advance_internal(game, config)
    } else {
        // > 508.1. First, the active player declares attackers. This turn-based action
        // > doesn't use the stack.
        //
        // > 508.1a. The active player chooses which creatures that they control, if
        // > any, will attack. The chosen creatures must be untapped, they can't also be
        // > battles, and each one must either have haste or have been controlled by the
        // > active player continuously since the turn began.
        // <https://yawgatog.com/resources/magic-rules/#R5081>

        game.combat = Some(CombatState::ProposingAttackers(ProposedAttackers {
            proposed_attacks: AttackerMap::default(),
            active_attackers: HashSet::new(),
        }));
        outcome::OK
    }
}

fn declare_blockers(game: &mut GameState, config: &StepConfig) -> Outcome {
    begin_step(game, GamePhaseStep::DeclareBlockers)?;
    if is_skipped(config, GamePhaseStep::DeclareBlockers) {
        advance_internal(game, config)
    } else {
        // > 509.1. First, the defending player declares blockers. This turn-based
        // > action doesn't use the stack.
        //
        // > 509.1a. The defending player chooses which creatures they control, if any, will block.
        // > The chosen creatures must be untapped and they can't also be battles. For each of the
        // > chosen creatures, the defending player chooses one creature for it to block that's
        // > attacking that player, a planeswalker they control, or a battle they protect.
        // <https://yawgatog.com/resources/magic-rules/#R5091>
        let next = players::next_player(game);
        let Some(CombatState::ConfirmedAttackers(attackers)) = game.combat.take() else {
            fail!("Not in the 'ConfirmedAttackers' state");
        };
        game.combat = Some(CombatState::ProposingBlockers(ProposedBlockers {
            defender: next,
            attackers,
            active_blockers: HashSet::new(),
            proposed_blocks: HashMap::new(),
        }));
        outcome::OK
    }
}

fn first_strike_damage(game: &mut GameState, config: &StepConfig) -> Outcome {
    begin_step(game, GamePhaseStep::FirstStrikeDamage)?;
    advance_if_skipped(game, config, GamePhaseStep::FirstStrikeDamage)
}

fn combat_damage(game: &mut GameState, config: &StepConfig) -> Outcome {
    begin_step(game, GamePhaseStep::CombatDamage)?;
    advance_if_skipped(game, config, GamePhaseStep::CombatDamage)
}

fn end_combat(game: &mut GameState, config: &StepConfig) -> Outcome {
    begin_step(game, GamePhaseStep::EndCombat)?;
    advance_if_skipped(game, config, GamePhaseStep::EndCombat)
}

fn post_combat_main(game: &mut GameState, config: &StepConfig) -> Outcome {
    begin_step(game, GamePhaseStep::PostCombatMain)?;
    game.combat = None;
    advance_if_skipped(game, config, GamePhaseStep::PostCombatMain)
}

fn end_step(game: &mut GameState, config: &StepConfig) -> Outcome {
    begin_step(game, GamePhaseStep::EndStep)?;
    advance_if_skipped(game, config, GamePhaseStep::EndStep)
}

fn cleanup(game: &mut GameState, config: &StepConfig) -> Outcome {
    begin_step(game, GamePhaseStep::Cleanup);

    // > 514.1. First, if the active player's hand contains more cards than their
    // > maximum hand size (normally seven), they discard enough cards to reduce
    // > their hand size to that number. This turn-based action doesn't use the
    // > stack.

    // > 514.2. Second, the following action_handlers happen simultaneously: all
    // > damage
    // > marked on permanents (including phased-out permanents) is removed and all
    // > "until end of turn" and "this turn" effects end. This turn-based action
    // > doesn't use the stack.

    // > 514.3. Normally, no player receives priority during the cleanup step, so no
    // > spells can be cast and no abilities can be activated. However, this rule is
    // > subject to the following exception:
    // > 514.3a. At this point, the game checks to see if any state-based
    // > action_handlers
    // > would be performed and/or any triggered abilities are waiting to be put
    // > onto the stack (including those that trigger "at the beginning of the next
    // > cleanup step"). If so, those state-based action_handlers are performed,
    // > then those
    // > triggered abilities are put on the stack, then the active player gets
    // > priority.
    // https://yawgatog.com/resources/magic-rules/#R5143
    advance_internal(game, &config.with_skip_this_turn(EnumSet::empty()))
}
