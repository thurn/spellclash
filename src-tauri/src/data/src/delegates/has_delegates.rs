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

use std::fmt::Debug;

use crate::card_states::zones::ZoneQueries;
use crate::core::primitives::{AbilityId, CardId, Zone};
use crate::delegates::scope::Scope;
use crate::game_states::game_state::GameState;

/// Marker trait for types which store and can query delegates.
pub trait HasDelegates {
    type ScopeType: Copy + Clone + Debug;

    fn current_zone(&self, card_id: CardId) -> Zone;

    fn create_scope(&self, ability_id: AbilityId) -> Self::ScopeType;
}

impl HasDelegates for GameState {
    type ScopeType = Scope;

    fn current_zone(&self, card_id: CardId) -> Zone {
        self.card(card_id).zone
    }

    fn create_scope(&self, ability_id: AbilityId) -> Self::ScopeType {
        Scope { controller: self.card(ability_id).controller, ability_id }
    }
}
