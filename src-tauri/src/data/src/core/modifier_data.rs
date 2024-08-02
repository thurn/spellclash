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

use primitives::game_primitives::{PermanentId, Source, PRINTED_TEXT_TIMESTAMP};

use crate::delegates::layer::{EffectSortingKey, Layer, PRINTED_RULE_SORTING_KEY};
use crate::delegates::scope::AbilityScope;
use crate::events::event_context::EventContext;
use crate::properties::duration::Duration;

/// Possible ways of modifying a card.
#[derive(Clone, Copy)]
pub enum ModifierMode {
    /// Modifier based on an ability printed on a card.
    PrintedAbility(AbilityScope),

    /// Modifier created by a game effect.
    Effect(EventContext, Layer, Duration),
}

impl ModifierMode {
    /// Creates a [ModifierMode] to create an ability-modifying effect which
    /// applies to the [PermanentId] permanent this turn.
    pub fn add_ability_this_turn(context: EventContext, id: PermanentId) -> Self {
        ModifierMode::Effect(
            context,
            Layer::AbilityModifyingEffects,
            Duration::WhileOnBattlefieldThisTurn(id, context.current_turn),
        )
    }

    pub fn sorting_key(&self) -> EffectSortingKey {
        match self {
            ModifierMode::PrintedAbility(_) => PRINTED_RULE_SORTING_KEY,
            ModifierMode::Effect(context, layer, _) => {
                EffectSortingKey::new(*layer, context.event_id.timestamp())
            }
        }
    }
}
