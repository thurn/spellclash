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

use std::collections::HashMap;

use data::card_states::cast_spell_plan::{CastSpellChoices, ManaPaymentPlan};
use data::card_states::zones::ZoneQueries;
use data::core::primitives::{CardId, ManaColor, Source};
use data::game_states::game_state::GameState;
use data::printed_cards::card_subtypes::LandSubtype;
use data::printed_cards::mana_cost::ManaCostItem;
use utils::outcome::{Outcome, Value};
use utils::{fail, outcome};

use crate::queries::card_queries;

// Map from mana colors to lists of card id which can produce that color of mana
// and number of land subtypes that card has.
type LandAbilityMap = HashMap<ManaColor, Vec<(CardId, usize)>>;

/// Builds a plan for paying a spell's mana costs.
///
/// The spell payment planner takes a card which the controller would like to
/// cast. It returns a suggested way in which the card's mana costs could be
/// paid. A [CastSpellChoices] is provided to describe choices the user made
/// while putting this spell on the stack.
///
/// An error is returned if the planner failed to find a legal combination of
/// choices which would result in this card's mana cost being paid.
pub fn mana_payment(
    game: &GameState,
    _source: Source,
    card_id: CardId,
    choices: CastSpellChoices,
) -> Value<ManaPaymentPlan> {
    let controller = game.card(card_id).controller;

    let mut lands: LandAbilityMap = HashMap::new();
    for card in game.battlefield(controller) {
        add_land_to_map(game, *card, &mut lands, ManaColor::White, LandSubtype::Plains);
        add_land_to_map(game, *card, &mut lands, ManaColor::Blue, LandSubtype::Island);
        add_land_to_map(game, *card, &mut lands, ManaColor::Black, LandSubtype::Swamp);
        add_land_to_map(game, *card, &mut lands, ManaColor::Red, LandSubtype::Mountain);
        add_land_to_map(game, *card, &mut lands, ManaColor::Green, LandSubtype::Forest);
    }
    lands.values_mut().for_each(|v| v.sort_by_key(|(_, subtypes)| *subtypes));

    let cost = card_queries::mana_cost_for_casting_card(game, card_id, choices)?;
    let mut result = ManaPaymentPlan::default();
    for item in cost.items {
        add_land_for_item(&mut result, &mut lands, item)?;
    }

    Ok(result)
}

fn add_land_to_map(
    game: &GameState,
    card_id: CardId,
    lands: &mut HashMap<ManaColor, Vec<(CardId, usize)>>,
    color: ManaColor,
    subtype: LandSubtype,
) {
    let subtypes = card_queries::land_subtypes_for_face(game, card_id);
    if subtypes.contains(subtype) {
        lands.entry(color).or_default().push((card_id, subtypes.len()));
    }
}

fn add_land_for_item(
    result: &mut ManaPaymentPlan,
    lands: &mut LandAbilityMap,
    item: ManaCostItem,
) -> Outcome {
    match item {
        ManaCostItem::Colored(color) => {
            // We prioritize tapping lands with fewer subtypes first.
            if let Some((land, _)) = lands.get_mut(&color).and_then(|v| v.pop()) {
                result.basic_land_abilities.push(land);
                outcome::OK
            } else {
                fail!("No land available to produce {color:?}");
            }
        }
        ManaCostItem::Generic => {
            let mut counts =
                lands.iter().map(|(color, vec)| (vec.len(), *color)).collect::<Vec<_>>();
            // We prioritize land types which the player controls more of
            // first.
            counts.sort_by(|a, b| b.cmp(a));
            for (_, color) in counts {
                if let Some((land, _)) = lands.get_mut(&color).and_then(|v| v.pop()) {
                    result.basic_land_abilities.push(land);
                    return outcome::OK;
                }
            }
            fail!("No land available to produce generic mana");
        }
        _ => {
            fail!("Not implemented");
        }
    }
}
