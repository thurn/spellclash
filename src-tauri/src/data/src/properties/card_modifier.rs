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
use primitives::game_primitives::Source;

use crate::core::layer::{EffectSortingKey, Layer};
use crate::core::rule_type;
use crate::core::rule_type::RuleType;
use crate::game_states::game_state::GameState;
use crate::properties::card_property::LostAllAbilities;
use crate::properties::duration::Duration;
use crate::properties::property_value::PropertyValue;
use crate::properties::query_condition::QueryCondition;

#[derive(Clone)]
pub struct CardModifier<TModifier> {
    pub source: Source,
    pub duration: Duration,
    pub rule_type: RuleType,
    pub effect: TModifier,
}

impl<TModifier: PropertyValue> CardModifier<TModifier> {
    /// Returns true if this modifier should currently be applied to the game.
    pub fn active(&self, game: &GameState) -> bool {
        rule_type::is_active(game, self.duration, self.rule_type, self.effect.effect_sorting_key())
    }
}
