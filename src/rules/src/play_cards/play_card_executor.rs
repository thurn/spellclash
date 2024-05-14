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
use utils::outcome::Outcome;
use utils::{fail, outcome};

use crate::mutations::{permanents, priority};

/// Plays a card, based on the set of choices in a completed [PlayCardPlan].
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
        let face = plan.spell_choices.play_as.single_face()?;
        permanents::turn_face_up(game, source, card_id, face)?;
        game.move_card(source, card_id, Zone::Battlefield)
    } else {
        game.card_mut(card_id).cast_as = plan.spell_choices.play_as.faces;
        game.move_card(source, card_id, Zone::Stack)?;

        // Automatically pass priority after putting something on the stack.
        // TODO: Implement holding priority.
        priority::pass(game, player)
    }
}
