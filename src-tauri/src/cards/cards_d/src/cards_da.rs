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

use abilities::characteristics::{base_power_toughness, colors, creature_subtypes};
use abilities::core::{effects, lose_all_abilities};
use abilities::restrictions::attack_restrictions;
use abilities::targeting::targets;
use abilities::triggers::state_triggers;
use data::card_definitions::ability_definition::SpellAbility;
use data::card_definitions::card_definition::CardDefinition;
use data::card_definitions::card_name;
use data::core::primitives::Color;
use data::printed_cards::card_subtypes::CreatureSubtype;
use rules::mutations::permanents;
use rules::predicates::card_predicates;

pub fn dance_of_the_skywise() -> CardDefinition {
    CardDefinition::new(card_name::DANCE_OF_THE_SKYWISE).ability(
        SpellAbility::new()
            .targets(targets::creature_you_control())
            .effect(|g, c, target| {
                effects::target_this_turn(g, c, target);
                lose_all_abilities::this_turn(g, target);
            })
            .delegates(|d| {
                colors::for_target_this_turn(d, Color::Blue);
                creature_subtypes::for_target_this_turn(
                    d,
                    CreatureSubtype::Dragon | CreatureSubtype::Illusion,
                );
                base_power_toughness::for_target_this_turn(d, 5, 5);
            }),
    )
}

pub fn dandan() -> CardDefinition {
    CardDefinition::new(card_name::DANDAN)
        .ability(attack_restrictions::cannot_attack_unless_defender_controls(
            card_predicates::island,
        ))
        .ability(state_triggers::when_controls_no(card_predicates::island, permanents::sacrifice))
}
