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

use data::card_states::zones::ToCardId;
use data::core::primitives::PermanentId;
use data::delegates::card_delegate_list::CardDelegateList;
use data::delegates::flag::Flag;
use data::delegates::scope::Scope;
use data::game_states::game_state::GameState;
use rules::queries::query_extension::QueryExt;

#[allow(unused)] // Used in docs
use crate::core::effects;

/// Configuration for cards gaining new abilities
pub enum GainAbility {
    /// Adds an ability to the card which owns this delegate.
    ThisCard,

    /// Adds an ability to any cards which have been marked as being affected by
    /// this delegate for a turn by a call to [effects::apply_this_turn].
    ForEffectThisTurn,
}

pub fn add_to_query<TArg, TResult, TFn>(
    query_delegate: &mut CardDelegateList<GameState, TArg, TResult>,
    add_ability: GainAbility,
    transformation: TFn,
) where
    TArg: ToCardId,
    TFn: Fn(&GameState, Scope, &TArg, TResult) -> TResult + Copy + Send + Sync + 'static,
{
    match add_ability {
        GainAbility::ThisCard => query_delegate.this(transformation),
        GainAbility::ForEffectThisTurn => query_delegate.this_turn(transformation),
    }
}
