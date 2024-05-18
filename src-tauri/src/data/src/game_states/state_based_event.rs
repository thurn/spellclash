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

use serde::{Deserialize, Serialize};

use crate::core::primitives::{CardId, EntityId, PlayerName};

/// Represents an event which *may* result in changes to the game state when
/// state-based actions are checked.
///
/// In order to make state-based action checks efficient, all game mutations
/// that may trigger one are required to register an associated event here.
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum StateBasedEvent {
    LifeTotalDecrease(PlayerName),
    DrawFromEmptyLibrary(PlayerName),
    GainedPoisonCounters(PlayerName),
    TokenLeftBattlefield(CardId),
    CopyLeftStackOrBattlefield(CardId),
    CreatureToughnessChanged(CardId),
    CreatureDamaged(CardId),
    CreatureDamagedByDeathtouch(CardId),
    PlaneswalkerLostLoyalty(CardId),
    LegendaryPermanentEntered(CardId),
}
