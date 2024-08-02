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

use primitives::game_primitives::{AbilityId, CardId, PermanentId};

use crate::core::layer::{EffectSortingKey, Layer, PRINTED_RULE_SORTING_KEY};
use crate::game_states::game_state::GameState;
use crate::properties::duration::Duration;

/// Possible types of property modifiers or game event callbacks.
///
/// Used primarily to handle the 'lose all abilities' effect.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RuleType {
    /// Modification or callback is a game effect which cannot be removed.
    Effect,

    /// Modification or callback is a result of an ability of the [CardId] card
    /// and should stop applying if this card loses all abilities.
    Ability(CardId),
}

/// Returns true if a property modifier or game event callback is currently
/// active.
///
/// A modifier or callback can be inactive it it has exceeded its [Duration], or
/// if its timestamp and layer are before the timestamp and layer of a 'lose
/// all abilities' effect.
pub fn is_active(
    game: &GameState,
    duration: Duration,
    rule_type: RuleType,
    effect_sorting_key: Option<EffectSortingKey>,
) -> bool {
    let key = effect_sorting_key.unwrap_or(PRINTED_RULE_SORTING_KEY);

    if !duration.is_active(game) {
        return false;
    }

    let RuleType::Ability(origin_card_id) = rule_type else {
        return true;
    };

    let Some(lost_at_timestamp) = game.has_lost_all_abilities(origin_card_id) else {
        return true;
    };

    // Abilities are lost unless this modifier/event was added after the 'lose all
    // abilities' effect, or if this item exists in a layer above the 'ability
    // modifying effects' layer.
    key > EffectSortingKey::new(Layer::AbilityModifyingEffects, lost_at_timestamp)
}
