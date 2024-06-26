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

use abilities::core::effects;
use abilities::targeting::targets;
use data::card_definitions::ability_definition::SpellAbility;
use data::card_definitions::card_definition::CardDefinition;
use data::card_definitions::card_name;
use rules::queries::query_extension::QueryExt;

pub fn giant_growth() -> CardDefinition {
    CardDefinition::new(card_name::GIANT_GROWTH).ability(
        SpellAbility::new()
            .targets(targets::creature())
            .effect(effects::target_this_turn)
            .delegates(|d| {
                d.power.this_turn(|_, _, _, current| current + 3);
                d.toughness.this_turn(|_, _, _, current| current + 3);
            }),
    )
}

pub fn gigantosaurus() -> CardDefinition {
    CardDefinition::new(card_name::GIGANTOSAURUS)
}
