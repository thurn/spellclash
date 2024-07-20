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

use crate::core::primitives::{AbilityId, EventId, PlayerName, Source};
use crate::game_states::game_state::TurnData;

/// Data passed as a parameter to an event callback function.
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub struct EventContext {
    /// A unique identifier for this instance of a callback function being
    /// invoked
    pub event_id: EventId,

    /// The [AbilityId] of the ability which created this callback function
    pub this: AbilityId,

    /// The player who currently controls the ability which created this
    /// callback function
    pub controller: PlayerName,

    /// The current turn of the game.
    pub current_turn: TurnData,

    /// The [Source] of the effect which caused this event to be fired.
    pub event_source: Source,
}

impl EventContext {
    /// Returns a [Source] representing the ability which added this callback
    /// function.
    ///
    /// Note that this is different from the source which *triggered* the event.
    pub fn this_source(&self) -> Source {
        Source::Ability { ability_id: self.this, controller: self.controller }
    }
}
