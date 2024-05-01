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

use std::io;
use std::io::Stdout;

use color_eyre::eyre::Result;
use crossterm::cursor;
use crossterm::event::{
    DisableBracketedPaste, DisableMouseCapture, EnableBracketedPaste, EnableMouseCapture,
};
use crossterm::terminal::{EnterAlternateScreen, LeaveAlternateScreen, SetTitle};
use ratatui::prelude::CrosstermBackend;
use ratatui::Terminal;

pub type Tui = Terminal<CrosstermBackend<Stdout>>;

pub fn enter() -> Result<Tui> {
    crossterm::terminal::enable_raw_mode()?;
    crossterm::execute!(io::stdout(), EnterAlternateScreen, cursor::Hide)?;
    crossterm::execute!(io::stdout(), EnableMouseCapture)?;
    crossterm::execute!(io::stdout(), EnableBracketedPaste)?;
    crossterm::execute!(io::stdout(), SetTitle("✦ spellclash ✦"))?;
    Ok(Terminal::new(CrosstermBackend::new(io::stdout()))?)
}

pub fn exit() -> Result<()> {
    if crossterm::terminal::is_raw_mode_enabled()? {
        crossterm::execute!(io::stdout(), DisableBracketedPaste)?;
        crossterm::execute!(io::stdout(), DisableMouseCapture)?;
        crossterm::execute!(io::stdout(), LeaveAlternateScreen, cursor::Show)?;
        crossterm::terminal::disable_raw_mode()?;
    }
    Ok(())
}
