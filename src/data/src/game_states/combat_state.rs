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

use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::core::primitives::CardId;

pub type AttackerId = CardId;
pub type BlockerId = CardId;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProposedBlocker {
    pub attacker: CardId,
    pub blocker: CardId,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum CombatAttackers {
    ProposingAttackers(Vec<CardId>),
    ConfirmedAttackers(Vec<CardId>),
    ProposingBlockers { active_blocker: Option<CardId>, proposed_blockers: Vec<ProposedBlocker> },
    ConfirmedBlockers(Vec<ProposedBlocker>),
    OrderedBlockers(HashMap<CardId, Vec<CardId>>),
}

/// State of an ongoing combat step within a game
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct CombatState {
    /// True if this combat state represents a currently active game phase.
    /// False if this is the empty combat state or represents a combat state
    /// which has been completed.
    pub currently_active: bool,

    pub attackers: Vec<CardId>,
}
