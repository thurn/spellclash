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

use data::card_definitions::ability_definition::TargetSelector;
use either::Either;
use enumset::EnumSet;
use primitives::game_primitives::{CardType, PermanentId, SpellId};
use rules::predicates::card_predicates;

use crate::targeting::pair_selector::PairSelector;
use crate::targeting::permanent_selectors::SinglePermanentSelector;
use crate::targeting::player_set::PlayerSet;
use crate::targeting::spell_selectors::SingleSpellSelector;

/// Target any creature on the battlefield
pub fn creature() -> impl TargetSelector<Target = PermanentId> {
    SinglePermanentSelector::new(PlayerSet::AllPlayers, card_predicates::creature)
}

/// Target a creature an opponent controls
pub fn creature_opponent_controls() -> impl TargetSelector<Target = PermanentId> {
    SinglePermanentSelector::new(PlayerSet::Opponents, card_predicates::creature)
}

/// Target a creature you control
pub fn creature_you_control() -> impl TargetSelector<Target = PermanentId> {
    SinglePermanentSelector::new(PlayerSet::You, card_predicates::creature)
}

/// Target any permanent on the battlefield
pub fn permanent() -> impl TargetSelector<Target = PermanentId> {
    SinglePermanentSelector::new(PlayerSet::AllPlayers, card_predicates::always_true)
}

/// Target any spell on the stack.
pub fn spell() -> impl TargetSelector<Target = SpellId> {
    SingleSpellSelector::new(PlayerSet::AllPlayers, card_predicates::always_true)
}

/// Target any spell or permanent
pub fn spell_or_permanent() -> impl TargetSelector<Target = Either<SpellId, PermanentId>> {
    PairSelector { first: spell(), second: permanent() }
}

/// Target any spell on the stack with any of the given [CardType]s.
pub fn spell_with_type(types: EnumSet<CardType>) -> impl TargetSelector<Target = SpellId> {
    SingleSpellSelector::new(PlayerSet::AllPlayers, card_predicates::has_any_types_in(types))
}
