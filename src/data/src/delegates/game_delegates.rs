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

use enumset::EnumSet;

use crate::core::primitives::Zone;

pub type DelegateFn = fn(&mut GameDelegates);

pub struct Delegate {
    /// [Zone]s in which this delegate should be active.
    ///
    /// The [Self::run] function will be invoked to populate this delegate's
    /// callbacks when its entity enters one of these zones, and the callbacks
    /// will be cleared when it exits one of these zones.
    pub zones: EnumSet<Zone>,

    /// Function to populate callbacks for this delegate
    pub run: DelegateFn,
}

#[derive(Debug, Clone, Default)]
pub struct GameDelegates {}
