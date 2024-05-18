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

use derive_more::{
    Add, AddAssign, Display, Div, DivAssign, From, Into, Mul, MulAssign, Sub, SubAssign, Sum,
};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

pub type TurnNumber = u64;
pub type ManaValue = u64;
pub type LifeValue = i64;
pub type Loyalty = u64;
pub type Power = i64;
pub type Toughness = i64;
pub type Damage = u64;

/// Converts a [Power] value to a [Damage] value.
pub fn power_to_damage(power: Power) -> Damage {
    power.max(0) as Damage
}
