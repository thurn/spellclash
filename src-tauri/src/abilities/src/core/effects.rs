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

use data::card_states::zones::ZoneQueries;
use data::core::primitives::{CardId, EntityId, HasSource, PermanentId, SpellId};
use data::delegates::delegate_type::DelegateType;
use data::delegates::game_delegates::GameDelegates;
use data::delegates::query_value::QueryValue;
use data::delegates::scope::{EffectContext, Scope};
use data::game_states::game_state::GameState;
use data::queries::card_modifier::CardModifier;
use data::queries::duration::Duration;
use data::queries::query_name::QueryName;
use utils::outcome;
use utils::outcome::Outcome;

/// Applies a modifier to the [PermanentId] permanent while it is on the
/// battlefield.
pub fn modify_permanent<TQuery: QueryName>(
    game: &mut GameState,
    context: EffectContext,
    permanent_id: PermanentId,
    effect: TQuery::Modifier,
) {
    let turn = game.turn;
    if let Some(card) = game.card_mut(permanent_id) {
        let query = TQuery::query_mut(&mut card.queries);
        query.add(CardModifier {
            source: context.source(),
            duration: Duration::WhileOnBattlefield(permanent_id),
            delegate_type: DelegateType::Effect,
            effect,
        });
    }
}

/// Applies a modifier to the [PermanentId] permanent while it is on the
/// battlefield this turn.
pub fn modify_permanent_this_turn<TQuery: QueryName>(
    game: &mut GameState,
    context: EffectContext,
    permanent_id: PermanentId,
    effect: TQuery::Modifier,
) {
    let turn = game.turn;
    if let Some(card) = game.card_mut(permanent_id) {
        let query = TQuery::query_mut(&mut card.queries);
        query.add(CardModifier {
            source: context.source(),
            duration: Duration::WhileOnBattlefieldThisTurn(permanent_id, turn),
            delegate_type: DelegateType::Effect,
            effect,
        });
    }
}
