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
use data::card_states::card_state::{CardFacing, TappedState};
use data::card_states::zones::ZoneQueries;
use data::core::numerics::Damage;
use data::core::primitives::{CardId, EntityId, HasCardId, HasSource, Zone, ALL_POSSIBLE_PLAYERS};
use data::game_states::game_state::{GameState, TurnData};
use data::game_states::state_based_event::StateBasedEvent;
use tracing::debug;
use utils::outcome;
use utils::outcome::Outcome;

/// Moves a card to a new zone, updates indices, assigns a new
/// [EntityId] to it, and fires all relevant events.
///
/// The card is added as the top card of the target zone if it is ordered.
///
/// Panics if this card was not found in its previous zone.
pub fn run(
    game: &mut GameState,
    _source: impl HasSource,
    id: impl HasCardId,
    zone: Zone,
) -> Outcome {
    let id = id.card_id();
    debug!(?id, ?zone, "Moving card to zone");
    on_leave_zone(game, id, game.card(id.card_id()).zone);
    game.zones.move_card(id, zone);
    on_enter_zone(game, id, zone);
    outcome::OK
}

fn on_leave_zone(game: &mut GameState, card_id: CardId, zone: Zone) {
    match zone {
        Zone::Stack => {
            let card = game.card_mut(card_id);
            card.cast_as.clear();
            card.targets.clear();
        }
        Zone::Battlefield => {
            let card = game.card_mut(card_id);
            card.tapped_state = TappedState::Untapped;
            card.damage = 0;
            card.attached_to = None;
        }
        _ => {}
    }
}

fn on_enter_zone(game: &mut GameState, card_id: CardId, zone: Zone) {
    let turn = game.turn;
    let card = game.card_mut(card_id);
    card.entered_current_zone = turn;
    match zone {
        Zone::Stack | Zone::Battlefield | Zone::Graveyard => {
            card.revealed_to = ALL_POSSIBLE_PLAYERS;
        }
        Zone::Hand => {
            let controller = card.controller;
            card.revealed_to.insert(controller);
            card.facing = CardFacing::FaceDown;
        }
        Zone::Library => {
            card.facing = CardFacing::FaceDown;
        }
        _ => {}
    }

    if card.kind == CardKind::Token && zone != Zone::Battlefield {
        game.add_state_based_event(StateBasedEvent::TokenLeftBattlefield(card_id));
    }
}
