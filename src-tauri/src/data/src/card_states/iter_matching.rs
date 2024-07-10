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

use std::collections::BTreeSet;

use crate::card_states::zones::ToCardId;
use crate::core::function_types::CardPredicate;
use crate::core::primitives::{HasSource, Source};
use crate::delegates::scope::Scope;
use crate::game_states::game_state::GameState;

pub trait IterMatching<TId: ToCardId, TFn: CardPredicate<TId>> {
    fn iter_matching<'a>(
        &'a self,
        game: &'a GameState,
        source: impl HasSource + 'a,
        predicate: TFn,
    ) -> impl Iterator<Item = TId> + 'a;

    fn any_matching<'a>(
        &'a self,
        game: &'a GameState,
        source: impl HasSource,
        predicate: TFn,
    ) -> bool {
        self.iter_matching(game, source, predicate).next().is_some()
    }
}

impl<TId: ToCardId, TFn: CardPredicate<TId>> IterMatching<TId, TFn> for BTreeSet<TId> {
    fn iter_matching<'a>(
        &'a self,
        game: &'a GameState,
        source: impl HasSource + 'a,
        predicate: TFn,
    ) -> impl Iterator<Item = TId> + 'a {
        self.iter().filter(move |&&id| predicate(game, source.source(), id) == Some(true)).copied()
    }
}
