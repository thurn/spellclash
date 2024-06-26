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

/// Helper function to run a closure and return `true` if the result is
/// `Some(true)`.
pub fn is_true(function: impl FnOnce() -> Option<bool>) -> bool {
    function().unwrap_or(false)
}

/// Helper function to run a closure and return `true` if the result is
/// `None` or `Some(false)`.
pub fn is_false(function: impl FnOnce() -> Option<bool>) -> bool {
    !is_true(function)
}

pub trait FilterSome: Iterator {
    /// Creates an iterator which yields only items for which a given predicate
    /// returns Some(true).
    fn filter_some<P>(self, predicate: P) -> impl Iterator<Item = Self::Item>
    where
        Self: Sized,
        P: FnMut(&Self::Item) -> Option<bool>;
}

impl<TIterator: Iterator> FilterSome for TIterator {
    fn filter_some<P>(self, mut predicate: P) -> impl Iterator<Item = Self::Item>
    where
        Self: Sized,
        P: FnMut(&Self::Item) -> Option<bool>,
    {
        self.filter(move |item| predicate(item) == Some(true))
    }
}
