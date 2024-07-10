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

use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};
use serde_with::serde_as;
use specta::Type;

#[derive(
    Copy, Clone, Eq, PartialEq, Hash, Debug, Ord, PartialOrd, Serialize, Deserialize, Type,
)]
#[serde(rename_all = "camelCase")]
pub enum FieldKey {
    PickNumberPrompt,
}

#[derive(Clone, Eq, PartialEq, Debug, Serialize, Deserialize, Type)]
#[serde(rename_all = "camelCase")]
pub enum FieldValue {
    String(String),
}

impl FieldValue {
    pub fn as_u32(&self) -> Option<u32> {
        match self {
            FieldValue::String(s) => s.parse().ok(),
        }
    }
}
