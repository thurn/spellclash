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

use data::card_states::play_card_plan::{ManaPaymentPlan, PlayCardPlan};
use data::card_states::zones::ZoneQueries;
use data::game_states::game_state::GameState;
use data::printed_cards::card_subtypes::LandType;
use data::printed_cards::mana_cost::ManaCostItem;
use primitives::game_primitives::{CardId, HasController, ManaColor, PermanentId, Source};
use tracing::instrument;
use utils::outcome;
use utils::outcome::Outcome;

use crate::queries::card_queries;

// Map from mana colors to lists of card id which can produce that color of mana
// and number of land subtypes that card has.
type SubtypeCount = usize;
type LandAbilityMap = BTreeMap<ManaColor, Vec<(PermanentId, SubtypeCount)>>;

/// Builds a plan for paying a spell's mana costs.
///
/// The spell payment planner takes a card which the controller would like to
/// cast. It returns a suggested way in which the card's mana costs could be
/// paid. A [PlayCardPlan] is provided to describe choices the user made
/// while putting this spell on the stack.
///
/// None is returned if the planner failed to find a legal combination of
/// choices which would result in this card's mana cost being paid.
#[instrument(level = "trace", skip_all)]
pub fn mana_payment(
    game: &GameState,
    _source: Source,
    card_id: CardId,
    plan: &PlayCardPlan,
) -> Option<ManaPaymentPlan> {
    let controller = game.card(card_id)?.controller();

    let mut lands: LandAbilityMap = BTreeMap::new();
    for card in game.battlefield(controller) {
        add_land_to_map(game, *card, &mut lands, ManaColor::White, LandType::Plains);
        add_land_to_map(game, *card, &mut lands, ManaColor::Blue, LandType::Island);
        add_land_to_map(game, *card, &mut lands, ManaColor::Black, LandType::Swamp);
        add_land_to_map(game, *card, &mut lands, ManaColor::Red, LandType::Mountain);
        add_land_to_map(game, *card, &mut lands, ManaColor::Green, LandType::Forest);
    }
    lands.values_mut().for_each(|v| v.sort_by_key(|(_, subtypes)| *subtypes));

    let cost = card_queries::mana_cost_for_casting_card(game, card_id, plan)?;
    let mut result = ManaPaymentPlan::default();
    for item in cost.items {
        add_land_for_item(&mut result, &mut lands, item)?;
    }

    Some(result)
}

fn add_land_to_map(
    game: &GameState,
    land_id: PermanentId,
    lands: &mut LandAbilityMap,
    color: ManaColor,
    subtype: LandType,
) -> Outcome {
    if game.card(land_id)?.tapped_state.is_tapped() {
        return outcome::OK;
    }
    let subtypes = card_queries::land_subtypes(game, Source::Game, land_id)?;
    if subtypes.contains(subtype) {
        lands.entry(color).or_default().push((land_id, subtypes.len()));
    }
    outcome::OK
}

fn add_land_for_item(
    result: &mut ManaPaymentPlan,
    lands: &mut LandAbilityMap,
    item: ManaCostItem,
) -> Option<()> {
    match item {
        ManaCostItem::Colored(color) => {
            // We prioritize tapping lands with fewer subtypes first.
            return if let Some((land, _)) = lands.get_mut(&color).and_then(|v| v.pop()) {
                result.basic_land_abilities_to_activate.push(land);
                Some(())
            } else {
                None
            };
        }
        ManaCostItem::Generic => {
            let mut counts =
                lands.iter().map(|(color, vec)| (vec.len(), *color)).collect::<Vec<_>>();
            // We prioritize land types which the player controls more of
            // first.
            counts.sort_by(|a, b| b.cmp(a));
            for (_, color) in counts {
                if let Some((land, _)) = lands.get_mut(&color).and_then(|v| v.pop()) {
                    result.basic_land_abilities_to_activate.push(land);
                    return Some(());
                }
            }
            None
        }
        _ => None,
    }
}
