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

use std::sync::Arc;

use data::actions::new_game_action::{NewGameAction, NewGameDebugOptions};
use data::actions::user_action::UserAction;
use data::core::primitives::{PlayerName, UserId};
use data::decks::deck_name;
use data::game_states::game_state::{DebugActAsPlayer, DebugConfiguration};
use data::users::user_state::UserState;
use database::sqlite_database::SqliteDatabase;
use display::commands::command::Command;
use display::commands::scene_identifier::SceneIdentifier;
use display::core::game_view::GameButtonView;
use display::core::main_menu_view::MainMenuView;
use tracing::info;
use uuid::uuid;

use crate::server_data::{ClientData, GameResponse};

/// Connect to the main menu scene
pub fn connect(_: SqliteDatabase, user: &UserState) -> GameResponse {
    info!(?user.id, "Connected");
    let client_data = ClientData::new(user.id, SceneIdentifier::MainMenu);
    GameResponse::new(client_data).command(Command::UpdateMainMenuView(main_menu_view()))
}

pub fn main_menu_view() -> MainMenuView {
    let new_game = UserAction::NewGameAction(NewGameAction {
        deck: deck_name::GREEN_VANILLA,
        opponent_deck: deck_name::GREEN_VANILLA,
        opponent_id: None,
        debug_options: NewGameDebugOptions {
            override_game_id: None,
            configuration: DebugConfiguration { reveal_all_cards: true, act_as_player: None },
        },
    });
    let new_debug_game = UserAction::NewGameAction(NewGameAction {
        deck: deck_name::GREEN_VANILLA,
        opponent_deck: deck_name::GREEN_VANILLA,
        opponent_id: None,
        debug_options: NewGameDebugOptions {
            override_game_id: None,
            configuration: DebugConfiguration {
                reveal_all_cards: true,
                act_as_player: Some(DebugActAsPlayer {
                    id: UserId(uuid!("d5f8cda2-0916-4655-8353-aaf435d562a5")),
                    name: PlayerName::Two,
                }),
            },
        },
    });
    let buttons = vec![
        GameButtonView::new_primary("Play", new_game),
        GameButtonView::new_default("Debug Game", new_debug_game),
        GameButtonView::new_default("Codex", UserAction::QuitGameAction),
        GameButtonView::new_default("Community", UserAction::QuitGameAction),
        GameButtonView::new_default("Settings", UserAction::QuitGameAction),
        GameButtonView::new_default("Quit", UserAction::QuitGameAction),
    ];
    MainMenuView { buttons }
}
