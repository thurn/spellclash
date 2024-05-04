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

use crate::actions::user_action::UserAction;

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum GameAction {
    /// Pass priority on the current stack item or game step.
    ///
    /// > If all players pass in succession (that is, if all players pass
    /// > without taking any actions in between passing), the spell or ability
    /// > on top of the stack resolves or, if the stack is empty, the phase or
    /// > step ends.
    ///
    /// <https://yawgatog.com/resources/magic-rules/#R1174>
    PassPriority,
}

impl From<GameAction> for UserAction {
    fn from(value: GameAction) -> Self {
        UserAction::GameAction(value)
    }
}
