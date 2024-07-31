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
use data::card_states::zones::ZoneQueries;
use data::core::function_types::CardPredicate;
use data::game_states::game_state::GameState;
use primitives::game_primitives::{EntityId, HasSource, PlayerName, Source, SpellId, StackItemId};

use crate::targeting::player_set;
use crate::targeting::player_set::PlayerSet;

pub struct SingleSpellSelector<TFn>
where
    TFn: CardPredicate<SpellId>,
{
    pub players: PlayerSet,
    pub predicate: TFn,
}

impl<TFn> SingleSpellSelector<TFn>
where
    TFn: CardPredicate<SpellId>,
{
    pub fn new(players: PlayerSet, predicate: TFn) -> Self {
        Self { players, predicate }
    }
}

impl<TFn> TargetSelector for SingleSpellSelector<TFn>
where
    TFn: CardPredicate<SpellId>,
{
    type Target = SpellId;

    fn valid_targets<'a>(
        &'a self,
        game: &'a GameState,
        controller: PlayerName,
        source: Source,
    ) -> Box<dyn Iterator<Item = EntityId> + 'a> {
        Box::new(
            player_set::players_in_set(game, controller, source, self.players).iter().flat_map(
                move |player| {
                    game.stack().iter().filter_map(move |&stack_item_id| {
                        let StackItemId::Card(card_id) = stack_item_id else {
                            return None;
                        };
                        let spell_id = SpellId::new(game.card(card_id)?.object_id, card_id);
                        if (self.predicate)(game, source, spell_id) == Some(true) {
                            Some(spell_id.into())
                        } else {
                            None
                        }
                    })
                },
            ),
        )
    }

    fn build_target_data(&self, game: &GameState, targets: &[EntityId]) -> Option<Self::Target> {
        Self::Target::try_from(*targets.first()?).ok()
    }
}
