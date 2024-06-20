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

use std::sync::atomic::{AtomicBool, Ordering};

use abilities::predicates::card_predicates;
use abilities::restrictions::attack_restrictions;
use data::card_definitions::ability_definition::StaticAbility;
use data::card_definitions::card_definition::CardDefinition;
use data::card_definitions::card_name;
use data::card_states::zones::ZoneQueries;
use data::core::primitives::{HasCardId, HasSource, Zone};
use rules::mutations::move_card;
use utils::outcome;

static RUNNING: AtomicBool = AtomicBool::new(false);
pub fn dandan() -> CardDefinition {
    CardDefinition::new(card_name::DANDAN).ability(StaticAbility::new().delegate(
        Zone::Battlefield,
        |d| {
            attack_restrictions::cannot_attack_unless_defender_controls(d, card_predicates::island);
            d.state_triggered_abilities.any(|g, s, _| {
                if !g.battlefield(s.controller).iter().any(|&id| card_predicates::island(g, s, id))
                    && !RUNNING.load(Ordering::Relaxed)
                {
                    RUNNING.store(true, Ordering::Relaxed);
                    move_card::run(g, s.source(), s.card_id(), Zone::Graveyard)?;
                }
                outcome::OK
            });
        },
    ))
}
