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

use crate::core::primitives::{CardId, PlayerName};

pub type AttackerId = CardId;

pub type BlockerId = CardId;

/// Map from each attacker to the list of blockers for that creature, in blocker
/// order (the first creature in the vector will receive damage first).
pub type BlockerMap = HashMap<CombatAttacker, Vec<BlockerId>>;

/// Possible entities a creature may attack
#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub enum AttackTarget {
    Player(PlayerName),
    Planeswalker(CardId),
    Battle(CardId),
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub struct CombatAttacker {
    pub attacker: AttackerId,
    pub target: AttackTarget,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub struct CombatBlock {
    pub attacker: AttackerId,
    pub blocker: BlockerId,
}

/// Tracks the state of creatures participating in a combat phase
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum CombatCreatures {
    /// Attackers have not yet been declared.
    BeginningOfCombat,

    /// The active player is selecting attackers and has currently picked this
    /// set of creatures to attack the indicated [AttackTarget]s.
    ProposingAttackers(Vec<CombatAttacker>),

    /// The active player has confirmed this set of attackers, and it has been
    /// validated as a legal attack.
    ConfirmedAttackers(Vec<CombatAttacker>),

    /// The defending player is selecting blockers and has currently picked this
    /// set of blockers.
    ProposingBlockers {
        /// Attacking creatures
        attackers: Vec<CombatAttacker>,

        /// Creature the defender is currently making block decisions for
        active_blocker: Option<BlockerId>,

        /// Current proposed blocks
        proposed_blocks: Vec<CombatBlock>,
    },

    /// The defending player has confirmed this set of blockers, and it has
    /// been validated as a legal set of blocks.
    ConfirmedBlockers {
        /// Attacking creatures
        attackers: Vec<CombatAttacker>,

        /// Blocks
        blocks: Vec<CombatBlock>,
    },

    /// The attacking player is ordering blockers.
    ///
    /// The [BlockerMap] contains the currently-proposed blocker ordering.
    OrderingBlockers(BlockerMap),

    /// The attacking player has ordered blockers and is ready to proceed to
    /// combat damage.
    ConfirmedBlockerOrder(BlockerMap),
}

/// State of an ongoing combat step within a game
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CombatState {
    /// True if this combat state represents a currently active game phase.
    ///
    /// False if this is the empty combat state or represents a combat state
    /// which has been completed.
    pub currently_active: bool,

    /// Current state of declaring attackers & blockers for this combat
    pub creatures: CombatCreatures,
}
