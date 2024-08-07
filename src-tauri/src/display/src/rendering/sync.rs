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

use data::actions::game_action::{CombatAction, GameAction};
use data::actions::prompt_action::PromptAction;
use data::actions::user_action::UserAction;
use data::card_states::card_state::CardState;
use data::card_states::zones::ZoneQueries;
use data::core::panel_address::GamePanelAddress;
use data::game_states::combat_state::CombatState;
use data::game_states::game_phase_step::GamePhaseStep;
use data::game_states::game_state::GameState;
use data::player_states::player_state::PlayerQueries;
use data::prompts::prompt::{Prompt, PromptType};
use data::prompts::select_order_prompt::CardOrderLocation;
use primitives::game_primitives::{PlayerName, Zone};
use rules::legality::{can_undo, legal_actions, legal_prompt_actions};

use crate::commands::field_state::FieldKey;
use crate::core::display_state::DisplayState;
use crate::core::game_view::{
    GameButtonView, GameControlView, GameView, GameViewState, PlayerView, TextInputView,
};
use crate::core::response_builder::ResponseBuilder;
use crate::rendering::card_view_context::CardViewContext;
use crate::rendering::{ability_sync, card_sync};

/// Converts a [GameState] into a series of commands inside the provided
/// [ResponseBuilder] describing the visual game state.
pub fn run(builder: &mut ResponseBuilder, game: &GameState) {
    let mut cards = game
        .zones
        .all_cards()
        .filter(|c| !skip_sending_to_client(c))
        .map(|c| card_sync::card_view(builder, &CardViewContext::Game(c.printed(), game, c)))
        .collect::<Vec<_>>();
    cards.append(
        &mut game
            .zones
            .all_stack_abilities()
            .filter_map(|a| {
                Some(ability_sync::stack_ability_view(builder, game.card(a.ability_id.card_id)?, a))
            })
            .collect(),
    );

    let display_state = builder.response_state.display_state;
    builder.push_game_view(GameView {
        viewer: player_view(display_state, game, builder.display_as_player()),
        opponent: player_view(display_state, game, match builder.display_as_player() {
            PlayerName::One => PlayerName::Two,
            PlayerName::Two => PlayerName::One,
            _ => todo!("Not implemented"),
        }),
        cards,
        status_description: format!(
            "{:?}\nTurn {}\nPlayer {:?}",
            game.step, game.turn.turn_number, game.turn.active_player
        ),
        card_drag_targets: card_drag_targets(builder, game),
        state: if game.combat.is_some() {
            GameViewState::CombatActive
        } else {
            GameViewState::None
        },
        top_controls: top_game_controls(game, builder, builder.act_as_player(game)),
        bottom_controls: bottom_game_controls(game, builder, builder.act_as_player(game)),
    });
}

fn card_drag_targets(
    response_builder: &ResponseBuilder,
    game: &GameState,
) -> Vec<CardOrderLocation> {
    if !response_builder.allow_actions() {
        return vec![];
    }

    if let Some(prompt) = &response_builder.response_state.display_state.prompt {
        if let PromptType::SelectOrder(select_and_order) = &prompt.prompt_type {
            return select_and_order.cards.keys().copied().collect();
        }
    }
    vec![]
}

fn player_view(display_state: &DisplayState, game: &GameState, player: PlayerName) -> PlayerView {
    PlayerView {
        life: game.player(player).life as f64,
        can_act: legal_actions::next_to_act(game, display_state.prompt.as_ref()) == Some(player),
    }
}

fn skip_sending_to_client(card: &CardState) -> bool {
    card.revealed_to.is_empty() && card.zone == Zone::Library
}

fn top_game_controls(
    game: &GameState,
    builder: &ResponseBuilder,
    _player: PlayerName,
) -> Vec<GameControlView> {
    if !builder.allow_actions() {
        return vec![];
    }

    let mut result = vec![
        GameButtonView::new_default("Leave Game", UserAction::LeaveGameAction),
        GameButtonView::new_default(
            "Debug",
            UserAction::OpenPanel(GamePanelAddress::GameDebugPanel.into()),
        ),
    ];
    if can_undo::can_undo(game) {
        result.push(GameButtonView::new_default("Undo", UserAction::Undo));
    }
    result.into_iter().map(GameControlView::Button).collect()
}

