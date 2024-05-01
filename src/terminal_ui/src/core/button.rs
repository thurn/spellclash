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

use ratatui::prelude::*;
use ratatui::symbols::border;
use ratatui::widgets::{Block, Borders, Paragraph, Wrap};
use typed_builder::TypedBuilder;

use crate::core::colors;
use crate::core::interface_action::InterfaceAction;
use crate::core::render_context::RenderContext;
use crate::core::widget_id::WidgetId;

#[derive(TypedBuilder)]
#[builder(builder_method(name = new))]
pub struct Button {
    #[builder(setter(into))]
    label: String,
    #[builder(setter(into))]
    action: InterfaceAction,
    id: WidgetId,
}

impl StatefulWidget for Button {
    type State = RenderContext;

    fn render(self, area: Rect, buf: &mut Buffer, context: &mut RenderContext) {
        let hovered = context.hovered(self.id, area);
        let pressed = context.mouse_down(self.id, area);
        context.clicked(self.id, area, self.action);

        Paragraph::new(
            self.label
                .split('\n')
                .map(|s| Line::from(text_style(s, hovered, pressed)))
                .collect::<Vec<_>>(),
        )
        .alignment(Alignment::Center)
        .wrap(Wrap { trim: true })
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_set(border::DOUBLE)
                .border_style(colors::white()),
        )
        .render(area, buf)
    }
}

fn text_style(text: &str, hovered: bool, pressed: bool) -> Span {
    let mut result = text.fg(colors::white());
    result = if pressed { result.underlined() } else { result };

    if hovered {
        result.bg(colors::selected())
    } else {
        result
    }
}
