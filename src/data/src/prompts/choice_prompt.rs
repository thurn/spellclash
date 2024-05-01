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

use serde::{Deserialize, Serialize};

use crate::core::primitives::ObjectId;
use crate::costs::cost::Cost;
use crate::effects::effect::Effect;
use crate::text_strings::Text;

/// A blocking choice for a player to pick one of a list of options before
/// any other game actions can occur.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChoicePrompt {
    /// If true, display a "continue" option to skip this prompt without taking
    /// an action.
    pub optional: bool,
    /// Choices to display for this prompt
    pub choices: Vec<Choice>,
}

/// A single option a user can select in a [ChoicePrompt].
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Choice {
    /// A label to display for the choice
    pub label: Text,

    /// The game object associated with this choice.
    ///
    /// If this object ID no longer exists when the prompt is shown, the choice
    /// will be omitted. If all non-"continue" choices are skipped, no prompt is
    /// shown at all.
    pub object_id: ObjectId,

    /// Costs to make this choice.
    pub costs: Vec<Cost>,

    /// Effects of this choice. If [Self::object_id] is provided, these effects
    /// will applied to that object, otherwise they will be evaluated with an
    /// empty object set.
    pub effects: Vec<Effect>,
}
