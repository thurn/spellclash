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
use data::core::primitives::Zone;
use data::prompts::card_selection_prompt::CardSelectionPrompt;
use data::prompts::prompt::{Prompt, PromptType};
use data::text_strings::Text;
use rules::mutations::cards;
use rules::mutations::cards::LibraryPosition;

pub fn brainstorm() -> CardDefinition {
    CardDefinition::new(card_name::BRAINSTORM).ability(SpellAbility::new().effects(|g, _c| {
        cards::draw(g, 3);
        g.prompts.push(|g, c| Prompt {
            player: c.controller,
            label: Some(Text::ReturnToTopOfDeck(2)),
            prompt_type: PromptType::SelectCards(CardSelectionPrompt {
                unchosen_subjects: g.zones.find_cards_ordered(c.controller, Zone::Hand),
                chosen_subjects: vec![],
                can_reorder: true,
                callback: |g, card_ids| {
                    cards::move_to_library(g, LibraryPosition::Top, card_ids);
                },
            }),
        });
    }))
}
