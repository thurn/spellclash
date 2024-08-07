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

use primitives::game_primitives::{
    CardId, EntityId, HasController, HasPlayerName, PlayerName, UserId,
};
use serde::{Deserialize, Serialize};

use crate::card_states::counters::Counters;
use crate::core::numerics::LifeValue;
use crate::decks::deck_name::DeckName;
use crate::player_states::game_agent::{GameAgent, GameAgentImpl, PromptAgentImpl};
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
#[derive(Clone)]
pub struct Players {
    player_1: PlayerState,
    player_2: PlayerState,
    player_3: PlayerState,
    player_4: PlayerState,
}

impl Players {
    pub fn new(player_1: PlayerState, player_2: PlayerState) -> Self {
        Self {
            player_1,
            player_2,
            player_3: PlayerState::new(PlayerName::Three, PlayerType::None, DeckName::default(), 0),
            player_4: PlayerState::new(PlayerName::Four, PlayerType::None, DeckName::default(), 0),
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

/// Possible types of players
#[derive(Clone, Serialize, Deserialize)]
pub enum PlayerType {
    Human(UserId),

    Agent(GameAgent),

    /// Player is not participating in this game
    None,
}

impl PlayerType {
    pub fn user_id(&self) -> Option<UserId> {
        match self {
            Self::Human(id) => Some(*id),
            _ => None,
        }
    }

    pub fn is_agent(&self) -> bool {
        matches!(self, Self::Agent(_))
    }
}

/// Represents the state of a single player within a game
#[derive(Clone)]
pub struct PlayerState {
    /// Name of this player
    pub name: PlayerName,

    /// Type of player this is
    pub player_type: PlayerType,

    /// Entity ID for this player
    pub entity_id: EntityId,

    /// Identifies this player's deck
    pub deck_name: DeckName,

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
    pub fn new(
        name: PlayerName,
        player_type: PlayerType,
        deck_name: DeckName,
        life: LifeValue,
    ) -> Self {
        Self {
            name,
            player_type,
            entity_id: name.entity_id(),
            deck_name,
            options: PlayerOptions::default(),
            life,
            controller: name,
            counters: Counters::default(),
            mana_pool: ManaPool::default(),
            prompts: Default::default(),
            selected_cards: vec![],
        }
    }

    pub fn agent(&self) -> Option<Box<dyn GameAgentImpl>> {
        match &self.player_type {
            PlayerType::Agent(agent) => agent.game_agent_reference.clone(),
            _ => None,
        }
    }

    pub fn prompt_agent(&self) -> Option<Box<dyn PromptAgentImpl>> {
        match &self.player_type {
            PlayerType::Agent(agent) => agent.prompt_agent_reference.clone(),
            _ => None,
        }
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
