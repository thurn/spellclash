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
}

impl From<GameAction> for UserAction {
    fn from(value: GameAction) -> Self {
        UserAction::GameAction(value)
    }
}
