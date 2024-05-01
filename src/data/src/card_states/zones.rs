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
use slotmap::SlotMap;

use crate::card_states::card_state::CardState;
use crate::card_states::stack_ability::{StackAbility, StackAbilityId};
use crate::core::numerics::Timestamp;
use crate::core::primitives::CardId;

/// Stores the state of all cards and all abilities currently on the stack.
#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct Zones {
    /// All cards, copies of cards on the stack, tokens, and emblems in the
    /// current game.
    all_cards: SlotMap<CardId, CardState>,

    /// Abilities currently on the stack
    stack_abilities: SlotMap<StackAbilityId, StackAbility>,

    /// Next timestamp to use for zone moves.
    next_timestamp: Timestamp,
}
