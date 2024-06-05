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

use data::actions::debug_action::DebugGameAction;
use data::actions::game_action::GameAction;
use data::card_states::zones::ZoneQueries;
use data::core::primitives::{CardId, PlayerName, Source, Zone};
use data::game_states::game_state::{GameState, GameStatus};
use data::printed_cards::printed_card::Face;
use tracing::{debug, info, instrument};

use crate::action_handlers::{combat_actions, debug_actions, prompt_actions};
use crate::legality::legal_actions;
use crate::mutations::{permanents, priority, state_based_actions};
use crate::play_cards::{pick_face_to_play, play_card};
use crate::queries::player_queries;
use crate::resolve_cards::resolve;
use crate::steps::step;

/// Options for executing a game action
#[derive(Debug, Clone, Copy)]
pub struct ExecuteAction {
    /// True if this is an automatically applied action, e.g. auto-pass
    pub automatic: bool,
    /// True if this action should be checked for legality
    pub validate: bool,
}

#[instrument(name = "actions_execute", level = "debug", skip(game))]
pub fn execute(
    game: &mut GameState,
    player: PlayerName,
    mut action: GameAction,
    options: ExecuteAction,
) {
    if options.validate {
        assert!(
            legal_actions::can_take_action(game, player, &action) || action.is_debug_action(),
            "Illegal game action {:?} for player {:?}",
            action,
            player
        );
    }

    if !options.automatic
        && game.undo_tracker.enabled
        && action != GameAction::DebugAction(DebugGameAction::Undo)
    {
        let mut clone = game.clone();
        clone.undo_tracker.enabled = false;
        clone.undo_tracker.undo = vec![];
        game.undo_tracker.undo.push(Box::new(clone));
    }

    match action {
        GameAction::DebugAction(a) => debug_actions::execute(game, player, a),
        GameAction::PassPriority => handle_pass_priority(game, player),
        GameAction::ProposePlayingCard(id) => handle_play_card(game, Source::Game, player, id),
        GameAction::CombatAction(a) => combat_actions::execute(game, player, a),
        GameAction::PromptAction(a) => prompt_actions::execute(game, player, a),
    };

    if legal_actions::can_any_player_pass_priority(game) {
        // If any player has priority as a result of this game action, check state-based
        // actions.
        state_based_actions::run(game);
    }
}

#[instrument(level = "debug", skip(game))]
fn handle_pass_priority(game: &mut GameState, player: PlayerName) {
    priority::pass(game, player)
}

#[instrument(level = "debug", skip(game))]
fn handle_play_card(game: &mut GameState, source: Source, player: PlayerName, card_id: CardId) {
    debug!(?player, ?card_id, "Playing card");
    play_card::execute(game, player, Source::Game, card_id)
}
