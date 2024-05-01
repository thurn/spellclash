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

use crossterm::event::{Event, KeyCode, KeyEventKind, MouseButton, MouseEventKind};
use data::actions::interface_action::InterfaceAction;
use data::core::widget_id::WidgetId;
use ratatui::layout::Position;
use ratatui::prelude::*;
use tracing::info;

#[derive(Default)]
pub struct RenderContext {
    event: Option<Event>,
    current_hover: Option<WidgetId>,
    current_mouse_down: Option<WidgetId>,
    exit: bool,
    action: Option<InterfaceAction>,
}

impl RenderContext {
    pub fn set_last_event(&mut self, event: Option<Event>) {
        if let Some(Event::Key(e)) = event {
            if e.kind == KeyEventKind::Press && e.code == KeyCode::Char('q') {
                self.exit = true;
            }
        }

        self.event = event;
    }

    pub fn set_current_hover(&mut self, current: Option<WidgetId>) {
        self.current_hover = current;
    }

    pub fn set_current_mouse_down(&mut self, current: Option<WidgetId>) {
        self.current_mouse_down = current;
    }

    pub fn should_exit(&self) -> bool {
        self.exit
    }

    pub fn finish_render(&mut self) -> Option<InterfaceAction> {
        let action = self.action;
        self.action = None;

        if matches!(self.event, Some(Event::Mouse(e))
            if e.kind == MouseEventKind::Up(MouseButton::Left))
        {
            // Stop press event states on mouse up
            self.current_mouse_down = None;
        }

        self.event = None;
        action
    }

    pub fn hovered(&mut self, id: WidgetId, area: Rect) -> bool {
        let current = self.current_hover == Some(id);
        let Some(Event::Mouse(e)) = self.event else {
            return current;
        };

        if e.kind == MouseEventKind::Moved {
            if area.contains(Position::new(e.column, e.row)) {
                self.action = Some(InterfaceAction::SetHover(Some(id)));
            } else if current && self.action.is_none() {
                self.action = Some(InterfaceAction::SetHover(None));
            }
        }

        current
    }

    pub fn mouse_down(&mut self, id: WidgetId, area: Rect) -> bool {
        let current = self.current_mouse_down == Some(id);
        let Some(Event::Mouse(e)) = self.event else {
            return current;
        };

        if e.kind == MouseEventKind::Down(MouseButton::Left) {
            if area.contains(Position::new(e.column, e.row)) {
                self.action = Some(InterfaceAction::SetMouseDown(Some(id)));
            } else if current && self.action.is_none() {
                self.action = Some(InterfaceAction::SetHover(None));
            }
        }

        current
    }

    pub fn clicked(&mut self, id: WidgetId, area: Rect, action: impl Into<InterfaceAction>) {
        if matches!(self.event, Some(Event::Mouse(e))
            if e.kind == MouseEventKind::Up(MouseButton::Left)
               && area.contains(Position::new(e.column, e.row))
               && self.current_mouse_down == Some(id))
        {
            info!(?id, "Widget clicked");
            self.action = Some(action.into());
        }
    }
}
