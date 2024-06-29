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

use crate::core::primitives::{HasSource, Source};

#[derive(Clone, Debug, Copy, Eq, PartialEq)]
#[must_use]
pub struct Flag {
    changed_by: Source,
    value: bool,
    locked: bool,
}

impl Flag {
    /// Creates a new flag with the given initial value
    pub fn new(value: bool) -> Self {
        Self { changed_by: Source::Game, value, locked: false }
    }

    /// Returns the current value of the flag
    pub fn value(self) -> bool {
        self.value
    }

    /// Boolean 'and' operation on a Flag.
    ///
    /// The resulting flag value will be true if the previous value was true
    /// *and* the provided `value` is true.
    ///
    /// If the provided `source` is an ability source and `value` is false,
    /// calling this method will lock the flag via [Self::lock]. This causes
    /// conditions to always 'win' over permissions, also known as the "can't
    /// beats can" rule.
    ///
    /// If the flag is locked, this method has no effect.
    pub fn add_condition(mut self, has_source: impl HasSource, value: bool) -> Self {
        if self.locked {
            return self;
        }

        let source = has_source.source();
        let new_value = self.value && value;
        if new_value != self.value {
            // Store the source which caused the value to change.
            self.changed_by = source;
        }
        if !value && source.is_ability_source() {
            self = self.lock();
        }
        self.value = new_value;
        self
    }

    /// Boolean 'or' operation on a Flag.
    ///
    /// The resulting flag value will be true if the previous value was true
    /// *or* the provided `value` is true.
    ///
    /// If the flag is locked, this method has no effect.
    pub fn add_permission(mut self, has_source: impl HasSource, value: bool) -> Self {
        if self.locked {
            return self;
        }

        let source = has_source.source();
        let new_value = self.value || value;
        if new_value != self.value {
            // Store the source which caused the value to change.
            self.changed_by = source;
        }
        self.value = new_value;
        self
    }

    /// Helper function equivalent to `add_condition(source, true)`.
    pub fn yes(self, source: impl HasSource) -> Self {
        self.add_permission(source, true)
    }

    /// Prevent all further modifications to the value of this flag.
    pub fn lock(mut self) -> Self {
        self.locked = true;
        self
    }
}

pub trait FlagOption {
    fn value(&self) -> bool;
}

impl FlagOption for Option<Flag> {
    fn value(&self) -> bool {
        self.map_or(false, |flag| flag.value())
    }
}
