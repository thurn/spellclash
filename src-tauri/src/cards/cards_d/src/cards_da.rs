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
use abilities::triggers::state_triggers;
use data::card_definitions::ability_choices::{
    AbilityChoiceBuilder, AbilityTarget, AbilityTargetPredicate, AbilityTargetQuantity,
};
use data::card_definitions::ability_definition::SpellAbility;
use data::card_definitions::card_definition::CardDefinition;
use data::card_definitions::card_name;
use data::core::primitives::{CardType, Zone};
use enumset::EnumSet;
use rules::mutations::permanents;
use rules::queries::card_queries;
use utils::outcome;

pub fn dance_of_the_skywise() -> CardDefinition {
    CardDefinition::new(card_name::DANCE_OF_THE_SKYWISE).ability(
        SpellAbility::new()
            .target(AbilityTarget {
                quantity: AbilityTargetQuantity::Exactly(1),
                predicate: AbilityTargetPredicate::Card(
                    EnumSet::only(Zone::Battlefield),
                    Box::new(|g, _, id| {
                        card_queries::card_types(g, id).contains(CardType::Creature)
                    }),
                ),
            })
            .effects(|g, s| outcome::OK),
    )
}

pub fn dandan() -> CardDefinition {
    CardDefinition::new(card_name::DANDAN)
        .ability(attack_restrictions::cannot_attack_unless_defender_controls(
            card_predicates::island,
        ))
        .ability(state_triggers::when_controls_no(card_predicates::island, permanents::sacrifice))
}
