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

use primitives::game_primitives::{EntityId, PlayerName, Source};

use crate::card_definitions::ability_definition::{Ability, AbilityBuilder, AbilityType, NoEffect};
use crate::card_states::play_card_plan::{ModalChoice, PlayCardChoices};
use crate::events::event_context::EventContext;
use crate::game_states::game_state::GameState;

pub struct ModalEffect {
    pub modes: Vec<Box<dyn Ability>>,
}

impl ModalEffect {
    pub fn new() -> Self {
        Self { modes: Vec::new() }
    }

    pub fn mode(mut self, mode: impl Ability + 'static) -> Self {
        self.modes.push(Box::new(mode));
        self
    }
}

impl Default for ModalEffect {
    fn default() -> Self {
        Self::new()
    }
}

pub struct AbilityMode;

pub type AbilityModeBuilder = AbilityBuilder<NoEffect>;

impl AbilityMode {
    #[allow(clippy::new_ret_no_self)]
    pub fn new() -> AbilityModeBuilder {
        AbilityBuilder {
            ability_type: AbilityType::Spell,
            properties: None,
            global_events: None,
            card_events: None,
            effect: NoEffect,
        }
    }
}

impl Ability for AbilityBuilder<ModalEffect> {
    fn is_modal(&self) -> bool {
        true
    }

    fn modes<'a>(&'a self) -> Box<dyn Iterator<Item = ModalChoice> + 'a> {
        Box::new(self.effect.modes.iter().enumerate().map(|(i, _)| ModalChoice(i)))
    }

    fn requires_targets(&self) -> bool {
        self.effect.modes.iter().any(|mode| mode.requires_targets())
    }

    fn valid_targets<'a>(
        &'a self,
        game: &'a GameState,
        choices: &'a PlayCardChoices,
        source: Source,
    ) -> Box<dyn Iterator<Item = EntityId> + 'a> {
        Box::new(
            self.effect
                .modes
                .iter()
                .enumerate()
                .filter(|(i, _)| choices.modes.contains(&ModalChoice(*i)))
                .flat_map(move |(_, mode)| mode.valid_targets(game, choices, source)),
        )
    }

    fn invoke_effect(
        &self,
        game: &mut GameState,
        context: EventContext,
        choices: &Option<PlayCardChoices>,
    ) {
        let Some(modal_choices) = choices.as_ref().map(|c| &c.modes) else {
            panic!("Expected modal choices for modal effect");
        };

        for (i, mode) in self.effect.modes.iter().enumerate() {
            if modal_choices.contains(&ModalChoice(i)) {
                mode.invoke_effect(game, context, choices);
            }
        }
    }
}
