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
use data::events::card_events::CardEvents;
use data::events::event_context::EventContext;
use data::game_states::game_state::GameState;
use primitives::game_primitives::{AbilityId, EventId, HasController, Source};
use utils::outcome;
use utils::outcome::Outcome;

/// Creates a new [EventContext] for the given ability.
///
/// Returns None if the card which owns this ability no longer exists.
pub fn build_event_context(game: &mut GameState, ability_id: AbilityId) -> Option<EventContext> {
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

pub fn add_card_event(
    game: &mut GameState,
    id: impl ToCardId,
    function: impl Fn(&mut CardEvents),
) -> Outcome {
    function(&mut game.card_mut(id)?.events);
    outcome::OK
}
