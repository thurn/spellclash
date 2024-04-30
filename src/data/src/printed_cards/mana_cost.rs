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

use crate::core::numerics::ManaValue;
use crate::core::primitives::Color;

/// Represents the printed mana cost of a card or ability
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ManaCost {
    /// List of symbols making up this mana cost
    pub costs: Vec<ManaCostItem>,
}

/// A single symbol within a mana cost
///
/// See <https://yawgatog.com/resources/magic-rules/#R1074> for a list of possible symbols.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ManaCostItem {
    Generic(ManaValue),
    Colorless,
    Colored(Color),
    Hybrid(Color, Color),
    MonoHybrid(Color),
    VariableX,
    Snow(Color),
    Phyrexian(Color),
    PhyrexianHybrid(Color, Color),
}
