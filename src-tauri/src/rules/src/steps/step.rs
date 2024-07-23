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

use std::collections::{BTreeMap, BTreeSet};

use data::card_states::zones::ZoneQueries;
use data::core::numerics;
use data::core::numerics::Damage;
use data::core::primitives::{CardId, CardType, PlayerName, Source};
use data::game_states::combat_state::{
    AttackTarget, AttackerMap, BlockerId, CombatState, ProposedAttackers, ProposedBlockers,
};
use data::game_states::game_phase_step::GamePhaseStep;
use data::game_states::game_state::GameState;
use data::game_states::this_turn_state::ThisTurnState;
use enumset::EnumSet;
use utils::outcome;
use utils::outcome::Outcome;

use crate::mutations::{change_controller, library, permanents, players, state_based_actions};
use crate::queries::{card_queries, player_queries};

/// Advances the game state to the indicated `step`.
///
/// Primary entry point for the game state machine. Performs all actions
/// for exiting the previous [GamePhaseStep] and then performs actions
/// which occur at the start of this step. Increments the turn number and active
/// player when transitioning to the Untap step.
pub fn advance(game: &mut GameState) {
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

fn begin_step(game: &mut GameState, step: GamePhaseStep) {
    game.step = step;
    game.priority = game.turn.active_player;
    game.passed.clear();
}

fn untap(game: &mut GameState) {
    begin_step(game, GamePhaseStep::Untap);
    let next = player_queries::next_player_after(game, game.turn.active_player);
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
        permanents::untap(game, Source::Game, card_id);
    }

    // > 502.4. No player receives priority during the untap step, so no spells can
    // > be cast or resolve and no abilities can be activated or resolve. Any
    // > ability that triggers during this step will be held until the next time a
    // > player would receive priority, which is usually during the upkeep step.
    // > (See rule 503, "Upkeep Step.")
    // <https://yawgatog.com/resources/magic-rules/#R5024>
    advance(game)
}

fn upkeep(game: &mut GameState) {
    begin_step(game, GamePhaseStep::Upkeep);
}

fn draw(game: &mut GameState) {
    begin_step(game, GamePhaseStep::Draw);

    // > 504.1. First, the active player draws a card. This turn-based action
    // doesn't use the stack.
    // <https://yawgatog.com/resources/magic-rules/#R5041>
    library::draw(game, Source::Game, game.turn.active_player);
}

fn pre_combat_main(game: &mut GameState) {
    begin_step(game, GamePhaseStep::PreCombatMain);
}

fn begin_combat(game: &mut GameState) {
    begin_step(game, GamePhaseStep::BeginCombat);
}

fn declare_attackers(game: &mut GameState) {
    begin_step(game, GamePhaseStep::DeclareAttackers);
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
        selected_attackers: BTreeSet::new(),
    }));
}

fn declare_blockers(game: &mut GameState) {
    begin_step(game, GamePhaseStep::DeclareBlockers);
    // > 509.1. First, the defending player declares blockers. This turn-based
    // > action doesn't use the stack.
    //
    // > 509.1a. The defending player chooses which creatures they control, if any,
    // > will block.
    // > The chosen creatures must be untapped and they can't also be battles. For
    // > each of the
    // > chosen creatures, the defending player chooses one creature for it to block
    // > that's
    // > attacking that player, a planeswalker they control, or a battle they
    // > protect.
    // <https://yawgatog.com/resources/magic-rules/#R5091>
    let next = player_queries::next_player(game);
    let Some(CombatState::ConfirmedAttackers(attackers)) = game.combat.take() else {
        panic!("Not in the 'ConfirmedAttackers' state");
    };
    game.combat = Some(CombatState::ProposingBlockers(ProposedBlockers {
        defender: next,
        attackers,
        selected_blockers: BTreeSet::new(),
        proposed_blocks: BTreeMap::new(),
    }));
}

fn first_strike_damage(game: &mut GameState) {
    begin_step(game, GamePhaseStep::FirstStrikeDamage);
}

pub enum CombatDamageAssignment {
    Player(PlayerName, Damage),
    Planeswalker(PlayerName, Damage),
    Battle(PlayerName, Damage),
    Creature(BlockerId, Damage),
}

