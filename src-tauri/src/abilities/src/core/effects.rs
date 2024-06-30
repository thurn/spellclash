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
use data::core::primitives::{CardId, EntityId, PermanentId};
use data::delegates::scope::{EffectContext, Scope};
use data::game_states::game_state::GameState;
use utils::outcome;
use utils::outcome::Outcome;

/// Marks a card as applying an effect to the target permanent until the end of
/// the current turn.
pub fn apply_this_turn(game: &mut GameState, context: EffectContext, target: impl Into<EntityId>) {
    game.ability_state.this_turn.add_effect(context.ability_id(), context.effect_id, target.into());
}
