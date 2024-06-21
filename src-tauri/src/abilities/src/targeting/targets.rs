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

use data::card_definitions::ability_choices::CardAbilityTarget;
use data::core::primitives::{CardType, Zone};
use enumset::EnumSet;
use rules::queries::card_queries;

/// Target any creature on the battlefield
pub fn creature() -> CardAbilityTarget {
    CardAbilityTarget {
        zones: EnumSet::only(Zone::Battlefield),
        predicate: Box::new(|g, _, id| {
            card_queries::card_types(g, id).contains(CardType::Creature)
        }),
    }
}
