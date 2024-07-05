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
use serde::{Deserialize, Serialize};

use crate::text_strings::Text;

pub trait MultipleChoicePromptTrait: Debug + DynClone + Send {
    fn choices(&self) -> Vec<Text>;
}

dyn_clone::clone_trait_object!(MultipleChoicePromptTrait);

/// A choice for a player to pick one item from a list of choice buttons.
#[derive(Clone, Debug)]
pub struct MultipleChoicePrompt<T> {
    /// Choices to display for this prompt
    pub choices: Vec<T>,
}

impl<T: Into<Text> + Debug + Clone + Send> MultipleChoicePromptTrait for MultipleChoicePrompt<T> {
    fn choices(&self) -> Vec<Text> {
        self.choices.iter().map(|c| c.clone().into()).collect()
    }
}
