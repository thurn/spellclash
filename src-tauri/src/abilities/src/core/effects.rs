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
use data::delegates::delegate_type::DelegateType;
use data::delegates::game_delegates::GameDelegates;
use data::delegates::scope::{EffectContext, Scope};
use data::game_states::game_state::GameState;
use utils::outcome;
use utils::outcome::Outcome;

/// Marks a card as applying an effect to the target entity until the end of
/// the current turn.
pub fn target_this_turn(game: &mut GameState, context: EffectContext, target: impl Into<EntityId>) {
    game.ability_state.this_turn.add_effect(context.ability_id(), context.effect_id, target.into());
}

/// Marks a card's "this turn" effects as continuing to affect their target
/// entity after it enters the battlefield.
///
/// This works by changing the registered [EntityId] for the effect when a card
/// enters the battlefield. This is needed for e.g. text-changing effects, where
/// changing the text of a permanent spell for the current turn should continue
/// to apply when that permanent is on the battlefield.
pub fn preserve_this_turn_effects_when_entering_battlefield(delegates: &mut GameDelegates) {
    delegates.will_enter_battlefield.whenever(DelegateType::Effect, |g, s, data| {
        if let Some(entity_id) = g.card(data.card_id).map(|c| c.entity_id()) {
            g.ability_state.this_turn.change_affected_entity_id(
                s.ability_id,
                entity_id,
                data.future_permanent_id.into(),
            );
        }
    });
}
