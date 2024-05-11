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

use crate::actions::debug_action::DebugGameAction;
use crate::actions::user_action::UserAction;
use crate::core::primitives::CardId;
use crate::game_states::combat_state::{AttackTarget, AttackerId, BlockerId};

/// Actions within a combat phase
#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum CombatAction {
    /// Select a creature as the 'active attacker'
    ///
    /// When an active attacker is selected, the attacker can select a target to
    /// attack via [Self::AttackWithActiveAttacker].
    ///
    /// If there is only one legal attack target, this creature will
    /// automatically be added to the 'proposed attacks' set and calling
    /// [Self::AttackWithActiveAttacker] is not needed.
    SetActiveAttacker(AttackerId),

    /// Adds the active attacker (selected via [Self::SetActiveAttacker]) as an
    /// attacker of this [AttackTarget]. Once the set of proposed attackers is
    /// finalized, assuming the choice is legal, they can be confirmed via
    /// [Self::ConfirmAttackers].
    AttackWithActiveAttacker(AttackTarget),

    /// Remove a creature from the 'proposed attackers' set.
    ///
    /// If this creature is currently the active attacker it is also removed
    /// from that status.
    RemoveAttacker(AttackerId),

    /// Lock in the set of proposed attackers for the declare attackers step.
    ConfirmAttackers,

    /// Select a creature as the 'active blocker'.
    ///
    /// When an active blocker is selected, the defender can select which
    /// attacker is being blocked via [Self::BlockWithActiveBlocker].
    SetActiveBlocker(BlockerId),

    /// Adds the active blocker (selected via [Self::SetActiveBlocker]) to
    /// the 'proposed blockers' set, blocking the indicated creature.
    ///
    /// Once the set of proposed blockers is finalized, assuming the choice is
    /// legal, they can be confirmed via [Self::ConfirmBlockers].
    BlockWithActiveBlocker(AttackerId),

    /// Removes a creature from the 'proposed blockers' set.
    ///
    /// If this creature is currently the active blocker it is also removed from
    /// that status.
    RemoveBlocker(BlockerId),

    /// Lock in the blocking decisions for the declare blockers step.
    ConfirmBlockers,

    /// Move the indicated blocker the the provided index `position` within the
    /// blockers list.
    OrderBlocker { attacker_id: AttackerId, blocker_id: BlockerId, position: usize },

    /// Lock in block ordering decisions for the declare blockers step.
    ConfirmBlockerOrder,
}

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum GameAction {
    /// Debugging action.
    DebugAction(DebugGameAction),

    /// Pass priority on the current stack item or game step.
    ///
    /// > If all players pass in succession (that is, if all players pass
    /// > without taking any action_handlers in between passing), the spell or
    /// > ability
    /// > on top of the stack resolves or, if the stack is empty, the phase or
    /// > step ends.
    ///
    /// <https://yawgatog.com/resources/magic-rules/#R1174>
    PassPriority,

    /// Cast a spell or play a land.
    ///
    /// This includes playing cards from exile, the graveyard, the library, etc.
    /// The player will be prompted to make choices (which face to play, which
    /// targets to select, etc) before placing this item on the stack.
    ///
    /// It is an error to attempt to play a token, emblem, copy of a card on
    /// the stack, or an ability on the stack.
    ProposePlayingCard(CardId),

    /// Take an action within a combat phase
    CombatAction(CombatAction),
}

impl From<GameAction> for UserAction {
    fn from(value: GameAction) -> Self {
        UserAction::GameAction(value)
    }
}
