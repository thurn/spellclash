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

use data::card_states::play_card_plan::{PlayCardPlan, PlayCardTiming};
use data::card_states::zones::ZoneQueries;
use data::core::primitives::{CardId, PlayerName, Source, Zone};
use data::game_states::game_state::GameState;
use data::player_states::player_state::PlayerQueries;
use utils::outcome;
use utils::outcome::Outcome;

use crate::mutations::{move_card, permanents, priority};

/// Plays a card, based on the set of choices in a completed [PlayCardPlan].
///
/// This will normally move spells to the stack and land cards to the
/// battlefield.
pub fn execute_plan(
    game: &mut GameState,
    player: PlayerName,
    card_id: CardId,
    source: Source,
    plan: PlayCardPlan,
) -> Outcome {
    for land in &plan.mana_payment.basic_land_abilities_to_activate {
        permanents::tap(game, source, *land)?;
    }

    if plan.spell_choices.play_as.play_as == PlayCardTiming::Land {
        game.history_counters_mut(player).lands_played += 1;
        let face = plan.spell_choices.play_as.single_face();
        permanents::turn_face_up(game, source, card_id, face)?;
        move_card::run(game, source, card_id, Zone::Battlefield)?;
    } else {
        game.card_mut(card_id).cast_as = plan.spell_choices.play_as.faces;
        move_card::run(game, source, card_id, Zone::Stack)?;

        // Once a card is played, abilities trigger and then a new priority round is created:
        //
        // > 601.2i. Once the steps described in 601.2a-h are completed, effects that modify the
        // > characteristics of the spell as it's cast are applied, then the spell becomes cast.
        // > Any abilities that trigger when a spell is cast or put onto the stack trigger at this
        // > time. If the spell's controller had priority before casting it, they get priority.
        game.passed_mut().clear();
        if !game.player(player).options.hold_priority {
            // Automatically pass priority after putting something on the stack.
            priority::pass(game, player)?;
        }
    }

    outcome::OK
}
