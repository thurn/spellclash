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

use abilities::characteristics::power_toughness;
use abilities::targeting::targets;
use data::card_definitions::ability_definition::SpellAbility;
use data::card_definitions::card_definition::CardDefinition;
use data::card_definitions::card_name;
use data::card_definitions::registry::Registry;

pub fn giant_growth(_: &mut Registry) -> CardDefinition {
    CardDefinition::new(card_name::GIANT_GROWTH).ability(
        SpellAbility::new().targets(targets::creature()).effect(|g, s, target| {
            power_toughness::add_this_turn(g, s, target, 3, 3);
        }),
    )
}

pub fn gigantosaurus(_: &mut Registry) -> CardDefinition {
    CardDefinition::new(card_name::GIGANTOSAURUS)
}
