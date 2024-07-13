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

use std::collections::BTreeMap;
use std::num::NonZeroU64;

use dashmap::DashSet;
use once_cell::sync::Lazy;

use crate::card_definitions::card_definition::CardDefinition;
use crate::card_definitions::card_name::CardName;

pub type CardFn = fn() -> CardDefinition;

pub static DEFINITIONS: Lazy<DashSet<(u64, CardFn)>> = Lazy::new(DashSet::new);

/// Contains [CardDefinition]s for all known cards, keyed by [CardName]
static CARDS: Lazy<BTreeMap<CardName, CardDefinition>> = Lazy::new(|| {
    let mut map = BTreeMap::new();
    let mut functions = DEFINITIONS.clone().into_iter().collect::<Vec<_>>();
    functions.sort_by_key(|(id, _)| *id);
    for (_, card_fn) in functions {
        let card = card_fn();
        assert!(!map.contains_key(&card.card_name()), "Duplicate card name found");
        map.insert(card.card_name(), card);
    }
    map
});

/// Returns an iterator over all known card definitions in an undefined order
pub fn all_cards() -> impl Iterator<Item = &'static CardDefinition> {
    assert!(CARDS.len() > 0, "Cards not found. Call card_list::initialize() first.");
    CARDS.values()
}

/// Looks up the definition for a [CardName].
///
/// Panics if no such card is defined. If this panics, you are probably not
/// calling initialize::run();
pub fn get(name: CardName) -> &'static CardDefinition {
    CARDS
        .get(&name)
        .unwrap_or_else(|| panic!("Card {name:?} not found. Call card_list::initialize() first."))
}
