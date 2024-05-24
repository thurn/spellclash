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

use std::fmt::Debug;

use dyn_clone::DynClone;

use crate::card_states::card_reference::CardReference;
use crate::printed_cards::printed_card_id::PrintedCardId;

/// Trait representing access to the Oracle card database.
pub trait Oracle: Debug + DynClone {
    /// Looks up card information based on its [PrintedCardId]
    fn card(&self, id: PrintedCardId) -> CardReference;
}

dyn_clone::clone_trait_object!(Oracle);
