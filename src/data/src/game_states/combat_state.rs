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

use std::collections::{HashMap, HashSet};

use serde::{Deserialize, Serialize};

use crate::core::primitives::{CardId, EntityId, PlayerName};

/// Entity which has been declared as an attacker
pub type AttackerId = EntityId;

/// Entity which has been declared as a blocker
pub type BlockerId = EntityId;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CombatAttack {
    /// Entity being attacked
    pub target: AttackTarget,

    /// Blockers for this creature, in blocker order (the first creature in the
    /// vector will receive damage first).
    pub blockers: Vec<BlockerId>,
}

/// Map from each attacker to the attacker state
pub type BlockerMap = HashMap<AttackerId, CombatAttack>;

/// Possible entities a creature may attack
#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub enum AttackTarget {
    Player(PlayerName),
    Planeswalker(CardId),
    Battle(CardId),
}

/// Tracks the state of creatures participating in a combat phase
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum CombatState {
    /// The active player is selecting attackers and has currently picked this
    /// set of creatures to attack the indicated [AttackTarget]s.
    ProposingAttackers {
        /// Creatures which have been selected to attack
        proposed_attacks: HashMap<AttackerId, AttackTarget>,

        /// Creatures the attacker is currently making attack decisions for
        active_attackers: HashSet<AttackerId>,
    },

    /// The active player has confirmed this set of attackers, and it has been
    /// validated as a legal attack.
    ConfirmedAttackers { attackers: HashMap<AttackerId, AttackTarget> },

    /// The defending player is selecting blockers and has currently picked this
    /// set of blockers.
    ProposingBlockers {
        /// Attacking creatures
        attackers: HashMap<AttackerId, AttackTarget>,

        /// Creature the defender is currently making block decisions for
        active_blockers: HashSet<BlockerId>,

        /// Current proposed blocks, keyed by Blocker ID
        proposed_blocks: HashMap<BlockerId, AttackerId>,
    },

    /// The defending player has confirmed this set of blockers, and it has
    /// been validated as a legal set of blocks.
    ConfirmedBlockers {
        /// Attacking creatures
        attackers: HashMap<AttackerId, AttackTarget>,

        /// Blocks
        blocks: HashMap<BlockerId, AttackerId>,
    },

    /// The attacking player is ordering blockers.
    ///
    /// The [BlockerMap] contains the currently-proposed blocker ordering.
    OrderingBlockers { blockers: BlockerMap },

    /// The attacking player has ordered blockers and is ready to proceed to
    /// combat damage.
    ConfirmedBlockerOrder { blockers: BlockerMap },
}
