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

use data::card_states::play_card_plan::{ManaPaymentPlan, ModalChoice, PlayCardPlan};
use data::core::primitives::{CardId, EntityId, Source};
use data::delegates::scope::AbilityId;
use data::game_states::game_state::GameState;
use data::printed_cards::printed_card::Face;
use enumset::EnumSet;

use crate::planner::spell_planner;
use crate::play_cards::pick_face_to_play;
use crate::play_cards::play_card::PlayCardStep;

/// A choice a player can make while playing a card
#[derive(Debug, Clone)]
pub enum PlayCardChoicePrompt {
    /// Pick a face of a card to play
    ///
    /// This prompt may be displayed more than once to pick multiple faces, e.g.
    /// in the case of cards with the Fuse ability
    SelectFace { valid_faces: Vec<Face> },

    /// Select one mode for a card out of a set of options.
    ///
    /// Cards which require multiple modes will result in this prompt being
    /// displayed more than once.
    Mode { valid_modes: EnumSet<ModalChoice> },

    /// Pick an alternate cost for playing this card
    AlternateCost { valid_costs: Vec<AbilityId> },

    /// Pick an *optional* additional costs for playing a card (e.g. for
    /// mechanics like Kicker) for a card, produced by the given ability.
    ///
    /// This does not include *required* additional costs like "as an additional
    /// cost to cast this spell, sacrifice an artifact".
    OptionalCost { ability: AbilityId },

    /// Pick a card in hand to splice with this card as it is being played
    Splice { cards: Vec<CardId> },

    /// Pick one target entity for a card from a set of legal targets.
    ///
    /// Target legality is determined by game rules, but specifically not by the
    /// ability of the player to pay costs or make other choices in future steps
    /// of the 'play card' process as a result of selecting this target.
    ///
    /// Cards which require multiple targets will display this prompt more than
    /// once.
    Target { valid_targets: Vec<EntityId> },

    /// Pick the value for an 'X' variable for a card between a minimum and
    /// maximum value (inclusive).
    SelectVariable { minimum: usize, maximum: usize },

    /// Select mana abilities to use to pay costs for this spell.
    ///
    /// Unlike the other choices here, this generates a single proposed
    /// [ManaPaymentPlan] which will allow the card to be played, and can be
    /// viewed as a boolean 'yes/no' choice about whether to use the proposed
    /// plan.
    ///
    /// If no payment plan can be found, casting the card is illegal.
    PayMana { mana_payment_plan: ManaPaymentPlan },

    /// Pay an additional cost for this spell which requires a choice, produced
    /// by the given ability. If multiple additional costs choices are
    /// required, this prompt will be displayed more than once.
    ///
    /// Costs which don't require a choice (e.g. life payment) are not included.
    AdditionalCost { ability: AbilityId },
}

/// Represents a possible choice a player can make in the process of playing a
/// card.
#[derive(Debug, Clone)]
pub enum PlayCardChoice {
    /// No legal option is available to the player which would allow them to
    /// play this card given the current [PlayCardPlan].
    Invalid,

    /// No further choices are required in this step of the 'play card' process,
    /// or the remaining choices only have one valid option. Advance to the next
    /// step of the process using a returned [PlayCardPlan], which will
    /// incorporate the results of those non-choices.
    Continue { updated_plan: PlayCardPlan },

    /// A choice is required in this step of the 'play card' process and the
    /// user can select one of the valid options in [PlayCardChoicePrompt].
    Prompt {
        /// True if the player can jump to the next step of the 'play card'
        /// process instead of picking a prompt option.
        ///
        /// This is used for example with "Pick up to three targets" cards.
        optional: bool,

        /// Prompt for a choice
        prompt: PlayCardChoicePrompt,
    },
}

/// Returns a choice a player must make to play a card as part of the
/// provided [PlayCardStep] using the previously-specified choices in
/// [PlayCardPlan].
///
/// A step require multiple choices, for example in the case of picking multiple
/// targets for a spell. In that case this function will return different
/// choices as the results of previous choices are added to the [PlayCardPlan].
pub fn choice_for_step(
    game: &GameState,
    source: Source,
    card_id: CardId,
    plan: &PlayCardPlan,
    step: PlayCardStep,
) -> PlayCardChoice {
    match step {
        PlayCardStep::ChooseFace => pick_face_to_play::run(game, source, card_id, plan),
        PlayCardStep::PayMana => pay_mana(game, source, card_id, &plan),
        _ => PlayCardChoice::Continue { updated_plan: plan.clone() },
    }
}

fn pay_mana(
    game: &GameState,
    source: Source,
    card_id: CardId,
    plan: &&PlayCardPlan,
) -> PlayCardChoice {
    let mana_payment_plan = spell_planner::mana_payment(game, source, card_id, &plan.spell_choices);
    match mana_payment_plan {
        Some(mana_payment_plan) => PlayCardChoice::Continue {
            updated_plan: PlayCardPlan {
                mana_payment: mana_payment_plan,
                spell_choices: plan.spell_choices.clone(),
            },
        },
        None => PlayCardChoice::Invalid,
    }
}
