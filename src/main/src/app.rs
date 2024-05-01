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

use std::time::Duration;

use color_eyre::Result;
use crossterm::event;
use data::actions::interface_action::InterfaceAction;
use data::game_states::game_state::GameState;
use display::core::layout;
use display::core::render_context::RenderContext;
use ratatui::layout::Size;
use ratatui::prelude::*;
use ratatui::symbols::border;
use ratatui::widgets::{Block, Borders, Paragraph, Wrap};
use rules::core::{handle_action, new_game};
use tracing::info;

use crate::tui::Tui;

pub fn run(tui: &mut Tui) -> Result<()> {
    let mut data = new_game::create();
    let mut context = RenderContext::default();
    while !context.should_exit() {
        context.set_last_event(if event::poll(Duration::from_millis(16))? {
            Some(event::read()?)
        } else {
            None
        });
        tui.draw(|frame| {
            frame.render_stateful_widget(App { data: &data }, frame.size(), &mut context);

            let Some(action) = context.finish_render() else {
                return;
            };

            match action {
                InterfaceAction::GameAction(game_action) => {
                    info!(?game_action, "Handling GameAction");
                    handle_action::handle_game_action(&mut data, game_action);
                }
                InterfaceAction::NewGameAction(new_game) => {
                    info!(?new_game, "Handling NewGameAction");
                }
                InterfaceAction::SetHover(id) => {
                    context.set_current_hover(id);
                }
                InterfaceAction::SetMouseDown(id) => {
                    context.set_current_mouse_down(id);
                }
            };
        })?;
    }
    Ok(())
}

pub struct App<'a> {
    pub data: &'a GameState,
}

const SCREEN_HEIGHT: u16 = 40;
const SCREEN_WIDTH: u16 = 56;

impl<'a> StatefulWidget for App<'a> {
    type State = RenderContext;

    fn render(self, area: Rect, buf: &mut Buffer, _context: &mut RenderContext) {
        if area.width < SCREEN_WIDTH || area.height < SCREEN_HEIGHT {
            Paragraph::new(vec![
                Line::from(
                    format!("Error: The minimum terminal size for this game is {SCREEN_HEIGHT} columns by {SCREEN_WIDTH} rows!"),
                ),
                Line::from(format!("Your terminal is {} by {}.", area.width, area.height)),
                Line::from("Press 'q' to quit."),
            ])
            .wrap(Wrap { trim: false })
            .alignment(Alignment::Center)
            .render(area, buf);
        } else if area.width >= SCREEN_WIDTH + 2 && area.height >= SCREEN_HEIGHT + 2 {
            // Render an outline around the game area if there's room
            let outline = layout::centered_rect(
                Size { width: SCREEN_WIDTH + 2, height: SCREEN_HEIGHT + 2 },
                area,
            );
            let block = Block::default().borders(Borders::ALL).border_set(border::ROUNDED);
            let game_area = block.inner(outline);
            block.render(outline, buf);
            render_game_area(game_area, buf);
        } else {
            let game_area =
                layout::centered_rect(Size { width: SCREEN_WIDTH, height: SCREEN_HEIGHT }, area);
            render_game_area(game_area, buf);
        }
    }
}

fn render_game_area(area: Rect, buf: &mut Buffer) {
    assert_eq!(area.width, SCREEN_WIDTH);
    assert_eq!(area.height, SCREEN_HEIGHT);
    Block::default().borders(Borders::ALL).border_set(border::DOUBLE).render(area, buf);
}
