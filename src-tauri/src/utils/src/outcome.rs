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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HaltCondition {
    Cancel,
}

/// Marker struct indicating that an operation applied its effects successfully.
///
/// Typically, effect functions will return None if they are unable to apply
/// their operation (e.g. because the card in question no longer exists) and
/// Some(Success) if the operation occurred successfully.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Success;

/// The result of an operation which may not apply its effects.
///
/// This return type should only be used for low-level mutations which perform a
/// single atomic change to game state like "move a card" or "tap a permanent".
/// Anything involving larger steps like "untap your lands" or "assign combat
/// damage" should *not* employ this system, as generally those operations are
/// expected to continue even if some constituent part is missing.
///
/// For convenience in those cases, you can wrap individual blocks of logic in
/// the [execute] function to have them apply atomically. This is often used in
/// the bodies of for loops.
pub type Outcome = Option<Success>;

/// A constant representing a successful outcome.
pub const OK: Outcome = Some(Success);

/// A constant representing a skipped outcome.
pub const SKIPPED: Outcome = None;

pub trait IsSuccess {
    fn is_success(&self) -> bool;
}

/// Executes a function and returns its outcome.
///
/// Used to create a block of code which should return early if an output is
/// skipped.
pub fn execute(mut function: impl FnMut() -> Outcome) -> Outcome {
    function()
}
