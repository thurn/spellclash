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

use data::card_definitions::ability_definition::{Ability, StaticAbility};
use data::card_definitions::registry::{QueryFn, Registry};
use data::card_states::zones::ZoneQueries;
use data::core::card_tags::CardTag;
use data::core::primitives::{HasSource, PermanentId, Source};
use data::delegates::delegate_type::DelegateType;
use data::delegates::game_delegate_data::CanBeBlocked;
use data::delegates::layer::Layer;
use data::delegates::query_value::{EnumSets, QueryValue};
use data::events::event_context::EventContext;
use data::game_states::game_state::GameState;
use data::printed_cards::card_subtypes::CreatureType;
use data::properties::card_modifier::CardModifier;
use data::properties::duration::Duration;
use data::properties::flag::Flag;
use enumset::EnumSet;
use rules::queries::combat_queries;
use utils::outcome::Outcome;

/// > 702.9a. Flying is an evasion ability.
///
/// > 702.9b. A creature with flying can't be blocked except by creatures with
/// > flying and/or reach. A creature with flying can block a creature with or
/// > without flying. (See rule 509, "Declare Blockers Step," and rule 702.17,
/// > "Reach.")
///
/// > 702.9c. Multiple instances of flying on the same creature are redundant.
///
/// <https://yawgatog.com/resources/magic-rules/#R7029>
pub fn gain_this_turn(game: &mut GameState, context: EventContext, id: PermanentId) -> Outcome {
    let turn = game.turn;
    game.card_mut(id)?.properties.tags.add_effect(
        context,
        Duration::WhileOnBattlefieldThisTurn(id, turn),
        EnumSets::add(Layer::AbilityModifyingEffects, context, CardTag::Flying),
    );
    game.card_mut(id)?.properties.can_be_blocked.add_effect(
        context,
        Duration::WhileOnBattlefieldThisTurn(id, turn),
        Flag::and(move |g, s, data: &CanBeBlocked| {
            Some(
                g.card(data.blocker_id)?
                    .properties
                    .tags
                    .query(g, s, EnumSet::empty())
                    .contains(CardTag::Flying),
            )
        }),
    )
}
