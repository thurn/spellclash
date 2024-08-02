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
use data::core::modifier_data::ModifierMode;
use data::delegates::layer::Layer;
use data::delegates::query_value::EnumSets;
use data::events::event_context::EventContext;
use data::game_states::game_state::GameState;
use data::properties::card_properties::CardProperties;
use data::properties::duration::Duration;
use data::properties::flag::Flag;
use primitives::game_primitives::{HasSource, PermanentId};
use utils::outcome::Outcome;

/// The Haste ability.
///
/// > 702.10a. Haste is a static ability.
///
/// > 702.10b. If a creature has haste, it can attack even if it hasn't been
/// > controlled by its controller continuously since their most recent turn
/// > began. (See rule 302.6.)
///
/// > 702.10c. If a creature has haste, its controller can activate its
/// > activated abilities whose cost includes the tap symbol or the untap symbol
/// > even if that creature hasn't been controlled by that player continuously
/// > since their most recent turn began. (See rule 302.6.)
///
/// > 702.10d. Multiple instances of haste on the same creature are redundant.
///
/// <https://yawgatog.com/resources/magic-rules/#R70210>
pub fn ability() -> impl Ability {
    StaticAbility::new().properties(|scope, properties| {
        gain(ModifierMode::PrintedAbility(scope), properties);
    })
}

/// Causes the [PermanentId] permanent to gain haste until the end of the turn.
pub fn gain_this_turn(game: &mut GameState, context: EventContext, id: PermanentId) -> Outcome {
    gain(ModifierMode::add_ability_this_turn(context, id), &mut game.card_mut(id)?.properties)
}

fn gain(mode: ModifierMode, properties: &mut CardProperties) -> Outcome {
    properties.tags.add_with_mode(mode, EnumSets::add_with_mode(mode, CardTag::Haste));
    properties.has_haste.add_with_mode(mode, Flag::set_with_mode(mode, true))
}
