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

use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};

use crate::actions::game_action::GameAction;
use crate::core::primitives::{GameId, PlayerName};
use crate::decks::deck_name::DeckName;
use crate::game_states::game_state::DebugConfiguration;
use crate::game_states::history_data::TakenGameAction;
use crate::player_states::player_map::PlayerMap;
use crate::player_states::player_state::PlayerType;
use crate::prompts::prompt::PromptResponse;

/// Identifies the serialization format version
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SerializedGameVersion {
    Version1,
}

/// Serializes the state of a game by storing the sequence of game actions
/// taken.
///
/// State is reconstructed by applying game actions in order.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SerializedGameState {
    pub version: SerializedGameVersion,
    pub id: GameId,
    pub seed: u64,
    pub start_player: PlayerName,
    pub decks: PlayerMap<DeckName>,
    pub player_types: PlayerMap<PlayerType>,
    pub player_actions: PlayerMap<Vec<TakenGameAction>>,
    pub prompt_responses: PlayerMap<Vec<PromptResponse>>,
    pub debug_configuration: DebugConfiguration,
}
