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

use data::actions::new_game_action::NewGameAction;
use data::card_definitions::card_name;
use data::card_states::card_kind::CardKind;
use data::card_states::zones::Zones;
use data::core::numerics::LifeValue;
use data::core::primitives::{Color, GameId, PlayerName, Source, UserId};
use data::decks::deck::Deck;
use data::decks::deck_name;
use data::decks::deck_name::DeckName;
use data::delegates::game_delegates::GameDelegates;
use data::game_states::animation_tracker::AnimationTracker;
use data::game_states::game_state::{
    DebugConfiguration, GameConfiguration, GameState, GameStatus, TurnData,
};
use data::game_states::game_step::GamePhaseStep;
use data::game_states::history_data::GameHistory;
use data::game_states::undo_tracker::UndoTracker;
use data::player_states::player_state::Players;
use data::prompts::prompt_manager::PromptManager;
use data::state_machines::state_machine_data::StateMachines;
use data::users::user_state::UserActivity;
use database::database::Database;
use display::commands::display_preferences::DisplayPreferences;
use display::commands::scene_name::SceneName;
use display::rendering::render;
use enumset::EnumSet;
use maplit::hashmap;
use rand_xoshiro::rand_core::SeedableRng;
use rand_xoshiro::Xoshiro256StarStar;
use rules::mutations::library;
use rules::steps::step;
use tracing::info;
use utils::fail;
use utils::outcome::Value;
use uuid::Uuid;

use crate::server_data::{ClientData, GameResponse};
use crate::{game_action_server, requests};

pub async fn create(
    database: Arc<dyn Database>,
    data: ClientData,
    action: NewGameAction,
) -> Value<GameResponse> {
    let mut user = requests::fetch_user(database.clone(), data.user_id).await?;

    let user_deck = find_deck(action.deck)?;
    let opponent_deck = find_deck(action.opponent_deck)?;

    let game_id = if let Some(id) = action.debug_options.override_game_id {
        id
    } else {
        GameId(Uuid::new_v4())
    };

    info!(?game_id, "Creating new game");
    let mut game = create_game(
        game_id,
        user.id,
        user_deck,
        action.debug_options.configuration.act_as_player.map(|p| p.id).or(action.opponent_id),
        opponent_deck,
        action.debug_options.configuration,
    );
    requests::initialize_game(database.clone(), &mut game)?;

    game.shuffle_library(PlayerName::One)?;
    library::draw_cards(&mut game, Source::Game, PlayerName::One, 7)?;
    game.shuffle_library(PlayerName::Two)?;
    library::draw_cards(&mut game, Source::Game, PlayerName::Two, 7)?;
    step::advance(&mut game)?;
    if let Some(action) = game_action_server::auto_pass_action(&game, PlayerName::One) {
        // Pass priority until the first configured stop.
        game_action_server::handle_game_action_internal(database.clone(), &data, action, &mut game)
            .await?;
    }

    user.activity = UserActivity::Playing(game.id);

    let opponent_ids = action.opponent_id.map(|o| vec![o]).unwrap_or_default();
    let opponent_responses = opponent_ids
        .iter()
        .map(|&id| {
            let mut commands = vec![];
            commands.append(&mut render::connect(
                &game,
                game.find_player_name(id)?,
                DisplayPreferences::default(),
            ));
            Ok((id, commands))
        })
        .collect::<Result<Vec<_>, _>>()?;
    let result = GameResponse::new(SceneName::Game, ClientData {
        user_id: user.id,
        game_id: Some(game.id),
        display_preferences: DisplayPreferences::default(),
        opponent_ids,
    })
    .commands(render::connect(
        &game,
        game.find_player_name(user.id)?,
        DisplayPreferences::default(),
    ))
    .opponent_responses(opponent_responses);

    database.write_game(&game).await?;
    database.write_user(&user).await?;
    if let Some(opponent_id) = action.opponent_id {
        let mut opponent = requests::fetch_user(database.clone(), opponent_id).await?;
        opponent.activity = UserActivity::Playing(game_id);
        database.write_user(&opponent).await?;
    }

    Ok(result)
}

fn create_game(
    game_id: GameId,
    user_id: UserId,
    user_deck: Deck,
    opponent_id: Option<UserId>,
    opponent_deck: Deck,
    debug: DebugConfiguration,
) -> GameState {
    let user_player_name = PlayerName::One;
    let (p1, p1_deck, p2, p2_deck) = match user_player_name {
        PlayerName::One => (Some(user_id), user_deck, opponent_id, opponent_deck),
        PlayerName::Two => (opponent_id, opponent_deck, Some(user_id), user_deck),
        _ => todo!("Not implemented"),
    };

    let mut zones = Zones::default();
    let turn = TurnData { active_player: PlayerName::One, turn_number: 0 };
    create_cards_in_deck(&mut zones, p1_deck, PlayerName::One, turn);
    create_cards_in_deck(&mut zones, p2_deck, PlayerName::Two, turn);

    GameState {
        id: game_id,
        status: GameStatus::Setup,
        step: GamePhaseStep::Untap,
        turn,
        priority: PlayerName::One,
        passed: EnumSet::empty(),
        configuration: GameConfiguration::new(PlayerName::One | PlayerName::Two, debug),
        state_machines: StateMachines::default(),
        players: Players::new(p1, p2, 20),
        zones,
        prompts: PromptManager::default(),
        combat: None,
        animations: AnimationTracker::default(),
        history: GameHistory::default(),
        rng: Xoshiro256StarStar::seed_from_u64(3141592653589793),
        undo_tracker: UndoTracker { enabled: true, undo: vec![] },
        delegates: GameDelegates::default(),
        state_based_events: Some(vec![]),
    }
}

fn create_cards_in_deck(zones: &mut Zones, deck: Deck, owner: PlayerName, turn: TurnData) {
    for (&name, &quantity) in &deck.cards {
        for _ in 0..quantity {
            zones.create_card_in_library(name, CardKind::Normal, owner, turn);
        }
    }
}

fn find_deck(name: DeckName) -> Value<Deck> {
    Ok(match name {
        deck_name::ALL_GRIZZLY_BEARS => Deck {
            colors: EnumSet::only(Color::Green),
            cards: hashmap! {
                card_name::GRIZZLY_BEARS => 35,
                card_name::FOREST => 25
            },
        },
        _ => {
            fail!("Unknown deck {name:?}");
        }
    })
}
