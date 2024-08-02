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

#[allow(unused)] // Used in docs
use crate::card_states::card_state::CardState;

/// Possible kinds of game objects represented by the [CardState] struct.
#[derive(Debug, Hash, EnumSetType)]
pub enum CardKind {
    /// Standard magic card
    Normal,

    /// Token created on the battlefield by an effect or a copy of a card on the
    /// stack.
    TokenOrStackCopy,

    /// Emblem, usually created by a planeswalker
    Emblem,
}
