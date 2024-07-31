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

use std::collections::{BTreeMap, BTreeSet};

use enum_kinds::EnumKind;
use primitives::game_primitives::{PermanentId, PlayerName};
use serde::{Deserialize, Serialize};
/// Entity which has been declared as an attacker
pub type AttackerId = PermanentId;

/// Entity which has been declared as a blocker
pub type BlockerId = PermanentId;

/// Possible entities a creature may attack
#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash, Ord, PartialOrd, Serialize, Deserialize)]
pub enum AttackTarget {
    Player(PlayerName),
    Planeswalker(PlayerName, PermanentId),
    Battle(PlayerName, PermanentId),
}

impl AttackTarget {
    pub fn defending_player(&self) -> PlayerName {
        match self {
            Self::Player(player) => *player,
            Self::Planeswalker(player, _) => *player,
            Self::Battle(player, _) => *player,
        }
    }
}

/// Tracks the state of creatures participating in a combat phase
#[derive(Debug, Clone, EnumKind)]
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
            Self::OrderingBlockers(blockers) => Some(&blockers.attackers),
            Self::ConfirmedBlockers(blockers) => Some(&blockers.attackers),
        }
    }
}

/// Attacks the active player is considering
#[derive(Debug, Clone)]
pub struct ProposedAttackers {
    /// Creatures which have been selected to attack
    pub proposed_attacks: AttackerMap,

    /// Creatures the attacker is currently making attack decisions for
    pub selected_attackers: BTreeSet<AttackerId>,
}

/// Represents declared attacks within a combat phase
#[derive(Debug, Clone, Default)]
pub struct AttackerMap {
    /// Creatures which have been selected to attack
    attacks: BTreeMap<AttackerId, AttackTarget>,
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
    pub fn all_attackers(&self) -> impl Iterator<Item = AttackerId> + '_ {
        self.attacks.keys().copied()
    }

    pub fn all(&self) -> impl Iterator<Item = (&AttackerId, &AttackTarget)> + '_ {
        self.attacks.iter()
    }

    pub fn get_target(&self, attacker: AttackerId) -> Option<AttackTarget> {
        self.attacks.get(&attacker).copied()
    }

    /// Returns the number of declared attackers
    pub fn len(&self) -> usize {
        self.attacks.len()
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
#[derive(Debug, Clone)]
pub struct ProposedBlockers {
    /// The player who is proposing blockers
    pub defender: PlayerName,

    /// Attacking creatures
    pub attackers: AttackerMap,

    /// Creatures the defender is currently making block decisions for
    pub selected_blockers: BTreeSet<BlockerId>,

    /// Current proposed blocks, keyed by Blocker ID
    pub proposed_blocks: BTreeMap<BlockerId, Vec<AttackerId>>,
}

#[derive(Debug, Clone)]
pub struct BlockerMap {
    /// All declared attackers along with their attack targets
    pub attackers: AttackerMap,

    /// Attackers which have been blocked and will not deal combat damage to
    /// their target.
    ///
    /// A [BlockerId] will be retained in this map even if the blocking creature
    /// is subsequently removed.
    pub blocked_attackers: BTreeMap<AttackerId, Vec<BlockerId>>,

    /// Map from Blocker ID to the attackers that creature is blocking
    pub reverse_lookup: BTreeMap<BlockerId, Vec<AttackerId>>,
}
