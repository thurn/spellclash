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

use color_eyre::Report;

/// Possible reasons why the rules engine should halt
#[derive(Debug)]
#[must_use]
pub enum StopCondition {
    Prompt,
    GameOver,
    Error(Report),
}

/// Represents the result of some game mutation.
///
/// The "outcome" system is a wrapper around [Result] that adds additional cases
/// where the rules engine should halt, but which are not necessarily
/// traditional errors. The [StopCondition] enum describe these in more detail.
pub type Outcome = Result<(), StopCondition>;

/// Equivalent alias to [Outcome] which wraps a returned value
pub type Value<T> = Result<T, StopCondition>;

/// Mutation completed successfully, execution can continue
pub const OK: Outcome = Ok(());

/// Mutation resulted in a choice the user must make, execution should halt
/// until choice is made and then restart.
pub const PROMPT: Outcome = Err(StopCondition::Prompt);

/// Mutation resulted in the game being over, execution should halt.
pub const GAME_OVER: Outcome = Err(StopCondition::GameOver);
