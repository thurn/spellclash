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

use color_eyre::owo_colors::Effect;
use dyn_clone::DynClone;

use crate::core::primitives::Source;
use crate::delegates::delegate_type::DelegateType;
use crate::delegates::layer::{EffectSortingKey, Layer};
use crate::delegates::query_value::QueryValue;
use crate::delegates::scope::Scope;
use crate::game_states::game_state::GameState;
use crate::queries::card_query::LostAllAbilities;
use crate::queries::duration::Duration;
use crate::queries::query_condition::QueryCondition;

#[derive(Clone)]
pub struct CardModifier<TModifier: QueryValue> {
    pub source: Source,
    pub duration: Duration,
    pub delegate_type: DelegateType,
    pub effect: TModifier,
}

impl<TModifier: QueryValue> CardModifier<TModifier> {
    pub fn active(&self, game: &GameState, lost_all_abilities: &Option<LostAllAbilities>) -> bool {
        if !self.duration.is_active(game) {
            return false;
        }

        if let Some(lost_all) = lost_all_abilities {
            if let Some(k) = self.effect.effect_sorting_key() {
                if self.delegate_type == DelegateType::Ability
                    && k < EffectSortingKey::new(Layer::AbilityModifyingEffects, lost_all.timestamp)
                {
                    return false;
                }
            }
        }

        true
    }
}
