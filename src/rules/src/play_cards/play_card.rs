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

use data::card_states::play_card_plan::PlayCardPlan;
use data::card_states::zones::ZoneQueries;
use data::core::primitives::{CardId, PlayerName, Source, Zone};
use data::game_states::game_state::GameState;
use enum_iterator::Sequence;
use utils::outcome::{Outcome, Value};
use utils::{fail, outcome};

use crate::play_cards::play_card_choices::{PlayCardChoice, PlayCardChoicePrompt};
use crate::play_cards::{play_card_choices, play_card_executor};

/// Plays a card.
///
/// This will prompt the player for all required choices to play the card, and
/// then put it into play. An error is returned if the player makes a choice
/// which results in this card being illegal to play (e.g. selecting a target
/// which increases the cost of a spell beyond their ability to play).
pub fn execute(
    game: &mut GameState,
    player: PlayerName,
    card_id: CardId,
    source: Source,
) -> Outcome {
    let plan = prompt_for_play_card_plan(game, card_id, source)?;
    play_card_executor::execute_plan(game, player, card_id, source, plan)
}

/// Returns true if the [PlayerName] player can currently legally play the
/// [CardId] card.
///
/// A player can play a card if they control that card and it is in their hand
/// (or if some other ability is allowing them to play it) and if they can make
/// a legal choice for each of the required choices which are part of playing
/// this card (targeting, paying mana).
pub fn can_play_card(
    game: &GameState,
    player: PlayerName,
    card_id: CardId,
    source: Source,
) -> bool {
    let card = game.card(card_id);
    if card.controller != player || card.zone != Zone::Hand {
        return false;
    }

    can_play_card_in_step(game, card_id, source, &PlayCardPlan::default(), PlayCardStep::ChooseFace)
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, Ord, PartialOrd, Sequence)]
pub enum PlayCardStep {
    /// Pick a face of the card to play, e.g. for adventures, split cards, mdfcs
    ChooseFace,

    /// Check whether timing restrictions & land play rules allow this card to
    /// be played.
    ///
    /// Selecting a face that is a land will end the 'play card' process here.
    CheckLegalityForTypes,

    /// Choose an optional alternate cost to pay
    SelectAlternateCost,

    /// Choose modes for a modal card
    SelectModes,

    /// Choose any optional additional costs to pay
    SelectAdditionalCosts,

    /// Choose any number of cards in hand to splice with this card
    Splice,

    /// Pick legal targets for the card. Having zero available targets may or
    /// may not be an error depending on the text.
    PickTargets,

    /// Pick the value of an "X" variable in the cost of a spell, if it's not
    /// determined by the rules text or target selection.
    SelectVariable,

    /// Activate mana abilities to pay for this card.
    PayMana,

    /// Pick options to pay any non-mana costs for this card.
    PayNonManaCosts,
}

/// Show a series of prompts to the player trying to play a card in order to
/// construct a valid [PlayCardPlan] to play this card.
///
/// Returns an error if the user makes choices which result in no legal way to
/// play this card.
fn prompt_for_play_card_plan(
    game: &GameState,
    card_id: CardId,
    source: Source,
) -> Value<PlayCardPlan> {
    let mut plan = PlayCardPlan::default();
    for step in enum_iterator::all::<PlayCardStep>() {
        loop {
            let choice = play_card_choices::choice_for_step(game, card_id, &plan, step);
            match choice {
                PlayCardChoice::None => {
                    break;
                }
                PlayCardChoice::Invalid => {
                    fail!("Cannot legally play {card_id:?} in step {step:?}");
                }
                PlayCardChoice::Prompt { optional, prompt } => {
                    show_prompt_and_add_to_plan(
                        game, card_id, source, optional, prompt, &mut plan,
                    )?;
                }
            }
        }
    }

    Ok(plan)
}

/// Show the player a [PlayCardChoicePrompt] and record the choice made in the
/// provided [PlayCardPlan].
fn show_prompt_and_add_to_plan(
    game: &GameState,
    card_id: CardId,
    source: Source,
    optional: bool,
    prompt: PlayCardChoicePrompt,
    plan: &mut PlayCardPlan,
) -> Outcome {
    outcome::OK
}

/// Recursively performs a depth-first search of all possible [PlayCardPlan]s
/// based on a provided partially-constructed [PlayCardPlan] to see if any of
/// them result in a legal set of choices which would allow the provided card to
/// be played.
fn can_play_card_in_step(
    game: &GameState,
    card_id: CardId,
    source: Source,
    plan: &PlayCardPlan,
    step: PlayCardStep,
) -> bool {
    let choice = play_card_choices::choice_for_step(game, card_id, plan, step);

    match choice {
        PlayCardChoice::None => {
            // Advance to next step or return true if we are at the end
            return step
                .next()
                .map_or(true, |next| can_play_card_in_step(game, card_id, source, plan, next));
        }
        PlayCardChoice::Invalid => {
            // A choice is required in this step, but no legal option is available.
            return false;
        }
        PlayCardChoice::Prompt { optional, prompt } => {
            if optional {
                // If the choice is optional and we can play this card without making a choice,
                // try skipping it.
                let advance = step
                    .next()
                    .map_or(true, |next| can_play_card_in_step(game, card_id, source, plan, next));
                if advance {
                    return true;
                }
            }

            for plan in legal_plans_for_prompt(game, card_id, source, plan, prompt) {
                if can_play_card_in_step(game, card_id, source, &plan, step) {
                    return true;
                }
            }
        }
    }

    // No legal plan exists
    false
}

/// Returns an iterator over possible extensions of the provided [PlayCardPlan]
/// for a [PlayCardChoicePrompt].
///
/// This will return one [PlayCardPlan] per option available in the prompt.
fn legal_plans_for_prompt(
    game: &GameState,
    card_id: CardId,
    source: Source,
    current: &PlayCardPlan,
    prompt: PlayCardChoicePrompt,
) -> impl Iterator<Item = PlayCardPlan> {
    iter::empty()
}
