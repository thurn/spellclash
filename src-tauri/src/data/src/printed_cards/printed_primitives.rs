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

use enumset::EnumSetType;
use serde::{Deserialize, Serialize};

use crate::core::numerics::{Loyalty, Power, Toughness};

/// Attraction lights printed on a card, used on certain Un-Set cards for the
/// 'attraction' mechanic.
#[derive(Debug, Serialize, Deserialize, EnumSetType)]
pub enum AttractionLight {
    One,
    Two,
    Three,
    Four,
    Fix,
    Six,
}

/// Printed loyalty value on a planeswalker card
#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
pub enum PrintedLoyalty {
    /// Number of starting loyalty counters
    Number(Loyalty),
    /// Starting loyalty is defined by rules text_strings
    X,
}

/// Printed power value on a card
#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
pub enum PrintedPower {
    /// Numeric power
    Number(Power),
    /// Power is defined by X in rules text_strings
    X,
    /// Power is defined by rules text_strings
    ///
    /// See <https://yawgatog.com/resources/magic-rules/#R2082>
    Star,
    /// Power is defined by rules text_strings plus one
    StarPlusOne,
}

/// Printed toughness value on a card
#[derive(Debug, Copy, Clone)]
pub enum PrintedToughness {
    /// Numeric toughness
    Number(Toughness),
    /// Toughness is defined by X in rules text_strings
    X,
    /// Toughness is defined by rules text_strings
    ///
    /// See <https://yawgatog.com/resources/magic-rules/#R2082>
    Star,
    /// Power is defined by rules text_strings plus one
    StarPlusOne,
}
