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
use std::time::Duration;

use data::actions::new_game_action::{NewGameAction, NewGameDebugOptions};
use data::actions::user_action::UserAction;
use data::decks::deck_name;
use data::game_states::game_state::{DebugActAsPlayer, DebugConfiguration};
use data::player_states::game_agent::{
    AgentEvaluator, AgentType, ChildScoreAlgorithm, GameAgent, MonteCarloAgent, StateCombiner,
    StatePredictor,
};
use data::player_states::player_state::PlayerType;
use data::users::user_state::UserState;
use database::sqlite_database::SqliteDatabase;
use display::commands::command::{Command, SceneView};
use display::commands::scene_identifier::SceneIdentifier;
use display::core::game_view::GameButtonView;
use display::core::main_menu_view::MainMenuView;
use primitives::game_primitives::{PlayerName, UserId};
use tokio::sync::mpsc::UnboundedSender;
use tracing::info;
use uuid::{uuid, Uuid};

use crate::server_data::{Client, ClientData, GameResponse};

/// Connect to the main menu scene
pub fn connect(response_channel: UnboundedSender<GameResponse>, user: &UserState) {
    info!(?user.id, "Connected");
    let client = Client {
        data: ClientData { user_id: user.id, scene: SceneIdentifier::MainMenu, id: Uuid::new_v4() },
        channel: response_channel,
    };
    client.send(Command::UpdateScene(SceneView::MainMenuView(main_menu_view())));
}

pub fn main_menu_view() -> MainMenuView {
    let deck = deck_name::DANDAN;
    let opponent_id = UserId(uuid!("d5f8cda2-0916-4655-8353-aaf435d562a5"));
    let new_local = UserAction::NewGameAction(NewGameAction {
        deck,
        opponent: PlayerType::Human(opponent_id),
        opponent_deck: deck,
        debug_options: NewGameDebugOptions {
            override_game_id: None,
            configuration: DebugConfiguration {
                reveal_all_cards: true,
                act_as_player: Some(DebugActAsPlayer { id: opponent_id, name: PlayerName::Two }),
            },
        },
    });
    let new_ai = UserAction::NewGameAction(NewGameAction {
        deck,
        opponent: PlayerType::Agent(GameAgent {
            search_duration: Duration::from_secs(3),
            agent_type: AgentType::MonteCarlo(MonteCarloAgent {
                child_score_algorithm: ChildScoreAlgorithm::Uct1,
                max_iterations: None,
            }),
            state_predictor: StatePredictor::Omniscient,
            state_combiner: StateCombiner::First,
            evaluator: AgentEvaluator::RandomPlayout(Box::new(AgentEvaluator::WinLoss)),
            prompt_agent_reference: None,
            game_agent_reference: None,
        }),
        opponent_deck: deck,
        debug_options: NewGameDebugOptions {
            override_game_id: None,
            configuration: DebugConfiguration { reveal_all_cards: true, act_as_player: None },
        },
    });

    let buttons = vec![
        GameButtonView::new_primary("vs Local", new_local),
        GameButtonView::new_primary("vs AI", new_ai),
        GameButtonView::new_default("Codex", UserAction::QuitGameAction),
        GameButtonView::new_default("Community", UserAction::QuitGameAction),
        GameButtonView::new_default("Settings", UserAction::QuitGameAction),
        GameButtonView::new_default("Quit", UserAction::QuitGameAction),
    ];
    MainMenuView { buttons }
}
