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

use std::collections::HashMap;
use std::sync::{Arc, Mutex, MutexGuard};
use std::time::{Duration, Instant};

use data::actions::game_action::{CombatAction, GameAction};
use data::actions::prompt_action::PromptAction;
use data::card_states::zones::ZoneQueries;
use data::core::primitives::{CardId, GameId, PlayerName};
use data::game_states::game_state::GameState;
use data::game_states::game_step::GamePhaseStep;
use data::player_states::player_state::{PlayerQueries, PlayerType};
use data::prompts::select_order_prompt::CardOrderLocation;
use data::users::user_state::UserState;
use database::sqlite_database::SqliteDatabase;
use display::commands::field_state::{FieldKey, FieldValue};
use display::commands::scene_identifier::SceneIdentifier;
use display::core::card_view::ClientCardId;
use display::core::display_state::DisplayState;
use display::rendering::render;
use enumset::{enum_set, EnumSet};
use once_cell::sync::Lazy;
use rules::action_handlers::actions::ExecuteAction;
use rules::action_handlers::prompt_actions::PromptExecutionResult;
use rules::action_handlers::{actions, prompt_actions};
use rules::legality::legal_actions;
use rules::legality::legal_actions::LegalActions;
use rules::queries::combat_queries;
use tokio::sync::mpsc;
use tokio::sync::mpsc::UnboundedSender;
use tokio::task;
use tracing::{debug, error, info, instrument};
use utils::outcome::HaltCondition;
use uuid::Uuid;

use crate::requests;
use crate::server_data::{Client, ClientData, GameResponse};

static DISPLAY_STATE: Lazy<Mutex<DisplayState>> = Lazy::new(|| Mutex::new(DisplayState::default()));

/// Connects to an ongoing game scene, returning a [GameResponse] which renders
/// its current visual state.
#[instrument(level = "debug", skip_all)]
pub fn connect(
    database: SqliteDatabase,
    response_channel: UnboundedSender<GameResponse>,
    user: &UserState,
    game_id: GameId,
) {
    let game = requests::fetch_game(database, game_id, None);
    let player_name = game.find_player_name(user.id);

    info!(?user.id, ?game.id, "Connected to game");
    let commands = render::connect(&game, player_name, &get_display_state());
    let client = Client {
        data: ClientData {
            user_id: user.id,
            scene: SceneIdentifier::Game(game.id),
            id: Uuid::new_v4(),
        },
        channel: response_channel,
    };
    client.send_all(commands);
}

#[instrument(level = "debug", skip(database, client))]
pub async fn handle_game_action(database: SqliteDatabase, client: &mut Client, action: GameAction) {
    let (sender, mut receiver) = mpsc::unbounded_channel();
    assert!(
        get_display_state().prompt.is_none(),
        "Cannot handle action {action:?} with an active prompt"
    );

    let mut action_client = client.clone();
    task::spawn_blocking(move || {
        let mut game =
            requests::fetch_game(database.clone(), action_client.data.game_id(), Some(sender));
        handle_game_action_internal(database, &mut action_client, action, &mut game);
    });

    while let Some(update) = receiver.recv().await {
        if let Some(prompt) = update.prompt.as_ref() {
            let kind = prompt.prompt_type.kind();
            info!(immediate = true, ?kind, "Awaiting prompt response")
        }
        let mut display_state = get_display_state();
        display_state.prompt = update.prompt;
        display_state.prompt_channel = update.response_channel;
        send_updates(&update.game, client, &display_state);
        display_state.game_snapshot = Some(update.game);
    }
}

#[instrument(level = "debug", skip(client))]
pub fn handle_prompt_action(client: &mut Client, action: PromptAction) {
    let mut display_state = get_display_state();
    let mut prompt = display_state.prompt.take().expect("No active prompt");
    match prompt_actions::execute(prompt, action) {
        PromptExecutionResult::Prompt(prompt) => {
            display_state.prompt = Some(prompt);
        }
        PromptExecutionResult::PromptResponse(response) => {
            let channel = display_state.prompt_channel.take().expect("No active prompt channel");
            display_state.prompt = None;
            let kind = response.kind();
            debug!(?kind, "Sending prompt response");
            channel.send(response).expect("Failed to send prompt response");
        }
    }
}

pub fn handle_update_field(
    database: SqliteDatabase,
    client: &mut Client,
    key: FieldKey,
    value: FieldValue,
) {
    let mut display_state = get_display_state();
    display_state.fields.insert(key, value);
    let mut game = requests::fetch_game(database.clone(), client.data.game_id(), None);
    send_updates(&game, client, &display_state);
}

pub fn handle_drag_card(
    database: SqliteDatabase,
    client: &mut Client,
    card_id: CardId,
    location: CardOrderLocation,
    index: u32,
) {
    info!(?card_id, ?location, "handle_drag_card");
    let mut display_state = get_display_state();
    let prompt = display_state.prompt.take().expect("No active prompt");
    let result = prompt_actions::execute(
        prompt,
        PromptAction::SelectOrder(card_id, location, index as usize),
    );
    let PromptExecutionResult::Prompt(prompt) = result else {
        panic!("Expected prompt result");
    };
    display_state.prompt = Some(prompt);
    let game = display_state.game_snapshot.as_ref().expect("No game snapshot saved");
    send_updates(game, client, &display_state);
}

