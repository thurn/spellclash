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

use data::card_definitions::ability_choices::{AbilityTargetPermanent, PlayerSet};
use data::card_states::zones::ZoneQueries;
use data::core::primitives::{CardType, HasController, Zone, ALL_POSSIBLE_PLAYERS};
use data::delegates::scope::DelegateScope;
use data::game_states::game_state::GameState;
use enumset::EnumSet;
use rules::predicates::card_predicates;
use rules::queries::{card_queries, player_queries};

/// Target any creature on the battlefield
pub fn creature() -> AbilityTargetPermanent {
    AbilityTargetPermanent {
        players: PlayerSet::AllPlayers,
        predicate: Box::new(card_predicates::creature),
    }
}

/// Target a creature an opponent controls
pub fn creature_opponent_controls() -> AbilityTargetPermanent {
    AbilityTargetPermanent {
        players: PlayerSet::Opponents,
        predicate: Box::new(card_predicates::creature),
    }
}
