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

use data::card_states::play_card_plan::{ManaPaymentPlan, PlayCardPlan};
use data::card_states::zones::{ToCardId, ZoneQueries};
use data::game_states::game_state::GameState;
use data::text_strings::Text;
use primitives::game_primitives::{HasSource, PlayerName, SpellId, Zone};
use utils::outcome;
use utils::outcome::Outcome;

use crate::mutations::move_card;
use crate::play_cards::play_card;

/// Counters the indicated spell.
///
/// > 701.5a. To counter a spell or ability means to cancel it, removing it from
/// > the stack. It doesn't resolve and none of its effects occur. A countered
/// > spell is put into its owner's graveyard.
///
/// <https://yawgatog.com/resources/magic-rules/#R7015a>
pub fn counter(game: &mut GameState, source: impl HasSource, target: SpellId) -> Outcome {
    move_card::run(game, source, target, Zone::Graveyard)
}

/// Allows a player to choose new targets for a spell on the stack.
///
/// > 115.7. Some effects allow a player to change the target(s) of a spell or
/// > ability, and other effects allow a player to choose new targets for a
/// > spell or ability.
///
/// > 115.7d. If an effect allows a player to "choose new targets" for a spell
/// > or ability, the player may leave any number of the targets unchanged, even
/// > if those targets would be illegal. If the player chooses to change some or
/// > all of the targets, the new targets must be legal and must not cause any
/// > unchanged targets to become illegal.
///
/// > 115.7e. When changing targets or choosing new targets for a spell or
/// > ability, only the final set of targets is evaluated to determine whether
/// > the change is legal.
///
/// <https://yawgatog.com/resources/magic-rules/#R1157>
pub fn choose_new_targets(
    game: &mut GameState,
    source: impl HasSource,
    player: PlayerName,
    target: impl ToCardId,
) -> Outcome {
    let card_id = target.to_card_id(game)?;
    let mut plan = PlayCardPlan {
        choices: game.card(card_id)?.cast_choices.as_ref()?.clone(),
        targets: vec![],
        mana_payment: ManaPaymentPlan::default(),
    };

    play_card::select_targets(game, player, card_id, &mut plan, Text::SelectNewTargets);
    game.card_mut(card_id)?.targets = plan.targets;
    outcome::OK
}
