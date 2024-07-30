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

use primitives::game_primitives::{HasObjectId, PermanentId, SpellId, Zone};
use crate::card_states::zones::ZoneQueries;
use crate::game_states::game_state::{GameState, TurnData};

/// Controls how long an effect should apply to the game.
#[derive(Clone)]
pub enum Duration {
    /// Effect applies until the end of the game
    Continuous,

    /// Effect applies while the [PermanentId] permanent is on the battlefield.
    WhileOnBattlefield(PermanentId),

    /// Effect applies while the [SpellId] spell is on the stack.
    WhileOnStack(SpellId),

    /// Effect applies while the [SpellId] spell is on the stack or after it has
    /// resolved and is on the battlefield.
    WhileOnStackOrBattlefield(SpellId),

    /// Effect applies while the [PermanentId] permanent is on the battlefield
    /// during the [TurnData] turn.
    WhileOnBattlefieldThisTurn(PermanentId, TurnData),
}

impl Duration {
    /// Returns true if the effects for this duration should apply in the
    /// provided [GameState].
    pub fn is_active(&self, game: &GameState) -> bool {
        self.is_active_helper(game) == Some(true)
    }

    fn is_active_helper(&self, game: &GameState) -> Option<bool> {
        Some(match self {
            Duration::Continuous => true,
            Duration::WhileOnBattlefield(permanent_id) => game.has_card(*permanent_id),
            Duration::WhileOnStack(spell_id) => game.has_card(*spell_id),
            Duration::WhileOnStackOrBattlefield(spell_id) => {
                game.has_card(*spell_id)
                    || (game.card(*spell_id)?.zone == Zone::Battlefield
                        && game.card(*spell_id)?.previous_object_id == Some(spell_id.object_id()))
            }
            Duration::WhileOnBattlefieldThisTurn(permanent_id, turn) => {
                game.turn == *turn && game.has_card(*permanent_id)
            }
        })
    }
}
