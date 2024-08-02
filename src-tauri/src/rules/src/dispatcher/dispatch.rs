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

use data::card_states::zones::{ToCardId, ZoneQueries};
use data::core::rule_type;
use data::events::card_events::CardEvents;
use data::events::event_context::EventContext;
use data::events::game_event::{GameEvent, GameEventCallback};
use data::events::game_events::GlobalEvents;
use data::game_states::game_state::GameState;
use primitives::game_primitives::{AbilityId, EventId, HasController, Source};
use utils::outcome;
use utils::outcome::Outcome;

pub fn add_card_event(
    game: &mut GameState,
    id: impl ToCardId,
    function: impl Fn(&mut CardEvents),
) -> Outcome {
    function(&mut game.card_mut(id)?.events);
    outcome::OK
}

pub fn game_event<TArg: 'static>(
    game: &mut GameState,
    event: fn(&GlobalEvents) -> &GameEvent<TArg>,
    source: Source,
    arg: TArg,
) {
    for i in 0..event(&game.events).callbacks.len() {
        outcome::execute(|| {
            let callback = &event(&game.events).callbacks[i];
            let mut context = build_callback_context(callback, game, source)?;

            // Do this after checking validity to avoid generating IDs when the function
            // isn't going to run
            context.event_id = EventId(game.zones.new_timestamp().0);

            let function = event(&game.events).callbacks[i].function.clone();
            function.invoke(game, context, &arg);
            outcome::OK
        });
    }
}

pub fn card_event<TArg: 'static>(
    game: &mut GameState,
    id: impl ToCardId,
    event: fn(&CardEvents) -> &GameEvent<TArg>,
    source: Source,
    arg: &TArg,
) -> Outcome {
    for i in 0..event(&game.card(id)?.events).callbacks.len() {
        outcome::execute(|| {
            let callback = &event(&game.card(id)?.events).callbacks[i];
            let mut context = build_callback_context(callback, game, source)?;

            // Do this after checking validity to avoid generating IDs when the function
            // isn't going to run
            context.event_id = EventId(game.zones.new_timestamp().0);

            let function = event(&game.card(id)?.events).callbacks[i].function.clone();
            function.invoke(game, context, arg);
            outcome::OK
        });
    }
    outcome::OK
}

/// Creates a new [EventContext] for invoking the effect of the given ability.
///
/// Returns None if the card which owns this ability no longer exists.
pub fn build_invocation_context(
    game: &mut GameState,
    ability_id: AbilityId,
) -> Option<EventContext> {
    let current_turn = game.turn;
    let event_id = EventId(game.zones.new_timestamp().0);
    let controller = game.card(ability_id)?.controller();
    Some(EventContext {
        event_id,
        this: ability_id,
        controller,
        current_turn,
        original_source: Source::Ability(ability_id),
    })
}

/// Builds the [EventContext] for invoking an event callback function.
fn build_callback_context<TArg>(
    callback: &GameEventCallback<TArg>,
    game: &GameState,
    original_source: Source,
) -> Option<EventContext> {
    let card = game.card(callback.ability_id)?;
    if !callback.zones.contains(card.zone) {
        return None;
    };

    if !rule_type::is_active(
        game,
        callback.duration,
        callback.rule_type,
        callback.effect_sorting_key,
    ) {
        return None;
    }

    Some(EventContext {
        event_id: EventId::default(),
        this: callback.ability_id,
        controller: card.controller(),
        current_turn: game.turn,
        original_source,
    })
}
