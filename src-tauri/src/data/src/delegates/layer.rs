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

use primitives::game_primitives::Timestamp;

/// Represents a layer or sublayer for resolving continuous effects.
///
/// > 613.1. The values of an object’s characteristics are determined by
/// > starting with the actual object. For a card, that means the values of the
/// > characteristics printed on that card. For a token or a copy of a spell or
/// > card, that means the values of the characteristics defined by the effect
/// > that created it. Then all applicable continuous effects are applied in a
/// > series of layers in the following order:
///
/// <https://yawgatog.com/resources/magic-rules/#R6131>
#[derive(Debug, Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
pub enum Layer {
    GameRules,
    CopyEffects,
    FaceDownEffects,
    ControlChangingCharacteristicEffects,
    ControlChangingEffects,
    TextChangingCharacteristicEffects,
    TextChangingEffects,
    TypeChangingCharacteristicEffects,
    TypeChangingEffects,
    ColorChangingCharacteristicEffects,
    ColorChangingEffects,
    AbilityModifyingCharacteristicEffects,
    AbilityModifyingEffects,
    PowerToughnessCharacteristicEffects,
    PowerToughnessSettingEffects,
    PowerToughnessModifyingEffects,
    PowerToughnessSwitchingEffects,
}

#[derive(Debug, Default, Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
pub struct EffectSortingKey {
    pub layer: Option<Layer>,
    pub timestamp: Option<Timestamp>,
}

impl EffectSortingKey {
    pub fn new(layer: Layer, timestamp: Timestamp) -> Self {
        Self { layer: Some(layer), timestamp: Some(timestamp) }
    }
}
