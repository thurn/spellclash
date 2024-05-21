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

use ai::core::ai_action;
use data::actions::game_action::{CombatAction, GameAction};
use data::card_states::zones::ZoneQueries;
use data::core::primitives::{GameId, PlayerName};
use data::game_states::game_state::GameState;
use data::game_states::game_step::GamePhaseStep;
use data::player_states::player_state::PlayerQueries;
use data::users::user_state::UserState;
use database::database::Database;
use display::commands::display_preferences::DisplayPreferences;
use display::commands::scene_identifier::SceneIdentifier;
use display::rendering::render;
use enumset::{enum_set, EnumSet};
use rules::action_handlers::actions;
use rules::action_handlers::actions::PlayerType;
use rules::legality::legal_actions;
use rules::queries::combat_queries;
use tracing::{debug, error, info, instrument};
use utils::outcome::Value;
use utils::with_error::WithError;

use crate::requests;
use crate::server_data::{ClientData, GameResponse};

/// Connects to an ongoing game scene, returning a [GameResponse] which renders
/// its current visual state.
#[instrument(level = "debug", skip(database))]
pub async fn connect(
    database: Arc<dyn Database>,
    user: &UserState,
    game_id: GameId,
) -> Value<GameResponse> {
    let game = requests::fetch_game(database, game_id).await?;
    let player_name = game.find_player_name(user.id)?;

    info!(?user.id, ?game.id, "Connected to game");
    let commands = render::connect(&game, player_name, DisplayPreferences::default());
    let client_data = ClientData {
        user_id: user.id,
        scene: SceneIdentifier::Game(game.id),
        modal_panel: None,
        display_preferences: DisplayPreferences::default(),
    };
    Ok(GameResponse::new(client_data).commands(commands))
}

#[instrument(level = "debug", skip(database))]
pub async fn handle_game_action(
    database: Arc<dyn Database>,
    data: ClientData,
    action: GameAction,
) -> Value<GameResponse> {
    let mut game = requests::fetch_game(
        database.clone(),
        data.game_id().with_error(|| "Expected current game ID")?,
    )
    .await?;
    let result = handle_game_action_internal(database, &data, action, &mut game).await;
    if result.is_err() {
        error!(?action, "Error running game action loop");
    }
    result
}

pub async fn handle_game_action_internal(
    database: Arc<dyn Database>,
    data: &ClientData,
    action: GameAction,
    game: &mut GameState,
) -> Value<GameResponse> {
    let user_player_name = game.find_player_name(data.user_id)?;
    let mut current_player = user_player_name;

    if let Some(act_as) = game.configuration.debug.act_as_player {
        // Override player we are acting as for debugging purposes
        if act_as.name == legal_actions::next_to_act(game) {
            current_player = act_as.name;
        }
    }

    let mut current_action = action;
    let mut current_action_is_automatic = false;
    let mut result = GameResponse::new(data.clone());

    loop {
        actions::execute(game, current_player, current_action, current_action_is_automatic)?;
        let user_result = render::render_updates(game, user_player_name, data.display_preferences);
        result = result.commands(user_result);
        let next_player = legal_actions::next_to_act(game);
        if let Some(action) = auto_pass_action(game, next_player) {
            debug!(?next_player, "Automatically passing");
            current_player = next_player;
            current_action = action;
            current_action_is_automatic = true;
        } else if game.player(next_player).user_id.is_some() {
            database.write_game(game).await?;
            return Ok(result);
        } else {
            debug!(?next_player, "Searching for AI action");
            current_player = next_player;
            current_action = ai_action::select(game, next_player).await?;
            current_action_is_automatic = true;
            debug!(?next_player, ?current_action, "AI action selected");
        }
    }
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
                && legal_actions::compute(game, player).len() <= 1
                && !(is_active_player && ALWAYS_STOP_ACTIVE.contains(game.step)
                    || !is_active_player && ALWAYS_STOP_INACTIVE.contains(game.step))
            {
                // No possible actions and we're not in an "always stop" step, automatically
                // pass
                return Some(GameAction::PassPriority);
            }
        } else if game.player(player).options.auto_pass
            && legal_actions::compute(game, player).len() <= 1
        {
            // No response available to item on stack, automatically pass
            return Some(GameAction::PassPriority);
        }
    }

    if legal_actions::can_take_action(game, player, CombatAction::ConfirmAttackers)
        && combat_queries::legal_attackers(game, player).next().is_none()
    {
        // No attacks available
        return Some(GameAction::CombatAction(CombatAction::ConfirmAttackers));
    }

    if legal_actions::can_take_action(game, player, CombatAction::ConfirmBlockers)
        && (combat_queries::legal_blockers(game, player).next().is_none() || empty_combat)
    {
        // No blocks available
        return Some(GameAction::CombatAction(CombatAction::ConfirmBlockers));
    }

    if legal_actions::can_take_action(game, player, CombatAction::ConfirmBlockerOrder) {
        // No blockers require ordering
        return Some(GameAction::CombatAction(CombatAction::ConfirmBlockerOrder));
    }

    None
}
