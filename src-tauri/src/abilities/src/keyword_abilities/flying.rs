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
use data::card_states::zones::ZoneQueries;
use data::core::card_tags::CardTag;
use data::core::layer::Layer;
use data::core::modifier_data::ModifierMode;
use data::events::event_context::EventContext;
use data::game_states::game_state::GameState;
use data::properties::card_properties::CardProperties;
use data::properties::card_property_data::CanBeBlocked;
use data::properties::duration::Duration;
use data::properties::flag::Flag;
use data::properties::property_value::EnumSets;
use enumset::EnumSet;
use primitives::game_primitives::{
    HasSource, PermanentId, Source, Timestamp, PRINTED_TEXT_TIMESTAMP,
};
use utils::outcome;
use utils::outcome::Outcome;

/// The Flying ability.
///
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
pub fn ability() -> impl Ability {
    StaticAbility::new().properties(|scope, properties| {
        gain(ModifierMode::PrintedAbility(scope), properties);
    })
}

/// Causes the [PermanentId] permanent to gain flying until the end of the turn.
pub fn gain_this_turn(game: &mut GameState, context: EventContext, id: PermanentId) -> Outcome {
    gain(ModifierMode::add_ability_this_turn(context, id), &mut game.card_mut(id)?.properties)
}

fn gain(mode: ModifierMode, properties: &mut CardProperties) -> Outcome {
    properties.tags.add_with_mode(mode, EnumSets::add_with_mode(mode, CardTag::Flying));
    properties.can_be_blocked.add_with_mode(
        mode,
        Flag::and(move |g, s, data: &CanBeBlocked| {
            g.card(data.blocker_id)?.has_tag(g, s, CardTag::Flying)
        }),
    )
}
