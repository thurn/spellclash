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

use data::actions::user_action::UserAction;
use data::game_states::game_state::GameState;
use data::prompts::prompt::{Prompt, PromptResponse};
use serde::{Deserialize, Serialize};
use specta::{DataType, Generics, Type, TypeMap};
use tokio::sync::oneshot;

use crate::commands::field_state::{FieldKey, FieldValue};

/// Contains user interface state information which is persisted in memory
/// during gameplay, but which is not serialized to the database.
///
/// Don't store anything here which can't be easily reconstructed if the client
/// exits.
#[derive(Default)]
pub struct DisplayState {
    /// States of displayed input fields.
    pub fields: BTreeMap<FieldKey, FieldValue>,

    /// A prompt currently being shown to the player.
    pub prompt: Option<Prompt>,

    /// A channel on which to send a [PromptResponse] to select an option to
    /// respond to the prompt in [Self::prompt].
    pub prompt_channel: Option<oneshot::Sender<PromptResponse>>,

    /// Current state of the game, used to render correct updates when a prompt
    /// is active.
    pub game_snapshot: Option<GameState>,
}

impl Type for DisplayState {
    fn inline(type_map: &mut TypeMap, generics: Generics) -> DataType {
        DataType::Unknown
    }
}
