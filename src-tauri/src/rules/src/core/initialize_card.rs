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

use data::card_definitions::definitions;
use data::card_states::zones::ZoneQueries;
use data::core::ability_scope::AbilityScope;
use data::game_states::game_state::GameState;
use primitives::game_primitives::{AbilityId, CardId};
use utils::outcome;
use utils::outcome::Outcome;

/// Runs initial setup code for a card's abilities immediately after it is
/// created.
pub fn run(game: &mut GameState, card_id: CardId) -> Outcome {
    let name = game.card(card_id)?.card_name;
    for (number, ability) in definitions::get(name).iterate_abilities() {
        let ability_id = AbilityId { card_id, number };
        let ability_scope = AbilityScope { ability_id };
        let card = game.card_mut(card_id)?;
        ability.add_properties(ability_scope, card);
        ability.add_card_events(ability_scope, &mut card.events);
        ability.add_global_events(ability_scope, &mut game.events);
    }
    outcome::OK
}
