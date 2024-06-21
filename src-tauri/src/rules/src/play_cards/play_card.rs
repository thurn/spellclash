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

use data::card_definitions::ability_choices::{
    AbilityTarget, AbilityTargetPredicate, AbilityTargetQuantity, CardAbilityTarget,
    PlayerAbilityTarget, PlayerSet,
};
use data::card_definitions::ability_definition::AbilityType;
use data::card_definitions::definitions;
use data::card_states::play_card_plan::{PlayAs, PlayCardPlan, PlayCardTiming};
use data::card_states::zones::ZoneQueries;
use data::core::primitives::{AbilityId, CardId, PlayerName, Source, Zone};
use data::delegates::has_delegates::HasDelegates;
use data::delegates::scope::Scope;
use data::game_states::game_state::GameState;
use enum_iterator::Sequence;
use enumset::EnumSet;
use tracing::instrument;
use utils::outcome::Outcome;

use crate::planner::spell_planner;
use crate::play_cards::{pick_face_to_play, play_card_executor};
use crate::queries::player_queries;

/// Plays a card.
///
/// This will prompt the player for all required choices to play the card, and
/// then put it into play. An error is returned if the player makes a choice
/// which results in this card being illegal to play (e.g. selecting a target
/// which increases the cost of a spell beyond their ability to play).
pub fn execute(
    game: &mut GameState,
    player: PlayerName,
    source: Source,
    card_id: CardId,
) -> Outcome {
    let mut plans = pick_face_to_play::play_as(game, source, card_id);
    assert_eq!(plans.len(), 1, "TODO: handle multiple faces");
    let mut plan = plans.remove(0);
    plan.mana_payment = spell_planner::mana_payment(game, source, card_id, &plan)
        .expect("Unable to pay mana for card");
    play_card_executor::execute_plan(game, player, card_id, source, plan)
}

/// Returns true if the [PlayerName] player can currently legally play the
/// [CardId] card.
///
/// A player can play a card if they control that card and it is in their hand
/// (or if some other ability is allowing them to play it) and if they can make
/// a legal choice for each of the required choices which are part of playing
/// this card (targeting, paying mana).
#[instrument(level = "trace", skip_all)]
pub fn can_play_card(
    game: &GameState,
    player: PlayerName,
    source: Source,
    card_id: CardId,
) -> bool {
    let card = game.card(card_id);
    if card.controller != player || card.zone != Zone::Hand {
        return false;
    }

    pick_face_to_play::play_as(game, source, card_id)
        .into_iter()
        .any(|plan| can_play_card_as(game, source, card_id, plan))
}

/// Check whether a [PlayCardPlan] could allow a card to be played
/// when populated with a face to play & timing value.
pub fn can_play_card_as(
    game: &GameState,
    source: Source,
    card_id: CardId,
    plan: PlayCardPlan,
) -> bool {
    match plan.play_as.timing {
        PlayCardTiming::Land => true,
        _ => can_pick_targets(game, source, card_id, plan),
    }
}

/// Check whether a [PlayCardPlan] could allow a card to be played
/// with valid targets.
pub fn can_pick_targets(
    game: &GameState,
    source: Source,
    card_id: CardId,
    plan: PlayCardPlan,
) -> bool {
    for (number, ability) in definitions::get(game.card(card_id).card_name).iterate_abilities() {
        if ability.ability_type == AbilityType::Spell {
            for target in &ability.choices.targets {
                let scope = game.create_scope(AbilityId { card_id, number });
                assert_eq!(
                    target.quantity,
                    AbilityTargetQuantity::Exactly(1),
                    "TODO: handle multiple targets"
                );
                if !valid_target_exists(game, scope, &target.predicate) {
                    return false;
                }
            }
        }
    }

    can_pay_mana_costs(game, source, card_id, plan)
}

fn valid_target_exists(game: &GameState, scope: Scope, target: &AbilityTargetPredicate) -> bool {
    match target {
        AbilityTargetPredicate::Card(data) => valid_card_target_exists(game, scope, data),
        AbilityTargetPredicate::Player(data) => valid_player_target_exists(game, scope, data),
        AbilityTargetPredicate::CardOrPlayer(data) => {
            valid_card_target_exists(game, scope, &data.card_target)
                || valid_player_target_exists(game, scope, &data.player_target)
        }
        AbilityTargetPredicate::StackAbility(predicate) => {
            game.zones.abilities_on_stack().any(|id| predicate(game, scope, id))
        }
        AbilityTargetPredicate::AnyOf(predicate_list) => {
            predicate_list.iter().any(|predicate| valid_target_exists(game, scope, predicate))
        }
    }
}

fn valid_card_target_exists(game: &GameState, scope: Scope, target: &CardAbilityTarget) -> bool {
    for zone in target.zones.iter() {
        for player in players_in_set(game, scope, target.players) {
            for card_id in game.zones.cards_in_zone(zone, player) {
                if (target.predicate)(game, scope, card_id) {
                    return true;
                }
            }
        }
    }

    false
}

fn valid_player_target_exists(
    game: &GameState,
    scope: Scope,
    target: &PlayerAbilityTarget,
) -> bool {
    for player in target.players.iter() {
        if (target.predicate)(game, scope, player) {
            return true;
        }
    }

    false
}

fn players_in_set(game: &GameState, scope: Scope, set: PlayerSet) -> EnumSet<PlayerName> {
    match set {
        PlayerSet::AllPlayers => player_queries::all_players(game),
        PlayerSet::You => EnumSet::only(scope.controller),
        PlayerSet::Opponents => player_queries::all_opponents(game, scope.controller),
    }
}

fn can_pay_mana_costs(
    game: &GameState,
    source: Source,
    card_id: CardId,
    plan: PlayCardPlan,
) -> bool {
    let mana_payment_plan = spell_planner::mana_payment(game, source, card_id, &plan);
    mana_payment_plan.is_some()
}
