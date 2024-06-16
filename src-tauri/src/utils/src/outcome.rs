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

/// Provides the ability to halt the rules engine during execution in order to
/// cancel a selected user action.
pub type Outcome = Result<(), HaltCondition>;

/// Outcome which wraps a return value
pub type PromptResult<T> = Result<T, HaltCondition>;

/// Continue execution.
pub const OK: Outcome = Ok(());

/// Cancel the current user action.
pub const CANCEL: Outcome = Err(HaltCondition::Cancel);
