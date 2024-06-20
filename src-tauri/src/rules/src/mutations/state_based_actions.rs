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

use std::cell::Cell;

use data::card_states::zones::ZoneQueries;
use data::core::numerics::{LifeValue, Toughness};
use data::core::primitives::{Source, Zone};
use data::game_states::game_state::{GameState, GameStatus};
use data::game_states::state_based_event::StateBasedEvent;
use data::player_states::player_state::PlayerQueries;
use enumset::EnumSet;
use tracing::instrument;
use utils::outcome;
use utils::outcome::Outcome;

use crate::mutations::move_card;
use crate::queries::{card_queries, player_queries};

/// Checks for state-based actions to perform in the provided game state.
#[instrument(name = "state_based_actions_run", level = "debug", skip(game))]
pub fn run(game: &mut GameState) -> Outcome {
    check_state_triggered_abilities(game)?;

    // > 704.3. Whenever a player would get priority (see rule 117, "Timing and
    // > Priority"), the game checks for any of the listed conditions for
    // > state-based actions, then performs all applicable state-based actions
    // > simultaneously as a single event. If any state-based actions are performed
    // > as a result of a check, the check is repeated; otherwise all triggered
    // > abilities that are waiting to be put on the stack are put on the stack,
    // > then the check is repeated. Once no more state-based actions have been
    // > performed as the result of a check and no triggered abilities are waiting
    // > to be put on the stack, the appropriate player gets priority
    // <https://yawgatog.com/resources/magic-rules/#R7043>

    let mut lost = EnumSet::empty();

    loop {
        let events = game.state_based_events.take().unwrap_or_default();
        if events.is_empty() {
            break;
        }

        for event in events {
            match event {
                StateBasedEvent::LifeTotalDecrease(player) => {
                    if game.player(player).life <= 0 {
                        lost.insert(player);
                    }
                }
                StateBasedEvent::DrawFromEmptyLibrary(player) => {
                    lost.insert(player);
                }
                StateBasedEvent::GainedPoisonCounters(_) => {}
                StateBasedEvent::TokenLeftBattlefield(_) => {}
                StateBasedEvent::CopyLeftStackOrBattlefield(_) => {}
                StateBasedEvent::CreatureToughnessChanged(card_id) => {
                    if card_queries::toughness(game, card_id) <= 0 {
                        move_card::run(game, Source::Game, card_id, Zone::Graveyard)?;
                    }
                }
                StateBasedEvent::CreatureDamaged(card_id) => {
                    if game.card(card_id).damage as i64 >= card_queries::toughness(game, card_id) {
                        move_card::run(game, Source::Game, card_id, Zone::Graveyard)?;
                    }
                }
                StateBasedEvent::CreatureDamagedByDeathtouch(card_id) => {
                    move_card::run(game, Source::Game, card_id, Zone::Graveyard)?;
                }
                StateBasedEvent::PlaneswalkerLostLoyalty(_) => {}
                StateBasedEvent::LegendaryPermanentEntered(_) => {}
            }
        }
    }

    if !lost.is_empty() {
        game.status =
            GameStatus::GameOver { winners: player_queries::all_players(game).difference(lost) };
    }

    outcome::OK
}

/// Checks for state-triggered abilities to fire.
///
/// > 603.8. Some triggered abilities trigger when a game state (such as a
/// > player controlling no permanents of a particular card type) is true,
/// > rather than triggering when an event occurs. These abilities trigger as
/// > soon as the game state matches the condition. They'll go onto the stack at
/// > the next available opportunity. These are called state triggers. (Note
/// > that state triggers aren't the same as state-based actions.) A
/// > state-triggered ability doesn't trigger again until the ability has
/// > resolved, has been countered, or has otherwise left the stack. Then, if
/// > the object with the ability is still in the same zone and the game state
/// > still matches its trigger condition, the ability will trigger again.
///
/// Per the rules, state triggered abilities can be triggered after *any*
/// mutation to game state, but I tried doing that and it's ludicrously
/// expensive. Instead, we just check for state-triggered abilities as part of
/// the state-based actions check and can insert ad-hoc checks into game effects
/// that seem likely to produce conditions *during* their execution that are not
/// visible *afterwards*, e.g. flicker effects. It's not perfect, but this is
/// probably the only reasonable way to handle it.
pub fn check_state_triggered_abilities(game: &mut GameState) -> Outcome {
    if !game.delegates.state_triggered_abilities.is_empty()
        && !game.checking_state_triggered_abilities
    {
        // Only run the check if it's not already running to prevent infinite loops.
        game.checking_state_triggered_abilities = true;
        game.delegates.state_triggered_abilities.invoke_with(game, &()).run(game)?;
        game.checking_state_triggered_abilities = false;
    }

    outcome::OK
}
