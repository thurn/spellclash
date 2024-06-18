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

use data::card_definitions::ability_definition::StaticAbility;
use data::card_definitions::card_definition::CardDefinition;
use data::card_definitions::card_name;
use data::core::primitives::Zone;
use data::delegates::game_delegates::AttackerData;
use data::game_states::game_state::GameState;

pub fn dandan() -> CardDefinition {
    CardDefinition::new(card_name::DANDAN).ability(
        StaticAbility::new().delegate(Zone::Battlefield, |on| {
            on.can_attack.this(|g, _, data, _| controls_island(g, data))
        }),
    )
}

fn controls_island(_game: &GameState, _data: &AttackerData) -> bool {
    true
}