pub fn handle_game_action_internal(
    database: SqliteDatabase,
    client: &mut Client,
    action: GameAction,
    game: &mut GameState,
) {
    let mut current_player = game.find_player_name(client.data.user_id);

    if let Some(act_as) = game.configuration.debug.act_as_player {
        // Override player we are acting as for debugging purposes
        if Some(act_as.name) == legal_actions::next_to_act(game, None) {
            current_player = act_as.name;
        }
    }

    let mut current_action = action;
    let mut skip_undo_tracking = false;

    {
        // We immediately send an update back to the client with 'forbid actions' set,
        // in order to clear active buttons & ui actions because `actions::execute` can
        // take a long time to run (e.g. if it results in the AI being prompted to make
        // a choice).
        let mut display_state = get_display_state();
        display_state.forbid_actions = true;
        send_updates(game, client, &display_state);
        display_state.forbid_actions = false;
    }

    loop {
        let result = actions::execute(game, current_player, current_action, ExecuteAction {
            skip_undo_tracking,
            validate: true,
        });
        if result == Err(HaltCondition::Cancel) {
            // Halt current user action, roll back UI to previous state.
            let mut display_state = get_display_state();
            display_state.prompt = None;
            let original = requests::fetch_game(database.clone(), client.data.game_id(), None);
            send_updates(&original, client, &display_state);
            break;
        }

        send_updates(game, client, &get_display_state());

        let Some(next_player) = legal_actions::next_to_act(game, None) else {
            break;
        };

        if let Some(action) = auto_pass_action(game, next_player) {
            debug!(?next_player, "Automatically passing");
            current_player = next_player;
            current_action = action;
            skip_undo_tracking = true;
        } else {
            match &game.player(next_player).player_type {
                PlayerType::Human(_) | PlayerType::None => {
                    database.write_game(game);
                    break;
                }
                PlayerType::Agent(agent) => {
                    debug!(?next_player, "Searching for AI action");
                    current_player = next_player;
                    current_action = agent.implementation().select_action(game, current_player);
                    skip_undo_tracking = true;
                    debug!(?next_player, ?current_action, "AI action selected");
                }
            }
        }
    }
}

fn send_updates(game: &GameState, client: &mut Client, display_state: &DisplayState) {
    let user_player_name = game.find_player_name(client.data.user_id);
    let commands = render::render_updates(game, user_player_name, display_state);
    client.send_all(commands);
}

fn get_display_state() -> MutexGuard<'static, DisplayState> {
    DISPLAY_STATE.lock().expect("Mutex is poisoned")
}

const ALWAYS_STOP_ACTIVE: EnumSet<GamePhaseStep> =
    enum_set!(GamePhaseStep::PreCombatMain | GamePhaseStep::PostCombatMain);
const ALWAYS_STOP_INACTIVE: EnumSet<GamePhaseStep> = enum_set!(GamePhaseStep::EndStep);

const ATTACK_STEPS: EnumSet<GamePhaseStep> = enum_set!(
    GamePhaseStep::DeclareAttackers
        | GamePhaseStep::DeclareBlockers
        | GamePhaseStep::FirstStrikeDamage
        | GamePhaseStep::CombatDamage
        | GamePhaseStep::EndCombat
);

/// Returns a game action (like passing priority) a player should automatically
/// take based on their stops and other configured options.
pub fn auto_pass_action(game: &GameState, player: PlayerName) -> Option<GameAction> {
    let is_active_player = game.turn.active_player == player;
    let empty_combat = game
        .combat
        .as_ref()
        .and_then(|c| c.confirmed_attackers())
        .map(|attackers| attackers.is_empty())
        .unwrap_or_default();

    if legal_actions::can_pass_priority(game, player) {
        if game.stack().is_empty() {
            if (is_active_player
                && !game.player(player).options.active_turn_stops.contains(game.step))
                || (!is_active_player
                    && !game.player(player).options.inactive_turn_stops.contains(game.step))
            {
                // Stack is empty with no stop set, automatically pass
                return Some(GameAction::PassPriority);
            }

            if ATTACK_STEPS.contains(game.step) && empty_combat {
                // A stop is set for combat, but there are no attackers, automatically pass
                return Some(GameAction::PassPriority);
            }

            if game.player(player).options.auto_pass
                && legal_actions::compute(game, player, LegalActions { for_human_player: true })
                    .len()
                    <= 1
                && !(is_active_player && ALWAYS_STOP_ACTIVE.contains(game.step)
                    || !is_active_player && ALWAYS_STOP_INACTIVE.contains(game.step))
            {
                // No possible actions and we're not in an "always stop" step, automatically
                // pass
                return Some(GameAction::PassPriority);
            }
        } else if game.player(player).options.auto_pass
            && legal_actions::compute(game, player, LegalActions { for_human_player: true }).len()
                <= 1
        {
            // No response available to item on stack, automatically pass
            return Some(GameAction::PassPriority);
        }
    }

    if legal_actions::can_take_action(
        game,
        player,
        &GameAction::CombatAction(CombatAction::ConfirmAttackers),
    ) && combat_queries::legal_attackers(game, player).next().is_none()
    {
        // No attacks available
        return Some(GameAction::CombatAction(CombatAction::ConfirmAttackers));
    }

    if legal_actions::can_take_action(
        game,
        player,
        &GameAction::CombatAction(CombatAction::ConfirmBlockers),
    ) && (combat_queries::legal_blockers(game, player).next().is_none() || empty_combat)
    {
        // No blocks available
        return Some(GameAction::CombatAction(CombatAction::ConfirmBlockers));
    }

    if legal_actions::can_take_action(
        game,
        player,
        &GameAction::CombatAction(CombatAction::ConfirmBlockerOrder),
    ) {
        // No blockers require ordering
        return Some(GameAction::CombatAction(CombatAction::ConfirmBlockerOrder));
    }

    None
}
