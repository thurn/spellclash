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

use data::actions::new_game_action::NewGameAction;
use data::card_states::zones::Zones;
use data::core::primitives::{GameId, PlayerName};
use data::delegates::game_delegates::GameDelegates;
use data::game_states::animation_tracker::AnimationTracker;
use data::game_states::combat_state::CombatState;
use data::game_states::game_state::{GameConfiguration, GameState, GameStatus, TurnData};
use data::game_states::game_step::GamePhaseStep;
use data::game_states::history_data::GameHistory;
use data::game_states::undo_state::UndoTracker;
use data::player_states::player_state::Players;
use data::state_machines::state_machine_data::StateMachines;
use rand_xoshiro::rand_core::SeedableRng;
use rand_xoshiro::Xoshiro256StarStar;
use uuid::Uuid;

pub fn _create(_: NewGameAction) -> GameState {
    GameState {
        id: GameId(Uuid::new_v4()),
        status: GameStatus::Setup,
        step: GamePhaseStep::Untap,
        current_turn: TurnData { turn: PlayerName::One, turn_number: 0 },
        configuration: GameConfiguration::default(),
        state_machines: StateMachines::default(),
        players: Players::default(),
        zones: Zones::default(),
        combat: CombatState::default(),
        animations: AnimationTracker::default(),
        history: GameHistory::default(),
        rng: Xoshiro256StarStar::seed_from_u64(314159265358979323),
        undo_tracker: UndoTracker::default(),
        delegates: GameDelegates::default(),
    }
}
