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

use abilities::targeting::targets;
use data::card_definitions::ability_definition::SpellAbility;
use data::card_definitions::card_definition::CardDefinition;
use data::card_definitions::card_name;
use data::card_states::zones::ZoneQueries;
use primitives::game_primitives::PermanentId;
use rules::mutations::{create_copy, permanents};

pub fn supplant_form() -> CardDefinition {
    CardDefinition::new(card_name::SUPPLANT_FORM).ability(
        SpellAbility::new().targets(targets::creature()).effect(|g, s, target: PermanentId| {
            if let Some(card) = g.card(target) {
                // Store facing before target leaves battlefield
                let (card_id, facing) = (card.id, card.facing);
                permanents::return_to_hand(g, s, card_id);
                create_copy::on_battlefield_of_card(g, card_id, s.controller, facing);
            }
        }),
    )
}
