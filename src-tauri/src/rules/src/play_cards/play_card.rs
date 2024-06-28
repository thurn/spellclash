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

use data::card_definitions::ability_definition::{Ability, AbilityType};
use data::card_definitions::definitions;
use data::card_states::iter_matching::IterMatching;
use data::card_states::play_card_plan::{PlayAs, PlayCardPlan, PlayCardTiming};
use data::card_states::zones::ZoneQueries;
use data::core::primitives::{
    AbilityId, CardId, EntityId, HasController, PlayerName, Source, Zone,
};
use data::delegates::has_delegates::HasDelegates;
use data::delegates::scope::Scope;
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
        .map(|(scope, ability)| {
            ability
                .valid_targets(game, scope)
                .map(|entity_id| Choice { entity_id })
                .collect::<Vec<_>>()
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
) -> impl Iterator<Item = (Scope, &dyn Ability)> {
    let Some(card_name) = game.card(card_id).map(|c| c.card_name) else {
        return Either::Left(iter::empty());
    };

    Either::Right(definitions::get(card_name).iterate_abilities().filter_map(
        move |(number, ability)| {
            if ability.get_ability_type() == AbilityType::Spell && ability.requires_targets() {
                Some((game.create_scope(AbilityId { card_id, number })?, ability))
            } else {
                None
            }
        },
    ))
}

fn valid_target_lists(
    game: &GameState,
    card_id: CardId,
) -> impl Iterator<Item = Vec<EntityId>> + '_ {
    let Some(card) = game.card(card_id) else {
        return Either::Left(iter::empty());
    };

    Either::Right(targeted_abilities(game, card_id).flat_map(|(scope, ability)| {
        ability.valid_targets(game, scope).map(|entity_id| vec![entity_id])
    }))
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
