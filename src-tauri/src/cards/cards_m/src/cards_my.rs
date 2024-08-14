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
use primitives::game_primitives::{CardType, Zone};
use rules::mutations::move_card;

pub fn mystic_retrieval() -> CardDefinition {
    CardDefinition::new(card_name::MYSTIC_RETRIEVAL).ability(
        SpellAbility::new()
            .targets(targets::card_in_your_graveyard_with_type(
                CardType::Instant | CardType::Sorcery,
            ))
            .effect(|g, c, target| {
                move_card::run(g, c, target, Zone::Hand);
            }),
    )
}
