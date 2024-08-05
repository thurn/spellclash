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

use data::game_states::game_state::GameState;
use primitives::game_primitives::{HasSource, SpellId, Zone};
use utils::outcome;
use utils::outcome::Outcome;

use crate::mutations::move_card;

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

pub fn choose_new_targets(
    game: &mut GameState,
    source: impl HasSource,
    target: SpellId,
) -> Outcome {
    todo!("Implement choose_new_targets")
}