fn combat_damage(game: &mut GameState) {
    begin_step(game, GamePhaseStep::CombatDamage);
    let Some(CombatState::ConfirmedBlockers(blockers)) = &game.combat else {
        panic!("Not in the 'ConfirmedBlockers' state");
    };

    // > 510.1. First, the active player announces how each attacking creature
    // > assigns its combat damage, then the defending player announces how each
    // > blocking creature assigns its combat damage.

    let mut damage_assignments = vec![];
    for (attacker_id, target) in blockers.attackers.all() {
        outcome::execute(|| {
            // > 510.1a. Each attacking creature and each blocking creature assigns
            // > combat damage equal to its power. Creatures that would assign 0 or less
            // > damage this way don't assign combat damage at all.
            // <https://yawgatog.com/resources/magic-rules/#R5101>
            if blockers.blocked_attackers.contains_key(attacker_id) {
                let blockers = &blockers.blocked_attackers[attacker_id];
                if blockers.len() != 1 {
                    // TODO: Implement support for multiple blockers
                }
                let blocker_id = blockers[0];
                damage_assignments.push(CombatDamageAssignment::Creature(
                    blocker_id,
                    numerics::power_to_damage(card_queries::power(
                        game,
                        Source::Game,
                        *attacker_id,
                    )?),
                ));
            } else {
                match target {
                    AttackTarget::Player(player) => {
                        damage_assignments.push(CombatDamageAssignment::Player(
                            *player,
                            numerics::power_to_damage(card_queries::power(
                                game,
                                Source::Game,
                                *attacker_id,
                            )?),
                        ));
                    }
                    _ => todo!("Implement attack target"),
                }
            }

            outcome::OK
        });
    }

    for (blocker_id, attackers) in &blockers.reverse_lookup {
        outcome::execute(|| {
            // > 510.1d. A blocking creature assigns combat damage to the creatures it's blocking.
            // > If it isn't currently blocking any creatures (if, for example, they were destroyed
            // > or removed from combat), it assigns no combat damage. If it's blocking exactly one
            // > creature, it assigns all its combat damage to that creature.
            // <https://yawgatog.com/resources/magic-rules/#R5101d>
            if attackers.len() != 1 {
                todo!("Implement support for blocking multiple attackers");
            }
            let attacker_id = attackers[0];
            damage_assignments.push(CombatDamageAssignment::Creature(
                attacker_id,
                numerics::power_to_damage(card_queries::power(game, Source::Game, *blocker_id)?),
            ));

            outcome::OK
        });
    }

    // > 510.2. Second, all combat damage that's been assigned is dealt
    // > simultaneously. This turn-based action doesn't use the stack. No player has
    // > the chance to cast spells or activate abilities between the time combat
    // > damage is assigned and the time it's dealt.
    // <https://yawgatog.com/resources/magic-rules/#R5102>
    for assignment in damage_assignments {
        match assignment {
            CombatDamageAssignment::Player(player, damage) => {
                players::deal_damage(game, Source::Game, player, damage);
            }
            CombatDamageAssignment::Planeswalker(player, damage) => {
                todo!("Implement planeswalker damage");
            }
            CombatDamageAssignment::Battle(player, damage) => {
                todo!("Implement battle damage");
            }
            CombatDamageAssignment::Creature(creature_id, damage) => {
                permanents::deal_damage(game, Source::Game, creature_id, damage);
            }
        }
    }

    // > 510.3. Third, the active player gets priority.
    // <https://yawgatog.com/resources/magic-rules/#R5103>
}

fn end_combat(game: &mut GameState) {
    begin_step(game, GamePhaseStep::EndCombat);
}

fn post_combat_main(game: &mut GameState) {
    begin_step(game, GamePhaseStep::PostCombatMain);
    game.combat = None;
}

fn end_step(game: &mut GameState) {
    begin_step(game, GamePhaseStep::EndStep);
}

fn cleanup(game: &mut GameState) {
    begin_step(game, GamePhaseStep::Cleanup);

    // > 514.1. First, if the active player's hand contains more cards than their
    // > maximum hand size (normally seven), they discard enough cards to reduce
    // > their hand size to that number. This turn-based action doesn't use the
    // > stack.
    //
    // <https://yawgatog.com/resources/magic-rules/#R5141>

    // > 514.2. Second, the following actions happen simultaneously: all
    // > damage marked on permanents (including phased-out permanents) is removed
    // > and all "until end of turn" and "this turn" effects end. This turn-based
    // > action doesn't use the stack.
    //
    // <https://yawgatog.com/resources/magic-rules/#R5142>
    for card in game.zones.all_cards_mut() {
        card.damage = 0;
    }

    for (event_id, target_id) in game.ability_state.this_turn.remove_control_changing_effects() {
        change_controller::remove_control(game, event_id, target_id);
    }
    game.ability_state.this_turn = ThisTurnState::default();

    // > 514.3. Normally, no player receives priority during the cleanup step, so no
    // > spells can be cast and no abilities can be activated. However, this rule is
    // > subject to the following exception:
    //
    // > 514.3a. At this point, the game checks to see if any state-based action
    // > would be performed and/or any triggered abilities are waiting to be put
    // > onto the stack (including those that trigger "at the beginning of the next
    // > cleanup step"). If so, those state-based actions are performed,
    // > then those triggered abilities are put on the stack, then the active player
    // > gets priority.
    //
    // https://yawgatog.com/resources/magic-rules/#R5143
    let anything_happened = state_based_actions::on_will_receive_priority(game);
    if !anything_happened {
        advance(game)
    }
}
