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

use serde::{Deserialize, Serialize};

use primitives::game_primitives::PlayerName;

/// Data structure which holds information about each player in a game
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlayerMap<T> {
    pub one: T,
    pub two: T,
    pub three: T,
    pub four: T,
}

impl<T: Default> Default for PlayerMap<T> {
    fn default() -> Self {
        Self { one: T::default(), two: T::default(), three: T::default(), four: T::default() }
    }
}

impl<T> PlayerMap<T> {
    pub fn build_from<V>(value: &V, builder: impl Fn(&V, PlayerName) -> T) -> Self {
        Self {
            one: builder(value, PlayerName::One),
            two: builder(value, PlayerName::Two),
            three: builder(value, PlayerName::Three),
            four: builder(value, PlayerName::Four),
        }
    }

    pub fn get(&self, player: PlayerName) -> &T {
        match player {
            PlayerName::One => &self.one,
            PlayerName::Two => &self.two,
            PlayerName::Three => &self.three,
            PlayerName::Four => &self.four,
        }
    }

    pub fn get_mut(&mut self, player: PlayerName) -> &mut T {
        match player {
            PlayerName::One => &mut self.one,
            PlayerName::Two => &mut self.two,
            PlayerName::Three => &mut self.three,
            PlayerName::Four => &mut self.four,
        }
    }

    pub fn values(&self) -> impl Iterator<Item = (PlayerName, &T)> + '_ {
        vec![
            (PlayerName::One, &self.one),
            (PlayerName::Two, &self.two),
            (PlayerName::Three, &self.three),
            (PlayerName::Four, &self.four),
        ]
        .into_iter()
    }
}
