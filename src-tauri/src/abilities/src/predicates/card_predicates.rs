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

use data::core::primitives::CardId;
use data::delegates::scope::DelegateScope;
use data::game_states::game_state::GameState;
use data::printed_cards::card_subtypes::LandSubtype;
use rules::queries::card_queries;

/// True if this card is an island.
pub fn island(game: &GameState, _: DelegateScope, card_id: CardId) -> bool {
    card_queries::land_subtypes(game, card_id).contains(LandSubtype::Island)
}
