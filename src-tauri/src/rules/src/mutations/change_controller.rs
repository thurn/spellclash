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
use data::card_states::zones::{ToCardId, ZoneQueries};
use data::events::card_events;
use data::events::card_events::PermanentControllerChangedEvent;
use data::game_states::game_state::GameState;
use primitives::game_primitives::{
    AbilityId, CardId, EventId, HasController, HasSource, PlayerName, Source,
};
use utils::outcome;
use utils::outcome::Outcome;

use crate::dispatcher::dispatch;

/// Causes `new_controller` to gain control of the [CardId] card.
///
/// The caller of this function is responsible for removing this status via
/// [remove_control] if it ends. The effect will also automatically end if this
/// card changes zones, except for a transition from the stack to the
/// battlefield.
pub fn gain_control(
    game: &mut GameState,
    source: Source,
    new_controller: PlayerName,
    event_id: EventId,
    card_id: impl ToCardId,
) -> Outcome {
    let card_id = card_id.to_card_id(game)?;
    let current = game.card(card_id)?.controller();

    if current != new_controller {
        game.zones.on_controller_changed(card_id, current, new_controller, game.turn);
        let turn = game.turn;
        let card = game.card_mut(card_id)?;
        let permanent_id = card.permanent_id();
        card.last_changed_control = turn;
        card.control_changing_effects
            .push(ControlChangingEffect { event_id, controller: new_controller });

        if let Some(id) = permanent_id {
            dispatch::card_event(
                game,
                card_id,
                |c| &c.controller_changed,
                source,
                &PermanentControllerChangedEvent {
                    permanent_id: id,
                    old_controller: current,
                    new_controller,
                },
            );
        }
    }
    outcome::OK
}

/// Gains control of the [CardId] card as described in [gain_control] for the
/// duration of the current turn. This effect is automatically ended in the
/// cleanup step.
pub fn gain_control_this_turn(
    game: &mut GameState,
    source: impl HasSource,
    new_controller: PlayerName,
    event_id: EventId,
    id: impl ToCardId,
) -> Outcome {
    let card_id = id.to_card_id(game)?;
    game.ability_state.this_turn.add_control_changing_effect(event_id, card_id);
    gain_control(game, source.source(), new_controller, event_id, card_id)
}

/// Removes all control-changing effects from the [CardId] card that were added
/// by the given [EventId].
pub fn remove_control(game: &mut GameState, event_id: EventId, card_id: CardId) -> Outcome {
    let card = game.card_mut(card_id)?;
    let current = card.controller();
    card.control_changing_effects.retain(|effect| effect.event_id != event_id);
    let new = card.controller();
    if current != new {
        game.zones.on_controller_changed(card_id, current, new, game.turn);
        let turn = game.turn;
        let card = game.card_mut(card_id)?;
        card.last_changed_control = turn;
        let permanent_id = card.permanent_id();
        if let Some(id) = permanent_id {
            dispatch::card_event(
                game,
                card_id,
                |c| &c.controller_changed,
                Source::Game,
                &PermanentControllerChangedEvent {
                    permanent_id: id,
                    old_controller: current,
                    new_controller: new,
                },
            );
        }
    }
    outcome::OK
}
