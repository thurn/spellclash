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

use primitives::game_primitives::Source;
use crate::events::event_context::EventContext;
use crate::events::game_event::GameEvent;
use crate::game_states::game_state::GameState;

#[derive(Default, Clone, Debug)]
pub struct GlobalEvents {
    /// Invoked every time game state-triggered abilities are checked.
    pub state_triggered_ability: GameEvent<()>,
}

pub fn dispatch<TArg: 'static>(
    game: &mut GameState,
    event: fn(&GlobalEvents) -> &GameEvent<TArg>,
    source: Source,
    arg: TArg,
) {
    for i in 0..event(&game.events).callbacks.len() {
        if let Some(context) = event(&game.events).callbacks[i].build_context(game, source) {
            let function = event(&game.events).callbacks[i].function.clone();
            function.invoke(game, context, &arg);
        }
    }
}
