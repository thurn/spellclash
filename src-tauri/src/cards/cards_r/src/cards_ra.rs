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

use abilities::core::effects;
use abilities::core::gain_ability::GainAbility;
use abilities::keyword_abilities::haste;
use abilities::targeting::targets;
use data::card_definitions::ability_definition::{DelayedTrigger, SpellAbility};
use data::card_definitions::card_definition::CardDefinition;
use data::card_definitions::card_name;
use data::core::primitives::HasSource;
use data::game_states::effect_state::EffectState;
use rules::mutations::trigger_extension::TriggerExt;
use rules::mutations::{change_controller, delayed_trigger, permanents};

pub fn ray_of_command() -> CardDefinition {
    let state = EffectState::new(0);
    CardDefinition::new(card_name::RAY_OF_COMMAND).ability(
        SpellAbility::new()
            .targets(targets::creature_opponent_controls())
            .effect(|g, c, target| {
                permanents::untap(g, c.source(), target);
                change_controller::gain_control_this_turn(g, c.controller(), c.effect_id, target);
                effects::target_this_turn(g, c, target);
                delayed_trigger::enable(g, c, state, target);
            })
            .delegates(|d| {
                d.permanent_controller_changed.delayed_trigger_if(|g, s, effect_id, data| {
                    data.old_controller == s.controller
                        && state.matches(g, effect_id, data.permanent_id)
                });
                haste::gain(d, GainAbility::ForTargetThisTurn);
            })
            .delayed_trigger(DelayedTrigger::new().effect(|g, c| {
                let effect_id = state.pop(g, c.effect_id);
                permanents::tap(g, c.source(), effect_id);
            })),
    )
}
