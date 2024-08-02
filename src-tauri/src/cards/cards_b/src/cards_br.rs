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
use data::card_states::zones::ZoneQueries;
use data::prompts::select_order_prompt::CardOrderLocation;
use data::text_strings::Text;
use rules::mutations::library;
use rules::prompt_handling::prompts;

pub fn brainstorm() -> CardDefinition {
    CardDefinition::new(card_name::BRAINSTORM).ability(SpellAbility::new().effect(|g, c| {
        library::draw_cards(g, c, c.controller, 3);
        let cards = prompts::select_ordered_from(
            g,
            c.controller,
            Text::HandToTopOfLibraryPrompt,
            &g.hand(c.controller).clone(),
            2,
            CardOrderLocation::TopOfLibrary,
        );
        library::move_all_to_top(g, c, &cards)
    }))
}

pub fn broodhunter_wurm() -> CardDefinition {
    CardDefinition::new(card_name::BROODHUNTER_WURM)
}
