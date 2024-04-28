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

use ratatui::layout::{Rect, Size};
use ratatui::prelude::*;

/// Returns a rectangle of the given [Size] centered within the provided [Rect].
///
/// If the provided size is too large to fit within the provided rectangle, it
/// is clamped to fit.
pub fn centered_rect(mut size: Size, rect: Rect) -> Rect {
    if size.width > rect.width {
        size.width = rect.width;
    }
    if size.height > rect.height {
        size.height = rect.height;
    }

    let vertical = Layout::vertical([
        Constraint::Fill(1),
        Constraint::Length(size.height),
        Constraint::Fill(1),
    ])
    .split(rect)[1];

    Layout::horizontal([Constraint::Fill(1), Constraint::Length(size.width), Constraint::Fill(1)])
        .split(vertical)[1]
}
