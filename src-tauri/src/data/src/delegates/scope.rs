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

use primitives::game_primitives::AbilityId;

use crate::card_states::zones::HasZones;
/// Context for an ability while it is being created, used to register
/// callbacks, etc
#[derive(Copy, Clone)]
pub struct AbilityScope {
    /// ID of this ability
    pub ability_id: AbilityId,
}
