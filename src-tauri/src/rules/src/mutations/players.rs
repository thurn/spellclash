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

use data::core::numerics::Damage;
use data::core::primitives::{PlayerName, Source};
use data::game_states::game_state::GameState;
use data::game_states::state_based_event::StateBasedEvent;
use data::player_states::player_state::PlayerQueries;
use tracing::debug;
use utils::outcome;
use utils::outcome::Outcome;

pub fn deal_damage(
    game: &mut GameState,
    _source: Source,
    player: PlayerName,
    damage: Damage,
) -> Outcome {
    debug!("Dealing {damage:?} damage to {player:?}");
    game.player_mut(player).life -= damage as i64;
    game.add_state_based_event(StateBasedEvent::LifeTotalDecrease(player));
    outcome::OK
}