fn bottom_game_controls(
    game: &GameState,
    builder: &ResponseBuilder,
    player: PlayerName,
) -> Vec<GameControlView> {
    if !builder.allow_actions() {
        return vec![];
    }

    if let Some(current) = &builder.display_state().prompt {
        return prompt_view(builder.display_state(), current, player);
    }

    let mut result = vec![];
    if legal_actions::can_take_action(game, player, &GameAction::PassPriority) {
        if game.stack().is_empty() {
            let next = match game.step {
                GamePhaseStep::Upkeep => "To Draw",
                GamePhaseStep::Draw => "To Main",
                GamePhaseStep::PreCombatMain => "Continue",
                GamePhaseStep::BeginCombat => "To Attackers",
                GamePhaseStep::DeclareAttackers => "To Blockers",
                GamePhaseStep::DeclareBlockers => "To Damage",
                GamePhaseStep::FirstStrikeDamage => "To Damage",
                GamePhaseStep::CombatDamage => "End Combat",
                GamePhaseStep::EndCombat => "End Combat",
                GamePhaseStep::PostCombatMain => "End Turn",
                GamePhaseStep::EndStep => "Next Turn",
                _ => "Continue",
            };
            result.push(GameButtonView::new_primary(next, GameAction::PassPriority));
        } else {
            result.push(GameButtonView::new_primary("Resolve", GameAction::PassPriority));
        }
    }
    if legal_actions::can_take_action(
        game,
        player,
        &GameAction::CombatAction(CombatAction::ConfirmAttackers),
    ) {
        if let Some(CombatState::ProposingAttackers(attackers)) = &game.combat {
            let count = attackers.proposed_attacks.len();
            result.push(GameButtonView::new_primary(
                format!("{} Attacker{}", count, if count == 1 { "" } else { "s" }),
                CombatAction::ConfirmAttackers,
            ));
        }
    }
    if legal_actions::can_take_action(
        game,
        player,
        &GameAction::CombatAction(CombatAction::ConfirmBlockers),
    ) {
        if let Some(CombatState::ProposingBlockers(blockers)) = &game.combat {
            let count = blockers.proposed_blocks.len();
            result.push(GameButtonView::new_primary(
                format!("{} Blocker{}", count, if count == 1 { "" } else { "s" }),
                CombatAction::ConfirmBlockers,
            ));
        }
    }
    if legal_actions::can_take_action(
        game,
        player,
        &GameAction::CombatAction(CombatAction::ConfirmBlockerOrder),
    ) {
        result
            .push(GameButtonView::new_primary("Confirm Order", CombatAction::ConfirmBlockerOrder));
    }

    result.into_iter().map(GameControlView::Button).collect()
}

fn prompt_view(state: &DisplayState, prompt: &Prompt, player: PlayerName) -> Vec<GameControlView> {
    match &prompt.prompt_type {
        PromptType::EntityChoice(_) => {
            vec![GameControlView::Text("Pick Entity".to_string())]
        }
        PromptType::SelectOrder(_) => {
            if legal_prompt_actions::can_take_action(
                prompt,
                player,
                PromptAction::SubmitCardSelection,
            ) {
                return vec![GameControlView::Button(GameButtonView::new_primary(
                    "Submit",
                    PromptAction::SubmitCardSelection,
                ))];
            }

            vec![]
        }
        PromptType::PlayCards(_) => {
            vec![]
        }
        PromptType::PickNumber(pick_number) => {
            let mut result =
                vec![GameControlView::TextInput(TextInputView { key: FieldKey::PickNumberPrompt })];
            if let Some(value) = state.fields.get(&FieldKey::PickNumberPrompt) {
                if let Some(n) = value.as_u32() {
                    if legal_prompt_actions::can_take_action(
                        prompt,
                        player,
                        PromptAction::PickNumber(n),
                    ) {
                        result.push(GameControlView::Button(GameButtonView::new_primary(
                            format!("Set {}", n),
                            PromptAction::PickNumber(n),
                        )));
                    }
                }
            }

            result
        }
        PromptType::MultipleChoice(data) => {
            let mut result = vec![];
            for (i, choice) in data.choices().iter().enumerate() {
                if legal_prompt_actions::can_take_action(
                    prompt,
                    player,
                    PromptAction::SelectChoice(i),
                ) {
                    result.push(GameControlView::Button(GameButtonView::new_primary(
                        choice.to_string(),
                        PromptAction::SelectChoice(i),
                    )));
                }
            }

            result
        }
    }
}
