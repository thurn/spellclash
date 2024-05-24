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

use std::sync::Arc;

use dashmap::mapref::one::Ref;

use crate::card_definitions::card_name::CardName;
use crate::printed_cards::printed_card::PrintedCard;
use crate::printed_cards::printed_card_id::PrintedCardId;

/// Information about a specific printing of a card, used to instantiate cards
/// in a game.
#[derive(Debug)]
pub struct CardReference {
    pub identifier: PrintedCardId,
    pub printed_card_reference: Arc<PrintedCard>,
}
