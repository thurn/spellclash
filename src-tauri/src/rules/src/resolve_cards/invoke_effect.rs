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

use data::card_definitions::ability_choices::CardOrPlayer;
use data::card_definitions::ability_definition::EffectFn;
use data::card_states::zones::ZoneQueries;
use data::core::primitives::{
    AbilityId, CardId, EntityId, PlayerName, StackAbilityId, StackItemId,
};
use data::delegates::has_delegates::HasDelegates;
use data::game_states::game_state::GameState;
use utils::outcome;
use utils::outcome::{Outcome, OutcomeWithResult};

pub fn run(
    game: &mut GameState,
    ability_id: AbilityId,
    stack_ability_id: Option<StackAbilityId>,
    targets: Vec<EntityId>,
    effect: &EffectFn,
) -> Outcome {
    let scope = game.create_scope(ability_id, stack_ability_id);
    match effect {
        EffectFn::NoEffect => outcome::OK,
        EffectFn::Untargeted(function) => function(game, scope),
        EffectFn::SingleCardTarget(function) => {
            if let Some(card_id) = single_card(game, &targets) {
                function(game, scope, card_id)
            } else {
                outcome::OK
            }
        }
        EffectFn::SinglePlayerTarget(fun) => {
            if let Some(player) = single_player(&targets) {
                fun(game, scope, player)
            } else {
                outcome::OK
            }
        }
        EffectFn::SingleCardOrPlayerTarget(fun) => {
            if let Some(card_id) = single_card(game, &targets) {
                fun(game, scope, CardOrPlayer::Card(card_id))
            } else if let Some(player) = single_player(&targets) {
                fun(game, scope, CardOrPlayer::Player(player))
            } else {
                outcome::OK
            }
        }
        EffectFn::Targeted(function) => function(game, scope, &targets),
    }
}

fn single_card(game: &GameState, targets: &[EntityId]) -> Option<CardId> {
    let entity = targets.first().copied()?;
    game.card_entity(entity).map(|c| c.id)
}

fn single_player(targets: &[EntityId]) -> Option<PlayerName> {
    let entity = targets.first().copied()?;
    match entity {
        EntityId::Player(player) => Some(player),
        _ => None,
    }
}
