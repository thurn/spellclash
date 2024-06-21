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

use data::card_states::card_state::ControlChangingEffect;
use data::card_states::zones::ZoneQueries;
use data::core::primitives::{AbilityId, CardId, EffectId, HasController, HasSource, PlayerName};
use data::delegates::scope::{DelegateScope, EffectScope};
use data::game_states::game_state::GameState;
use utils::outcome;
use utils::outcome::Outcome;

/// Causes the controller of [EffectScope] to gain control of the [CardId]
/// card.
///
/// The caller of this function is responsible for removing this status via
/// [remove_control] if it ends. The effect will also automatically end if this
/// card changes zones, except for a transition from the stack to the
/// battlefield.
pub fn gain_control(game: &mut GameState, scope: EffectScope, card_id: CardId) -> Outcome {
    let current = game.card(card_id).controller();
    if current != scope.controller {
        game.zones.on_controller_changed(card_id, current, scope.controller, game.turn);
        game.card_mut(card_id).last_changed_control = game.turn;
        game.card_mut(card_id).control_changing_effects.push(ControlChangingEffect {
            effect_id: scope.effect_id,
            controller: scope.controller,
        });
    }
    outcome::OK
}

/// Gains control of the [CardId] card as described in [gain_control] for the
/// duration of the current turn. This effect is automatically ended in the
/// cleanup step.
pub fn gain_control_this_turn(
    game: &mut GameState,
    scope: EffectScope,
    card_id: CardId,
) -> Outcome {
    game.this_turn.add_control_changing_effect(scope.effect_id, card_id);
    gain_control(game, scope, card_id)
}

/// Removes all control-changing effects from the [CardId] card that were added
/// by the given [EffectId].
pub fn remove_control(game: &mut GameState, effect_id: EffectId, card_id: CardId) -> Outcome {
    let current = game.card(card_id).controller();
    game.card_mut(card_id).control_changing_effects.retain(|effect| effect.effect_id != effect_id);
    let new = game.card(card_id).controller();
    if current != new {
        game.zones.on_controller_changed(card_id, current, new, game.turn);
        game.card_mut(card_id).last_changed_control = game.turn;
    }
    outcome::OK
}
