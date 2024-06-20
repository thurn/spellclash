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

use std::marker::PhantomData;

use ai::core::state_evaluator::StateEvaluator;
use ai::core::win_loss_evaluator::WinLossEvaluator;
use ai::monte_carlo::monte_carlo_search::RandomPlayoutEvaluator;
use data::core::primitives::PlayerName;
use data::decks::deck_name;
use testing::ai_testing::test_games;

#[test]
fn all_dandans() {
    let evaluator =
        RandomPlayoutEvaluator { evaluator: WinLossEvaluator, phantom_data: PhantomData };
    let game = test_games::create(deck_name::ALL_DANDANS);
    evaluator.evaluate(&game, PlayerName::One);
}
