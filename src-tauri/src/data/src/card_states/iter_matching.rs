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

use std::collections::HashSet;

use crate::card_states::zones::ToCardId;
use crate::core::function_types::{CardPredicate, ScopedCardPredicate};
use crate::delegates::scope::Scope;
use crate::game_states::game_state::GameState;

pub trait IterMatching<TId: ToCardId, TFn: CardPredicate<TId>> {
    fn iter_matching<'a>(
        &'a self,
        game: &'a GameState,
        predicate: TFn,
    ) -> impl Iterator<Item = TId> + 'a;
}

impl<TId: ToCardId, TFn: CardPredicate<TId>> IterMatching<TId, TFn> for HashSet<TId> {
    fn iter_matching<'a>(
        &'a self,
        game: &'a GameState,
        predicate: TFn,
    ) -> impl Iterator<Item = TId> + 'a {
        self.iter().filter(move |&&id| predicate(game, id) == Some(true)).copied()
    }
}

pub trait IterMatchingScoped<
    TId: ToCardId,
    TScope: Scope + 'static,
    TFn: ScopedCardPredicate<TId, TScope>,
>
{
    fn iter_matching_scoped<'a>(
        &'a self,
        game: &'a GameState,
        scope: TScope,
        predicate: TFn,
    ) -> impl Iterator<Item = TId> + 'a;
}

impl<TId: ToCardId, TScope: Scope + 'static, TFn: ScopedCardPredicate<TId, TScope>>
    IterMatchingScoped<TId, TScope, TFn> for HashSet<TId>
{
    fn iter_matching_scoped<'a>(
        &'a self,
        game: &'a GameState,
        scope: TScope,
        predicate: TFn,
    ) -> impl Iterator<Item = TId> + 'a {
        self.iter().filter(move |&&id| predicate(game, scope, id) == Some(true)).copied()
    }
}
