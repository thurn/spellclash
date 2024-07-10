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

use data::actions::game_action::GameAction;
use data::decks::deck_name;
use data::game_states::game_state::GameStatus;
use rand::prelude::SliceRandom;
use rand::SeedableRng;
use rand_xoshiro::SplitMix64;
use rules::action_handlers::actions;
use rules::action_handlers::actions::ExecuteAction;
use rules::legality::legal_actions;
use rules::legality::legal_actions::LegalActions;
use testing::ai_testing::test_games;

#[test]
pub fn game_is_deterministic() {
    let actions = run();
    for _ in 0..100 {
        assert_eq!(actions, run());
    }
}

fn run() -> Vec<GameAction> {
    let mut result = vec![];
    let mut game = test_games::create(deck_name::SOME_DANDANS);
    let mut rng = SplitMix64::seed_from_u64(156562599311216480);
    while !matches!(game.status, GameStatus::GameOver { .. }) {
        let player = legal_actions::next_to_act(&game, None).unwrap();
        let legal = legal_actions::compute(&game, player, LegalActions { for_human_player: false });
        let action = *legal.choose(&mut rng).unwrap();
        result.push(action);
        actions::execute(&mut game, player, action, ExecuteAction {
            skip_undo_tracking: true,
            validate: false,
        });
    }
    result
}
