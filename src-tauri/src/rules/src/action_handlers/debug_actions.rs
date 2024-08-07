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

use data::actions::debug_action::DebugGameAction;
use data::actions::game_action::GameAction;
use data::card_states::zones::ZoneQueries;
use data::core::numerics::LifeValue;
use data::game_states::game_state::GameState;
use data::player_states::player_state::PlayerQueries;
use data::prompts::pick_number_prompt::PickNumberPrompt;
use data::text_strings::Text;
use primitives::game_primitives::{CardType, PlayerName, Source, Zone};
use tracing::{debug, instrument};
use utils::outcome;
use utils::outcome::Outcome;

use crate::mutations::{move_card, players};
use crate::prompt_handling::prompts;
use crate::queries::card_queries;

#[instrument(level = "debug", skip(game))]
pub fn execute(game: &mut GameState, player: PlayerName, action: DebugGameAction) {
    match action {
        DebugGameAction::SetLifeTotal(target) => {
            let amount = prompts::pick_number(game, player, Text::SelectNumber, PickNumberPrompt {
                minimum: 0,
                maximum: 20,
            });
            debug!(?target, ?amount, "(Debug) Setting life total");
            players::set_life_total(game, Source::Game, target, amount as LifeValue);
        }
        DebugGameAction::RevealHand(target) => {
            for card_id in game.hand(target).clone() {
                outcome::execute(|| {
                    let card = game.card_mut(card_id)?;
                    card.revealed_to.insert(player);
                    outcome::OK
                });
            }
        }
        DebugGameAction::DestroyAllLands(target) => {
            for permanent_id in game.battlefield(target).clone() {
                outcome::execute(|| {
                    if card_queries::card_types(game, Source::Game, permanent_id)?
                        .contains(CardType::Land)
                    {
                        move_card::run(game, Source::Game, permanent_id, Zone::Graveyard)?;
                    }
                    outcome::OK
                });
            }
        }
    }
}
