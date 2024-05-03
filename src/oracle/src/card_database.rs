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

use data::game_states::game_state::GameState;
use database::database::Database;
use utils::outcome::Outcome;
use utils::with_error::WithError;
use utils::{fail, outcome};

use crate::card_json;

pub async fn populate(_: &impl Database, game: &mut GameState) -> Outcome {
    let cards = match card_json::CARDS.as_ref() {
        Ok(c) => c,
        Err(e) => {
            fail!("Error parsing card json data: {:?}", e);
        }
    };

    for card in game.zones.all_cards_mut() {
        card.printed_card_reference = Some(
            cards
                .get(&card.card_name)
                .with_error(|| format!("Unknown card: {:?}", card.card_name))?,
        );
    }

    outcome::OK
}
