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

use color_eyre::eyre::bail;
use color_eyre::Result;
use data::actions::new_game_action::NewGameAction;
use data::card_definitions::card_name;
use data::card_states::card_kind::CardKind;
use data::card_states::zones::Zones;
use data::core::numerics::LifeValue;
use data::core::primitives::{Color, GameId, PlayerName, UserId, Zone};
use data::decks::deck::Deck;
use data::decks::deck_name;
use data::decks::deck_name::DeckName;
use data::delegates::game_delegates::GameDelegates;
use data::effects::effect::Effect;
use data::game_states::animation_tracker::AnimationTracker;
use data::game_states::combat_state::CombatState;
use data::game_states::game_state::{GameConfiguration, GameState, GameStatus, TurnData};
use data::game_states::game_step::GamePhaseStep;
use data::game_states::history_data::GameHistory;
use data::game_states::undo_state::UndoTracker;
use data::player_states::player_state::Players;
use data::state_machines::state_machine_data::StateMachines;
use data::users::user_state::UserActivity;
use database::database::Database;
use display::commands::display_preferences::DisplayPreferences;
use display::commands::scene_name::SceneName;
use enumset::EnumSet;
use maplit::hashmap;
use rand::prelude::SliceRandom;
use rand_xoshiro::rand_core::SeedableRng;
use rand_xoshiro::Xoshiro256StarStar;
use rules::mutations::mutation;
use tracing::info;
use uuid::Uuid;

use crate::requests;
use crate::server_data::{ClientData, GameResponse};

pub async fn create(
    database: &impl Database,
    data: ClientData,
    action: NewGameAction,
) -> Result<GameResponse> {
    let mut user = requests::fetch_user(database, data.user_id).await?;

    let user_deck = find_deck(action.deck)?;
    let opponent_deck = find_deck(action.opponent_deck)?;

    let game_id = if let Some(id) = action.debug_options.override_game_id {
        id
    } else {
        GameId(Uuid::new_v4())
    };

    info!(?game_id, "Creating new game");
    let mut game = create_game(game_id, user.id, user_deck, action.opponent_id, opponent_deck);
    mutation::apply_effect(&mut game, PlayerName::One, Effect::DealOpeningHand);
    mutation::apply_effect(&mut game, PlayerName::Two, Effect::DealOpeningHand);

    user.activity = UserActivity::Playing(game.id);

    let opponent_ids = action.opponent_id.map(|o| vec![o]).unwrap_or_default();
    let opponent_responses = opponent_ids
        .iter()
        .map(|&id| (id, vec![requests::force_load_scene(SceneName::Game)]))
        .collect::<Vec<_>>();
    let result = GameResponse::new(ClientData {
        user_id: user.id,
        game_id: Some(game.id),
        display_preferences: DisplayPreferences::default(),
        opponent_ids,
    })
    .command(requests::force_load_scene(SceneName::Game))
    .opponent_responses(opponent_responses);

    database.write_game(&game).await?;
    database.write_user(&user).await?;
    if let Some(opponent_id) = action.opponent_id {
        let mut opponent = requests::fetch_user(database, opponent_id).await?;
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
) -> GameState {
    let mut rng = Xoshiro256StarStar::seed_from_u64(314159265358979323);
    let user_player_name = [PlayerName::One, PlayerName::Two].choose(&mut rng).unwrap();
    let (p1, p1_deck, p2, p2_deck) = match user_player_name {
        PlayerName::One => (Some(user_id), user_deck, opponent_id, opponent_deck),
        PlayerName::Two => (opponent_id, opponent_deck, Some(user_id), user_deck),
    };

    let mut zones = Zones::default();
    create_cards_in_deck(&mut zones, p1_deck, PlayerName::One);
    create_cards_in_deck(&mut zones, p2_deck, PlayerName::Two);

    GameState {
        id: game_id,
        status: GameStatus::Setup,
        step: GamePhaseStep::Untap,
        current_turn: TurnData { turn: PlayerName::One, turn_number: 0 },
        priority: PlayerName::One,
        configuration: GameConfiguration::default(),
        state_machines: StateMachines::default(),
        players: Players::new(p1, p2, LifeValue(20)),
        zones,
        combat: CombatState::default(),
        animations: AnimationTracker::default(),
        history: GameHistory::default(),
        rng: Xoshiro256StarStar::seed_from_u64(314159265358979323),
        undo_tracker: UndoTracker::default(),
        delegates: GameDelegates::default(),
    }
}

fn create_cards_in_deck(zones: &mut Zones, deck: Deck, owner: PlayerName) {
    for (&name, &quantity) in &deck.cards {
        for _ in 0..quantity {
            zones.create_hidden_card(name, CardKind::Normal, owner, Zone::Library);
        }
    }
}

fn find_deck(name: DeckName) -> Result<Deck> {
    Ok(match name {
        deck_name::ALL_GRIZZLY_BEARS => Deck {
            colors: EnumSet::only(Color::Green),
            cards: hashmap! {
                card_name::GRIZZLY_BEARS => 35,
                card_name::FOREST => 25
            },
        },
        _ => bail!("Unknown deck {name:?}"),
    })
}
