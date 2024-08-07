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

use color_eyre::owo_colors::OwoColorize;
use data::card_definitions::ability_definition::{Ability, AbilityType};
use data::card_definitions::definitions;
use data::card_states::iter_matching::IterMatching;
use data::card_states::play_card_plan::{PlayCardChoices, PlayCardPlan, PlayCardTiming};
use data::card_states::zones::ZoneQueries;
use data::game_states::game_state::GameState;
use data::prompts::entity_choice_prompt::Choice;
use data::text_strings::Text;
use either::Either;
use primitives::game_primitives::{
    AbilityId, CardId, EntityId, HasController, PlayerName, Source, Zone,
};
use tracing::instrument;
use utils::outcome::Outcome;

use crate::core::debug_snapshot;
use crate::planner::spell_planner;
use crate::play_cards::{pick_face_to_play, play_card_executor};
use crate::prompt_handling::prompts;
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
    let mut plan = select_face(game, player, source, card_id);
    select_modes(game, player, card_id, &mut plan);
    select_targets(game, player, card_id, &mut plan);
    plan.mana_payment = spell_planner::mana_payment(game, source, card_id, &plan)
        .expect("Unable to pay mana for card");
    play_card_executor::execute_plan(game, player, card_id, source, plan)
}

fn select_face(
    game: &mut GameState,
    player: PlayerName,
    source: Source,
    card_id: CardId,
) -> PlayCardPlan {
    let mut plans = pick_face_to_play::play_as(game, player, source, card_id);
    assert!(!plans.is_empty(), "No valid plans to play card");
    assert_eq!(plans.len(), 1, "TODO: Handle playing cards with multiple faces");
    plans.remove(0)
}

fn select_modes(
    game: &mut GameState,
    player: PlayerName,
    card_id: CardId,
    plan: &mut PlayCardPlan,
) {
    let mut iterator = modal_spell_abilities(game, card_id);
    let Some((source, ability)) = iterator.next() else {
        return;
    };
    assert!(iterator.next().is_none(), "Card cannot have multiple modal abilities");
    drop(iterator);

    let mut valid_choices = vec![];
    for mode in ability.modes() {
        plan.choices.modes.clear();
        plan.choices.modes.push(mode);
        if has_valid_targets(game, source, card_id, plan) {
            valid_choices.push(mode);
        }
    }

    // TODO: Handle selecting multiple modes
    let choice = prompts::multiple_choice(game, player, Text::SelectMode, valid_choices);
    plan.choices.modes.clear();
    plan.choices.modes.push(choice);
}

fn select_targets(
    game: &mut GameState,
    player: PlayerName,
    card_id: CardId,
    plan: &mut PlayCardPlan,
) {
    let prompt_lists = targeted_spell_abilities(game, card_id)
        .map(|(s, ability)| {
            ability
                .valid_targets(game, &plan.choices, s)
                .map(|entity_id| Choice { entity_id })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();
    for choices in prompt_lists {
        assert!(!choices.is_empty(), "No valid targets available");
        let response = prompts::choose_entity(game, player, Text::SelectTarget, choices);
        plan.targets.push(response);
    }
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

    pick_face_to_play::play_as(game, player, source, card_id)
        .into_iter()
        .any(|mut plan| can_play_card_as(game, source, card_id, &mut plan))
}

/// Check whether a [PlayCardPlan] could allow a card to be played
/// when populated with a face to play & timing value.
fn can_play_card_as(
    game: &GameState,
    source: Source,
    card_id: CardId,
    plan: &mut PlayCardPlan,
) -> bool {
    match plan.choices.play_as.timing {
        PlayCardTiming::Land => true,
        _ => has_valid_modes(game, source, card_id, plan),
    }
}

/// Check whether a [PlayCardPlan] which is populated with a face to play
/// could allow a card to be played with valid modes.
fn has_valid_modes(
    game: &GameState,
    source: Source,
    card_id: CardId,
    plan: &mut PlayCardPlan,
) -> bool {
    let Some((_, ability)) = modal_spell_abilities(game, card_id).next() else {
        return has_valid_targets(game, source, card_id, plan);
    };

    for mode in ability.modes() {
        // TODO: Handle selecting multiple modes.
        plan.choices.modes.clear();
        plan.choices.modes.push(mode);
        if has_valid_targets(game, source, card_id, plan) {
            return true;
        }
    }

    false
}

/// Check whether a [PlayCardPlan] which is populated with a face to play and
/// mode selection could allow a card to be played with valid targets.
fn has_valid_targets(
    game: &GameState,
    source: Source,
    card_id: CardId,
    plan: &mut PlayCardPlan,
) -> bool {
    if targeted_spell_abilities(game, card_id).next().is_some() {
        for list in valid_target_lists(game, &plan.choices, card_id) {
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

/// Returns an iterator over spell abilities of this card which are modal
fn modal_spell_abilities(
    game: &GameState,
    card_id: CardId,
) -> impl Iterator<Item = (Source, &dyn Ability)> {
    spell_abilities_matching(game, card_id, |ability| ability.is_modal())
}

/// Returns an iterator over spell abilities of this card which require targets.
fn targeted_spell_abilities(
    game: &GameState,
    card_id: CardId,
) -> impl Iterator<Item = (Source, &dyn Ability)> {
    spell_abilities_matching(game, card_id, |ability| ability.requires_targets())
}

/// Returns an iterator over spell abilities of this card which match a given
/// predicate
fn spell_abilities_matching(
    game: &GameState,
    card_id: CardId,
    predicate: impl Fn(&dyn Ability) -> bool,
) -> impl Iterator<Item = (Source, &dyn Ability)> {
    let Some(card_name) = game.card(card_id).map(|c| c.card_name) else {
        return Either::Left(iter::empty());
    };

    Either::Right(definitions::get(card_name).iterate_abilities().filter_map(
        move |(number, ability)| {
            if ability.get_ability_type() == AbilityType::Spell && predicate(ability) {
                Some((Source::Ability(AbilityId { card_id, number }), ability))
            } else {
                None
            }
        },
    ))
}

/// Returns an iterator over valid target lists for spell abilities of this
/// card.
fn valid_target_lists<'a>(
    game: &'a GameState,
    choices: &'a PlayCardChoices,
    card_id: CardId,
) -> impl Iterator<Item = Vec<EntityId>> + 'a {
    let Some(card) = game.card(card_id) else {
        return Either::Left(iter::empty());
    };

    Either::Right(targeted_spell_abilities(game, card_id).flat_map(move |(scope, ability)| {
        ability.valid_targets(game, choices, scope).map(|entity_id| vec![entity_id])
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
