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

use data::core::primitives::Color;
use data::delegates::game_delegates::GameDelegates;
use data::delegates::layer::Layer;
use data::delegates::query_value::{EnumSets, QueryValue};
use data::printed_cards::card_subtypes::CreatureSubtype;
use enumset::EnumSet;
use rules::queries::query_extension::QueryExt;

/// Sets a card's creature subtypes for the current turn when affected by this
/// card.
pub fn for_target_this_turn(d: &mut GameDelegates, types: impl Into<EnumSet<CreatureSubtype>>) {
    let types = types.into();
    d.creature_subtypes
        .this_turn(move |_, c, _| EnumSets::set(Layer::TypeChangingEffects, c, types));
}
