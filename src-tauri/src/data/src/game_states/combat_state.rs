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

use enum_kinds::EnumKind;
use serde::{Deserialize, Serialize};
use serde_with::serde_as;

use crate::core::primitives::{CardId, EntityId, PlayerName};

/// Entity which has been declared as an attacker
pub type AttackerId = EntityId;

/// Entity which has been declared as a blocker
pub type BlockerId = EntityId;

/// Possible entities a creature may attack
#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub enum AttackTarget {
    Player(PlayerName),
    Planeswalker(EntityId),
    Battle(EntityId),
}

/// Tracks the state of creatures participating in a combat phase
#[derive(Debug, Clone, Serialize, Deserialize, EnumKind)]
#[enum_kind(CombatStateKind)]
pub enum CombatState {
    /// The active player is selecting attackers and has currently picked this
    /// set of creatures to attack the indicated [AttackTarget]s.
    ProposingAttackers(ProposedAttackers),

    /// The active player has confirmed this set of attackers, and it has been
    /// validated as a legal attack.
    ConfirmedAttackers(AttackerMap),

    /// The defending player is selecting blockers and has currently picked this
    /// set of blockers.
    ProposingBlockers(ProposedBlockers),

    /// The defending player has confirmed this set of blockers, and it has
    /// been validated as legal. The attacking player is ordering blockers.
    ///
    /// The [BlockerMap] contains the currently-proposed blocker ordering.
    OrderingBlockers(BlockerMap),

    /// The attacking player has ordered blockers and is ready to proceed to
    /// combat damage.
    ConfirmedBlockers(BlockerMap),
}

impl CombatState {
    pub fn kind(&self) -> CombatStateKind {
        self.into()
    }

    /// Returns all confirmed attackers for this combat state, or None if
    /// attackers have not yet been confirmed.
    pub fn confirmed_attackers(&self) -> Option<&AttackerMap> {
        match self {
            Self::ProposingAttackers(_) => None,
            Self::ConfirmedAttackers(attackers) => Some(attackers),
            Self::ProposingBlockers(blockers) => Some(&blockers.attackers),
            Self::OrderingBlockers(blockers) => Some(&blockers.all_attackers),
            Self::ConfirmedBlockers(blockers) => Some(&blockers.all_attackers),
        }
    }
}

/// Attacks the active player is considering
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProposedAttackers {
    /// Creatures which have been selected to attack
    pub proposed_attacks: AttackerMap,

    /// Creatures the attacker is currently making attack decisions for
    pub selected_attackers: HashSet<AttackerId>,
}

/// Represents declared attacks within a combat phase
#[serde_as]
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct AttackerMap {
    /// Creatures which have been selected to attack
    #[serde_as(as = "Vec<(_, _)>")]
    attacks: HashMap<AttackerId, AttackTarget>,
}

impl AttackerMap {
    pub fn contains(&self, attacker: AttackerId) -> bool {
        self.attacks.contains_key(&attacker)
    }

    pub fn insert(&mut self, attacker: AttackerId, target: AttackTarget) {
        self.attacks.insert(attacker, target);
    }

    pub fn remove(&mut self, attacker: AttackerId) {
        self.attacks.remove(&attacker);
    }

    /// Iterator over all declared attackers
    pub fn all(&self) -> impl Iterator<Item = AttackerId> + '_ {
        self.attacks.keys().copied()
    }

    pub fn get_target(&self, attacker: AttackerId) -> Option<AttackTarget> {
        self.attacks.get(&attacker).copied()
    }

    /// Returns true if there are no declared attackers
    pub fn is_empty(&self) -> bool {
        self.attacks.is_empty()
    }

    /// If there is exactly one attacking creature, returns its [AttackerId].
    pub fn exactly_one(&self) -> Option<AttackerId> {
        if self.attacks.len() == 1 {
            self.attacks.keys().nth(0).copied()
        } else {
            None
        }
    }
}

/// Blocks the defending player is considering
#[serde_as]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProposedBlockers {
    /// The player who is proposing blockers
    pub defender: PlayerName,

    /// Attacking creatures
    pub attackers: AttackerMap,

    /// Creatures the defender is currently making block decisions for
    pub selected_blockers: HashSet<BlockerId>,

    /// Current proposed blocks, keyed by Blocker ID
    #[serde_as(as = "Vec<(_, _)>")]
    pub proposed_blocks: HashMap<BlockerId, AttackerId>,
}

#[serde_as]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BlockerMap {
    /// All declared attackers along with their attack targets
    pub all_attackers: AttackerMap,

    /// Attackers which have been blocked and will not deal combat damage to
    /// their target
    #[serde_as(as = "Vec<(_, _)>")]
    pub blocked_attackers: HashMap<AttackerId, Vec<BlockerId>>,

    /// Map from Blocker ID to the attacker that creature is blocking
    #[serde_as(as = "Vec<(_, _)>")]
    pub reverse_lookup: HashMap<BlockerId, AttackerId>,
}
