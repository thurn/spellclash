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

use abilities::predicates::card_predicates;
use abilities::restrictions::attack_restrictions;
use abilities::targeting::targets;
use abilities::triggers::state_triggers;
use data::card_definitions::ability_choices::AbilityChoiceBuilder;
use data::card_definitions::ability_definition::SpellAbility;
use data::card_definitions::card_definition::CardDefinition;
use data::card_definitions::card_name;
use data::core::primitives::{HasSource, Zone};
use rules::mutations::{move_card, permanents};

pub fn dance_of_the_skywise() -> CardDefinition {
    CardDefinition::new(card_name::DANCE_OF_THE_SKYWISE).ability(
        SpellAbility::new()
            .target(targets::creature())
            .effect(|g, s, target| move_card::run(g, s.source(), target, Zone::Graveyard)),
    )
}

pub fn dandan() -> CardDefinition {
    CardDefinition::new(card_name::DANDAN)
        .ability(attack_restrictions::cannot_attack_unless_defender_controls(
            card_predicates::island,
        ))
        .ability(state_triggers::when_controls_no(card_predicates::island, permanents::sacrifice))
}
