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
use data::core::primitives::{AbilityId, EffectId, PermanentId};
use data::delegates::scope::EffectContext;
use data::game_states::combat_state::AttackerId;
use data::game_states::effect_state::EffectState;
use data::game_states::game_state::GameState;
use data::game_states::state_value::StateValue;

/// Activates the delayed trigger associated with the current ability in
/// [EffectContext] and stores a state value in the provided [EffectState].
///
/// A copy of the trigger will be placed on the stack for each call to this
/// function once the trigger condition matches.
pub fn enable<T: Into<StateValue> + TryFrom<StateValue> + PartialEq>(
    game: &mut GameState,
    context: EffectContext,
    state: &EffectState<T>,
    state_value: T,
) {
    game.ability_state
        .delayed_triggers
        .entry(context.ability_id())
        .or_default()
        .push(context.effect_id);
    state.store(game, context.effect_id, state_value);
}

/// Deactivates the delayed trigger instance associated with the given
/// [AbilityId] and [EffectId].
pub fn disable(game: &mut GameState, ability_id: AbilityId, effect_id: EffectId) {
    if let Some(list) = game.ability_state.delayed_triggers.get_mut(&ability_id) {
        list.retain(|&id| id != effect_id);
        if list.is_empty() {
            game.ability_state.delayed_triggers.remove(&ability_id);
        }
    }
}
