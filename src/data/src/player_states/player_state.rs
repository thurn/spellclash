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

use crate::card_states::counters::Counters;
use crate::core::primitives::{CardId, HasController, HasObjectId, HasOwner, ObjectId, PlayerName};
use crate::player_states::mana_pool::ManaPool;
use crate::player_states::prompt_stack::PromptStack;

/// Represents the state of players within a game
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Players {
    player_1: PlayerState,
    player_2: PlayerState,
}

impl Players {
    /// Looks up a player by name
    pub fn player(&self, name: PlayerName) -> &PlayerState {
        match name {
            PlayerName::One => &self.player_1,
            PlayerName::Two => &self.player_2,
        }
    }

    /// Mutable reference to a player by name
    pub fn player_mut(&mut self, name: PlayerName) -> &mut PlayerState {
        match name {
            PlayerName::One => &mut self.player_1,
            PlayerName::Two => &mut self.player_2,
        }
    }
}

/// Represents the state of a single player within a game
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlayerState {
    /// Name of this player
    pub name: PlayerName,

    /// Object ID for this player
    pub object_id: ObjectId,

    /// Player currently controlling this player
    pub controller: PlayerName,

    /// Counters currently on this player
    pub counters: Counters,

    /// Mana currently available to this player
    pub mana_pool: ManaPool,

    /// Stack of choices currently facing this player.
    ///
    /// See [PromptStack].
    pub prompts: PromptStack,

    /// Cards this player has currently selected.
    ///
    /// Typically used as part of a multi-part prompt resolution like "pick two
    /// target creatures".
    pub selected_cards: Vec<CardId>,
}

impl PlayerState {
    pub fn new(name: PlayerName) -> Self {
        Self {
            name,
            object_id: name.object_id(),
            controller: name,
            counters: Counters::default(),
            mana_pool: ManaPool::default(),
            prompts: Default::default(),
            selected_cards: vec![],
        }
    }
}

impl HasObjectId for PlayerState {
    fn object_id(&self) -> ObjectId {
        self.object_id
    }
}

impl HasOwner for PlayerState {
    fn owner(&self) -> PlayerName {
        self.name
    }
}

impl HasController for PlayerState {
    fn controller(&self) -> PlayerName {
        self.controller
    }
}
