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

use data::actions::game_action::GameAction;
use data::core::primitives::{CardId, PlayerName, Source, Zone};
use data::game_states::game_state::GameState;
use data::printed_cards::printed_card::Face;
use tracing::{debug, instrument};
use utils::outcome::Outcome;
use utils::{fail, outcome, verify};

use crate::legality::can_play_face::CanPlayAs;
use crate::legality::{can_play_face, legal_actions};
use crate::mutations::cards;
use crate::queries::players;
use crate::spell_casting::cast_spell;
use crate::steps::step;

#[instrument(err, level = "debug", skip(game))]
pub fn execute(game: &mut GameState, player: PlayerName, action: GameAction) -> Outcome {
    match action {
        GameAction::PassPriority => handle_pass_priority(game, player),
        GameAction::ProposePlayingCard(id) => handle_play_card(game, Source::Game, player, id),
    }
}

#[instrument(err, level = "debug", skip(game))]
fn handle_pass_priority(game: &mut GameState, player: PlayerName) -> Outcome {
    debug!(?player, "Passing priority");
    verify!(legal_actions::can_pass_priority(game, player), "Cannot pass priority for {player:?}");
    game.passed.insert(player);
    if game.passed.len() == game.configuration.all_players.len() {
        step::advance(game)
    } else {
        game.priority = players::next_player_after(game, game.priority);
        outcome::OK
    }
}

#[instrument(err, level = "debug", skip(game))]
fn handle_play_card(
    game: &mut GameState,
    source: Source,
    player: PlayerName,
    card: CardId,
) -> Outcome {
    debug!(?player, ?card, "Playing card");
    let Some(play) = can_play_face::play_as(game, player, card).next() else {
        fail!("Cannot legally play card {card:?} as {player:?}");
    };

    match play {
        CanPlayAs::Land(face) => handle_play_land(game, source, player, card, face),
        CanPlayAs::Instant(face) | CanPlayAs::Sorcery(face) => {
            cast_spell::run(game, source, card, face)
        }
    }
}

#[instrument(err, level = "debug", skip(game))]
fn handle_play_land(
    game: &mut GameState,
    source: Source,
    player: PlayerName,
    card: CardId,
    face: Face,
) -> Outcome {
    game.history_counters_mut(player).lands_played += 1;
    cards::turn_face_up(game, source, card, face)?;
    game.zones.move_card(source, card, Zone::Battlefield)
}
