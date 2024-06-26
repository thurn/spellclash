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
use data::core::primitives::PermanentId;
use data::delegates::scope::EffectContext;
use data::game_states::combat_state::AttackerId;
use data::game_states::effect_state::EffectState;
use data::game_states::game_state::GameState;

pub fn enable<TTarget>(
    game: &mut GameState,
    scope: EffectContext,
    state: &EffectState<TTarget>,
    target: TTarget,
) {
    todo!()
}
