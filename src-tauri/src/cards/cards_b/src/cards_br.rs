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

use data::card_definitions::ability_definition::SpellAbility;
use data::card_definitions::card_definition::CardDefinition;
use data::card_definitions::card_name;
use data::prompts::card_select_and_order_prompt::Quantity;
use rules::mutations::library;
use rules::prompt_handling::prompts;

pub fn brainstorm() -> CardDefinition {
    CardDefinition::new(card_name::BRAINSTORM).ability(SpellAbility::new().effects(|g, s| {
        library::draw_cards(g, s, s.controller, 3);
        let ids = prompts::hand_to_top_of_library(g, s, Quantity::Count(2));
        library::move_all_to_top(g, s, ids.iter())
    }))
}
