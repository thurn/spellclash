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

use data::card_states::play_card_plan::PlayCardPlan;
use data::core::primitives::{CardId, PlayerName, Source};
use data::game_states::game_state::GameState;
use utils::outcome;
use utils::outcome::Outcome;

/// Plays a card, based on the set of choices in a completed [PlayCardPlan].
pub fn execute_plan(
    game: &mut GameState,
    player: PlayerName,
    card_id: CardId,
    source: Source,
    plan: PlayCardPlan,
) -> Outcome {
    outcome::OK
}
