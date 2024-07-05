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

use data::card_definitions::ability_definition::TargetSelector;
use data::core::primitives::EntityId;
use data::delegates::scope::Scope;
use data::game_states::game_state::GameState;
use either::Either;

/// A target selector that selects from either of two other target selectors.
pub struct PairSelector<T: TargetSelector, U: TargetSelector> {
    pub first: T,
    pub second: U,
}

impl<T: TargetSelector, U: TargetSelector> TargetSelector for PairSelector<T, U> {
    type Target = Either<T::Target, U::Target>;

    fn valid_targets<'a>(
        &'a self,
        game: &'a GameState,
        scope: Scope,
    ) -> Box<dyn Iterator<Item = EntityId> + 'a> {
        Box::new(
            self.first.valid_targets(game, scope).chain(self.second.valid_targets(game, scope)),
        )
    }

    fn build_target_data(&self, game: &GameState, targets: &[EntityId]) -> Option<Self::Target> {
        if let Some(first) = self.first.build_target_data(game, targets) {
            return Some(Either::Left(first));
        }
        if let Some(second) = self.second.build_target_data(game, targets) {
            return Some(Either::Right(second));
        }
        None
    }
}
