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

use data::card_states::zones::ZoneQueries;
use data::core::primitives::CardId;
use data::delegates::scope::Scope;
use data::game_states::game_state::GameState;
use data::prompts::card_selection_prompt::{CardSelectionPrompt, Quantity};
use data::text_strings::Text;
use utils::outcome::Value;

/// Prompt to select a quantity of cards from controller's hand.
///
/// Allows reordering.
pub fn select_in_hand(
    game: &mut GameState,
    scope: Scope,
    quantity: Quantity,
    text: Text,
) -> Value<Vec<CardId>> {
    game.prompts.select_cards(
        scope.controller,
        text,
        CardSelectionPrompt::new(game.hand(scope.controller).iter().copied().collect())
            .can_reorder(true)
            .quantity(quantity),
    )
}
