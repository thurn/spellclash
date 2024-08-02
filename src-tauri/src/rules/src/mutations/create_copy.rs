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

use data::card_states::card_kind::CardKind;
use data::card_states::card_state::CardFacing;
use data::card_states::zones::{ToCardId, ZoneQueries};
use data::game_states::game_state::GameState;
use data::printed_cards::printed_card::Face;
use primitives::game_primitives::{PermanentId, PlayerName, Zone};
use tracing::Instrument;
use utils::outcome;
use utils::outcome::Outcome;

use crate::core::initialize_card;
use crate::queries::player_queries;

/// Creates a copy of a permanent currently on the battlefield as a token owned
/// by the [PlayerName] player.
pub fn of_permanent(game: &mut GameState, id: PermanentId, owner: PlayerName) -> Outcome {
    on_battlefield_of_card(game, id, owner, game.card(id)?.facing)
}

/// Creates a copy of a card, which does not need to be on the battlefield, as a
/// token owned by the [PlayerName] player using the provided [CardFacing].
pub fn on_battlefield_of_card(
    game: &mut GameState,
    id: impl ToCardId,
    owner: PlayerName,
    facing: CardFacing,
) -> Outcome {
    let all_players = player_queries::all_players(game);
    let reference = game.oracle().card(game.card(id)?.printed_card_id);
    let turn = game.turn;
    let new_card_id = game.zones.create_card_in_zone(
        reference,
        Zone::Battlefield,
        CardKind::TokenOrStackCopy,
        owner,
        turn,
    );
    game.card_mut(new_card_id)?.facing = facing;
    game.card_mut(new_card_id)?.revealed_to = all_players;
    initialize_card::run(game, new_card_id)
}
