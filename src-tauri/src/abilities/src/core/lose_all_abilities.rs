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

use data::card_states::zones::ZoneQueries;
use data::core::primitives::{PermanentId, Timestamp};
use data::game_states::game_state::GameState;
use data::properties::card_property::LostAllAbilities;
use data::properties::duration::Duration;

/// Causes the [PermanentId] permanent to lose all abilities for the duration of
/// the current turn.
pub fn this_turn(game: &mut GameState, permanent_id: PermanentId, timestamp: impl Into<Timestamp>) {
    let turn = game.turn;
    let timestamp = timestamp.into();
    game.ability_state.this_turn.set_lost_all_abilities(permanent_id, timestamp);
    if let Some(card) = game.card_mut(permanent_id) {
        let lost = LostAllAbilities {
            timestamp,
            duration: Duration::WhileOnBattlefieldThisTurn(permanent_id, turn),
        };
        card.properties.tags.set_lost_all_abilities(lost.clone());
        card.properties.can_attack_target.set_lost_all_abilities(lost.clone());
        card.properties.can_be_blocked.set_lost_all_abilities(lost.clone());
        card.properties.has_haste.set_lost_all_abilities(lost.clone());
        card.properties.colors.set_lost_all_abilities(lost.clone());
        card.properties.creature_types.set_lost_all_abilities(lost.clone());
        card.properties.land_types.set_lost_all_abilities(lost.clone());
        card.properties.change_land_type_text.set_lost_all_abilities(lost.clone());
        card.properties.change_color_text.set_lost_all_abilities(lost.clone());
        card.properties.power.set_lost_all_abilities(lost.clone());
        card.properties.base_power.set_lost_all_abilities(lost.clone());
        card.properties.toughness.set_lost_all_abilities(lost.clone());
        card.properties.base_toughness.set_lost_all_abilities(lost.clone());
    }
}
