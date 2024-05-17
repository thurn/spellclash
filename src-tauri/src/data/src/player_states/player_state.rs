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
use crate::core::numerics::LifeValue;
use crate::core::primitives::{
    CardId, EntityId, HasController, HasEntityId, HasPlayerName, PlayerName, UserId,
};
use crate::player_states::mana_pool::ManaPool;
use crate::player_states::player_options::PlayerOptions;
use crate::player_states::prompt_stack::PromptStack;

pub trait PlayerQueries {
    /// Looks up a player by name
    fn player(&self, name: PlayerName) -> &PlayerState;

    /// Mutable version of [Self::player].
    fn player_mut(&mut self, name: PlayerName) -> &mut PlayerState;
}

/// Represents the state of players within a game
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Players {
    player_1: PlayerState,
    player_2: PlayerState,
    player_3: PlayerState,
    player_4: PlayerState,
}

impl Players {
    pub fn new(p1: Option<UserId>, p2: Option<UserId>, starting_life: LifeValue) -> Self {
        Self {
            player_1: PlayerState::new(PlayerName::One, p1, starting_life),
            player_2: PlayerState::new(PlayerName::Two, p2, starting_life),
            player_3: PlayerState::new(PlayerName::Three, None, starting_life),
            player_4: PlayerState::new(PlayerName::Three, None, starting_life),
        }
    }
}

impl PlayerQueries for Players {
    fn player(&self, name: PlayerName) -> &PlayerState {
        match name {
            PlayerName::One => &self.player_1,
            PlayerName::Two => &self.player_2,
            PlayerName::Three => &self.player_3,
            PlayerName::Four => &self.player_4,
        }
    }

    fn player_mut(&mut self, name: PlayerName) -> &mut PlayerState {
        match name {
            PlayerName::One => &mut self.player_1,
            PlayerName::Two => &mut self.player_2,
            PlayerName::Three => &mut self.player_3,
            PlayerName::Four => &mut self.player_4,
        }
    }
}

/// Represents the state of a single player within a game
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlayerState {
    /// Name of this player
    pub name: PlayerName,

    /// Optionally, the ID of a user who is this player
    pub user_id: Option<UserId>,

    /// Entity ID for this player
    pub entity_id: EntityId,

    /// Configuration for this player
    pub options: PlayerOptions,

    /// Current amount of life for this player
    pub life: LifeValue,

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
    pub fn new(name: PlayerName, user_id: Option<UserId>, life: LifeValue) -> Self {
        Self {
            name,
            user_id,
            entity_id: name.entity_id(),
            options: PlayerOptions::default(),
            life,
            controller: name,
            counters: Counters::default(),
            mana_pool: ManaPool::default(),
            prompts: Default::default(),
            selected_cards: vec![],
        }
    }
}

impl HasEntityId for PlayerState {
    fn entity_id(&self) -> EntityId {
        self.entity_id
    }
}

impl HasPlayerName for PlayerState {
    fn player_name(&self) -> PlayerName {
        self.name
    }
}

impl HasController for PlayerState {
    fn controller(&self) -> PlayerName {
        self.controller
    }
}
