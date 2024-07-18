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

/// A stack of active prompts for a player.
///
/// Prompts represent choices a player must make within a game. No other game
/// action_handlers are allowed while a prompt is active. Because this is a
/// stack, choices that cause further prompts to be shown will appear before
/// earlier prompts.
#[derive(Debug, Clone, Default)]
pub struct PromptStack {}
