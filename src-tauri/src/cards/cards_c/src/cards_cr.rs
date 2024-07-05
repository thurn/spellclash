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

use abilities::characteristics::change_text;
use abilities::core::effects;
use abilities::targeting::targets;
use data::card_definitions::ability_definition::SpellAbility;
use data::card_definitions::card_definition::CardDefinition;
use data::card_definitions::card_name;
use data::core::primitives::HasSource;
use data::game_states::effect_state::EffectState;
use rules::mutations::library;

pub fn craw_wurm() -> CardDefinition {
    CardDefinition::new(card_name::CRAW_WURM)
}

pub fn crystal_spray() -> CardDefinition {
    let state = EffectState::new();
    CardDefinition::new(card_name::CRYSTAL_SPRAY).ability(
        SpellAbility::new()
            .targets(targets::spell_or_permanent())
            .effect(|g, c, target| {
                let choice = change_text::choose_basic_land_types_or_colors(g, c.controller());
                state.store(g, c.effect_id, choice);
                effects::target_this_turn(g, c, target);
                library::draw(g, c.source(), c.controller());
            })
            .delegates(|d| {
                change_text::change_basic_land_type_or_color(d, state);
            }),
    )
}
