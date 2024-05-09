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

use data::card_states::play_card_plan::CastSpellChoices;
use data::core::primitives::{CardId, Source};
use data::game_states::game_state::GameState;

/// Returns true if there is any set of choices the [CardId]'s controller could
/// make to successfully cast this card, given the set of choices selected in
/// [CastSpellChoices].
pub fn to_cast(
    game: &GameState,
    _source: Source,
    card_id: CardId,
    choices: CastSpellChoices,
) -> bool {
    true
}
