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

use crate::core::primitives::ManaColor;

/// Represents the printed mana cost of a card or ability
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ManaCost {
    /// List of symbols making up this mana cost
    pub items: Vec<ManaCostItem>,
}

/// A single symbol within a mana cost.
///
/// Ordered by approximately how difficult this cost is to pay in a typical game
/// state.
///
/// See <https://yawgatog.com/resources/magic-rules/#R1074> for a list of possible symbols.
#[derive(Debug, Clone, Copy, Ord, PartialOrd, Eq, PartialEq, Serialize, Deserialize)]
pub enum ManaCostItem {
    Snow(ManaColor),
    Colored(ManaColor),
    Hybrid(ManaColor, ManaColor),
    MonoHybrid(ManaColor),
    Phyrexian(ManaColor),
    PhyrexianHybrid(ManaColor, ManaColor),
    VariableX,
    /// One generic mana
    Generic,
}
