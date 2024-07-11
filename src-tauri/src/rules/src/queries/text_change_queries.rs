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

use data::core::primitives::{Color, Source};
use data::game_states::game_state::GameState;
use data::printed_cards::card_subtypes::LandType;

/// Returns the [LandType] which has replaced the given `subtype` for the
/// card providing the given [Source], if any. Otherwise, returns the original
/// subtype.
pub fn land_subtype(game: &GameState, source: Source, subtype: LandType) -> LandType {
    match source {
        Source::Game => subtype,
        Source::Ability { ability_id, .. } => game.delegates.change_land_subtype_text.query(
            game,
            source,
            &ability_id.card_id,
            subtype,
        ),
    }
}

/// Returns the [Color] which has replaced the given `color` for the
/// card providing the given [Source], if any. Otherwise, returns the original
/// color.
pub fn color(game: &GameState, source: Source, color: Color) -> Color {
    match source {
        Source::Game => color,
        Source::Ability { ability_id, .. } => {
            game.delegates.change_color_text.query(game, source, &ability_id.card_id, color)
        }
    }
}
