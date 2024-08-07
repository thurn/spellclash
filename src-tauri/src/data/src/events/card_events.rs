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

use invokable_macro::Invokable;
use primitives::game_primitives::{PermanentId, PlayerName, Source};
use utils::outcome;
use utils::outcome::Outcome;

use crate::card_states::zones::{ToCardId, ZoneQueries};
use crate::events::game_event::GameEvent;
use crate::game_states::game_state::GameState;

#[derive(Debug, Clone, Copy)]
pub struct PermanentControllerChangedEvent {
    pub permanent_id: PermanentId,
    pub old_controller: PlayerName,
    pub new_controller: PlayerName,
}

#[derive(Default, Clone, Debug, Invokable)]
pub struct CardEvents {
    /// A card is about to enter the battlefield and be assigned a
    /// [PermanentId].
    ///
    /// Note that this is *not yet* the current [PermanentId] of this entity.
    pub will_enter_battlefield: GameEvent<PermanentId>,

    /// The card with the given [PermanentId] is about to leave the battlefield.
    pub will_leave_battlefield: GameEvent<PermanentId>,

    /// Invoked whenever a permanent's controller explicitly changes
    ///
    /// This is *not* invoked when e.g. the permanent changes zones and reverts
    /// to its owner's control.
    pub controller_changed: GameEvent<PermanentControllerChangedEvent>,
}
