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
    AbilityTarget, AbilityTargetPermanent, AbilityTargetPlayer, AbilityTargetPredicate,
    AbilityTargetQuantity, PlayerSet,
};
use data::card_definitions::ability_definition::AbilityType;
use data::card_definitions::definitions;
use data::card_states::iter_matching::IterMatching;
use data::card_states::play_card_plan::{PlayAs, PlayCardPlan, PlayCardTiming};
use data::card_states::zones::ZoneQueries;
use data::core::primitives::{
    AbilityId, CardId, EntityId, HasController, PlayerName, Source, Zone,
};
use data::delegates::has_delegates::HasDelegates;
use data::delegates::scope::DelegateScope;
use data::game_states::game_state::GameState;
use data::prompts::choice_prompt::Choice;
use data::text_strings::Text;
use either::Either;
use enum_iterator::Sequence;
use enumset::EnumSet;
use tracing::instrument;
use utils::bools::FilterSome;
use utils::outcome::Outcome;

use crate::planner::spell_planner;
use crate::play_cards::{pick_face_to_play, play_card_executor};
use crate::prompt_handling::prompts;
use crate::queries::player_queries;

/// Plays a card.
///
/// This will prompt the player for all required choices to play the card, and
/// then put it into play.
pub fn execute(
    game: &mut GameState,
    player: PlayerName,
    source: Source,
    card_id: CardId,
) -> Outcome {
    let mut plans = pick_face_to_play::play_as(game, source, card_id);
    assert_eq!(plans.len(), 1, "TODO: handle multiple faces");
    let mut plan = plans.remove(0);

    let prompt_lists = targeted_abilities(game, card_id)
        .filter_map(|(ability_id, target)| {
            let scope = game.create_delegate_scope(ability_id)?;
            Some(
                valid_targets(game, scope, &target.predicate)
                    .map(|entity_id| Choice { entity_id })
                    .collect::<Vec<_>>(),
            )
        })
        .collect::<Vec<_>>();
    for choices in prompt_lists {
        assert!(!choices.is_empty(), "No valid targets available");
        let response = prompts::choose_entity(game, player, Text::SelectTarget, choices);
        plan.targets.push(response);
    }

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
    let Some(card) = game.card(card_id) else {
        return false;
    };

    if card.controller() != player || card.zone != Zone::Hand {
        return false;
    }

    pick_face_to_play::play_as(game, source, card_id)
        .into_iter()
        .any(|mut plan| can_play_card_as(game, source, card_id, &mut plan))
}

/// Check whether a [PlayCardPlan] could allow a card to be played
/// when populated with a face to play & timing value.
pub fn can_play_card_as(
    game: &GameState,
    source: Source,
    card_id: CardId,
    plan: &mut PlayCardPlan,
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
    plan: &mut PlayCardPlan,
) -> bool {
    if targeted_abilities(game, card_id).next().is_some() {
        for list in valid_target_lists(game, card_id) {
            plan.targets = list;
            if can_pay_mana_costs(game, source, card_id, plan) {
                return true;
            }
        }
        false
    } else {
        can_pay_mana_costs(game, source, card_id, plan)
    }
}

/// Returns an iterator over spell abilities of this card which require targets.
fn targeted_abilities(
    game: &GameState,
    card_id: CardId,
) -> impl Iterator<Item = (AbilityId, &AbilityTarget)> {
    let Some(card_name) = game.card(card_id).map(|c| c.card_name) else {
        return Either::Left(iter::empty());
    };

    Either::Right(
        definitions::get(card_name)
            .iterate_abilities()
            .filter(|(_, ability)| ability.ability_type == AbilityType::Spell)
            .flat_map(move |(number, ability)| {
                ability
                    .choices
                    .targets
                    .iter()
                    .map(move |target| (AbilityId { card_id, number }, target))
            }),
    )
}

fn valid_target_lists(
    game: &GameState,
    card_id: CardId,
) -> impl Iterator<Item = Vec<EntityId>> + '_ {
    let Some(card) = game.card(card_id) else {
        return Either::Left(iter::empty());
    };

    Either::Right(
        definitions::get(card.card_name)
            .iterate_abilities()
            .filter(|(_, ability)| ability.ability_type == AbilityType::Spell)
            .flat_map(move |(number, ability)| {
                ability.choices.targets.iter().flat_map(move |target| {
                    let scope =
                        game.create_delegate_scope(AbilityId { card_id, number }).unwrap_or_else(
                            || panic!("Unable to create delegate scope for card {card_id:?}"),
                        );
                    assert_eq!(
                        target.quantity,
                        AbilityTargetQuantity::Exactly(1),
                        "TODO: handle multiple target quantities"
                    );
                    valid_targets(game, scope, &target.predicate).map(|entity_id| vec![entity_id])
                })
            }),
    )
}

fn valid_targets<'a>(
    game: &'a GameState,
    scope: DelegateScope,
    target: &'a AbilityTargetPredicate,
) -> Box<dyn Iterator<Item = EntityId> + 'a> {
    match target {
        AbilityTargetPredicate::Permanent(data) => valid_card_targets(game, scope, data),
        AbilityTargetPredicate::Player(data) => valid_player_targets(game, data),
        AbilityTargetPredicate::CardOrPlayer(data) => Box::new(
            valid_card_targets(game, scope, &data.target_permanent)
                .chain(valid_player_targets(game, &data.target_player)),
        ),
        AbilityTargetPredicate::StackAbility(predicate) => Box::new(
            game.zones
                .abilities_on_stack()
                .filter_some(move |&ability_id| predicate(game, ability_id))
                .map(EntityId::StackAbility),
        ),
        AbilityTargetPredicate::AnyOf(predicate_list) => Box::new(
            predicate_list.iter().flat_map(move |predicate| valid_targets(game, scope, predicate)),
        ),
    }
}

fn valid_card_targets<'a>(
    game: &'a GameState,
    scope: DelegateScope,
    target: &'a AbilityTargetPermanent,
) -> Box<dyn Iterator<Item = EntityId> + 'a> {
    Box::new(players_in_set(game, scope, target.players).iter().flat_map(move |player| {
        game.battlefield(player).iter().filter_map(move |&permanent_id| {
            if (target.predicate)(game, permanent_id) == Some(true) {
                Some(permanent_id.to_entity_id())
            } else {
                None
            }
        })
    }))
}

fn valid_player_targets<'a>(
    game: &'a GameState,
    target: &'a AbilityTargetPlayer,
) -> Box<dyn Iterator<Item = EntityId> + 'a> {
    Box::new(
        target
            .players
            .iter()
            .filter(move |&player| (target.predicate)(game, player))
            .map(|player| player.entity_id()),
    )
}

fn players_in_set(game: &GameState, scope: DelegateScope, set: PlayerSet) -> EnumSet<PlayerName> {
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
    plan: &PlayCardPlan,
) -> bool {
    let mana_payment_plan = spell_planner::mana_payment(game, source, card_id, plan);
    mana_payment_plan.is_some()
}
