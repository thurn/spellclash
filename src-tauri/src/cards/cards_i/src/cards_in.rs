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
use data::card_definitions::modal_effect::{AbilityMode, ModalEffect};
use rules::mutations::{create_copy, spells};

pub fn insidious_will() -> CardDefinition {
    CardDefinition::new(card_name::INSIDIOUS_WILL).ability(
        SpellAbility::new().modal_effect(
            ModalEffect::new()
                .mode(AbilityMode::new().targets(targets::spell()).effect(|g, c, target| {
                    spells::counter(g, c, target);
                }))
                .mode(AbilityMode::new().targets(targets::spell()).effect(|g, c, target| {
                    spells::choose_new_targets(g, c, target);
                }))
                .mode(AbilityMode::new().targets(targets::spell()).effect(|g, c, target| {
                    create_copy::of_spell(g, target, c.controller);
                })),
        ),
    )
}
