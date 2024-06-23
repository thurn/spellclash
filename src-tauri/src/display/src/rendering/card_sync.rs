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
use data::card_states::card_state::{CardFacing, CardState, TappedState};
use data::card_states::zones::ZoneQueries;
use data::core::primitives::{PlayerName, Source};
use data::game_states::game_state::GameState;
use data::printed_cards::printed_card::{Face, PrintedCardFace};
use data::printed_cards::printed_card_id::PrintedCardId;
use data::prompts::prompt::{Prompt, PromptType};
use data::prompts::select_order_prompt::CardOrderLocation;
use rules::legality::legal_actions;
use rules::play_cards::play_card;
use rules::queries::combat_queries;
use rules::queries::combat_queries::CombatRole;

use crate::core::card_view::{
    CardView, ClientCardId, RevealedCardFace, RevealedCardStatus, RevealedCardView,
};
use crate::core::object_position::ObjectPosition;
use crate::core::response_builder::ResponseBuilder;
use crate::rendering::card_view_context::CardViewContext;
use crate::rendering::positions;

/// Builds a display representation of the state of a single card or card-like
/// object
pub fn card_view(builder: &ResponseBuilder, context: &CardViewContext) -> CardView {
    let is_revealed = context
        .query_or(true, |_, card| card.revealed_to.contains(builder.display_as_player()))
        || builder.response_state.reveal_all_cards;
    CardView {
        id: ClientCardId::new(context.card_id()),
        position: context.query_or(ObjectPosition::default(), |game, card| {
            positions::calculate(builder, game, card)
        }),
        card_back: "https://i.imgur.com/gCqKv0M.png".to_string(),
        revealed: is_revealed.then(|| RevealedCardView {
            image: card_image(context.printed_card_id(), context.image_face()),
            face: card_face(&context.printed().face),
            status: context.query_or(None, |game, card| card_status(builder, game, card)),
            is_ability: false,
            click_action: context.query_or(None, |game, card| card_action(builder, game, card)),
            can_drag: context.query_or(false, |game, card| can_drag(builder, game, card)),
            face_b: context.printed().face_b.as_ref().map(card_face),
            layout: context.printed().layout,
        }),
        revealed_to_opponents: context
            .query_or(false, |_, card| !card.zone.is_public() && card.revealed_to.len() > 1),
        card_facing: context.query_or(CardFacing::FaceUp(Face::Primary), |_, card| card.facing),
        tapped_state: context.query_or(TappedState::Untapped, |_, card| card.tapped_state),
        damage: Default::default(),
        create_position: if builder.response_state.animate {
            context.query_or_none(|_, card| {
                positions::for_card(card, positions::deck(builder, card.owner))
            })
        } else {
            None
        },
        destroy_position: context.query_or_none(|_, card| {
            positions::for_card(card, positions::deck(builder, card.owner))
        }),
    }
}

fn card_face(printed: &PrintedCardFace) -> RevealedCardFace {
    RevealedCardFace {
        name: printed.displayed_name.clone(),
        layout: printed.layout,
        rules_text: printed.oracle_text.clone(),
    }
}

fn card_status(
    builder: &ResponseBuilder,
    game: &GameState,
    card: &CardState,
) -> Option<RevealedCardStatus> {
    if let Some(prompt) = builder.current_prompt() {
        return prompt_card_status(builder, game, prompt, card);
    }

    if play_card::can_play_card(game, builder.act_as_player(game), Source::Game, card.id)
        && builder.allow_actions()
    {
        Some(RevealedCardStatus::CanPlay)
    } else {
        match combat_queries::role(game, card.permanent_id()?) {
            None => None,
            Some(CombatRole::SelectedAttacker) => {
                Some(RevealedCardStatus::Attacking("SA".to_string()))
            }
            Some(CombatRole::ProposedAttacker(target)) => {
                Some(RevealedCardStatus::Attacking(format!("PA@{:?}", target)))
            }
            Some(CombatRole::Attacker(target)) => {
                Some(RevealedCardStatus::Attacking(format!("A@{:?}", target)))
            }
            Some(CombatRole::SelectedBlocker) => {
                Some(RevealedCardStatus::Attacking("SB".to_string()))
            }
            Some(CombatRole::ProposedBlocker(attacker)) => Some(RevealedCardStatus::Blocking(
                format!("B@{:?}", game.card(attacker)?.printed().face.displayed_name),
            )),
            Some(CombatRole::Blocking { attacker, order }) => Some(RevealedCardStatus::Blocking(
                format!("B@{:?}@{}", game.card(attacker)?.printed().face.displayed_name, order),
            )),
        }
    }
}

fn prompt_card_status(
    builder: &ResponseBuilder,
    game: &GameState,
    prompt: &Prompt,
    card: &CardState,
) -> Option<RevealedCardStatus> {
    if let PromptType::EntityChoice(choice) = &prompt.prompt_type {
        if choice.choices.iter().any(|c| c.entity_id == card.entity_id()) {
            return Some(RevealedCardStatus::CanSelect);
        }
    }
    None
}

fn card_action(
    builder: &ResponseBuilder,
    game: &GameState,
    card: &CardState,
) -> Option<UserAction> {
    let player = builder.act_as_player(game);

    if !builder.allow_actions() {
        return None;
    }

    if let Some(prompt) = builder.current_prompt() {
        return prompt_card_action(builder, game, prompt, card);
    }

    if play_card::can_play_card(game, player, Source::Game, card.id) {
        return Some(GameAction::ProposePlayingCard(card.id).into());
    }

    let permanent_id = card.permanent_id()?;
    if legal_actions::can_take_action(
        game,
        player,
        &GameAction::CombatAction(CombatAction::AddSelectedAttacker(permanent_id)),
    ) {
        Some(CombatAction::AddSelectedAttacker(permanent_id).into())
    } else if legal_actions::can_take_action(
        game,
        player,
        &GameAction::CombatAction(CombatAction::RemoveAttacker(permanent_id)),
    ) {
        Some(CombatAction::RemoveAttacker(permanent_id).into())
    } else if legal_actions::can_take_action(
        game,
        player,
        &GameAction::CombatAction(CombatAction::AddSelectedBlocker(permanent_id)),
    ) {
        Some(CombatAction::AddSelectedBlocker(permanent_id).into())
    } else if legal_actions::can_take_action(
        game,
        player,
        &GameAction::CombatAction(CombatAction::RemoveBlocker(permanent_id)),
    ) {
        Some(CombatAction::RemoveBlocker(permanent_id).into())
    } else if legal_actions::can_take_action(
        game,
        player,
        &GameAction::CombatAction(CombatAction::SetSelectedBlockersTarget(permanent_id)),
    ) {
        Some(CombatAction::SetSelectedBlockersTarget(permanent_id).into())
    } else {
        None
    }
}

fn prompt_card_action(
    builder: &ResponseBuilder,
    game: &GameState,
    prompt: &Prompt,
    card: &CardState,
) -> Option<UserAction> {
    if let PromptType::EntityChoice(choice) = &prompt.prompt_type {
        if choice.choices.iter().any(|c| c.entity_id == card.entity_id()) {
            return Some(PromptAction::SelectEntity(card.entity_id()).into());
        }
    }
    None
}

fn can_drag(builder: &ResponseBuilder, game: &GameState, card: &CardState) -> bool {
    if !builder.allow_actions() {
        return false;
    }

    if let Some(prompt) = &builder.response_state.display_state.prompt {
        if let PromptType::SelectOrder(select_and_order) = &prompt.prompt_type {
            return select_and_order.contains_card(card.id);
        }
    }

    false
}

pub fn card_image(card_id: PrintedCardId, face: Face) -> String {
    let id = card_id.0.to_string();
    let dir1 = id.chars().next().unwrap();
    let dir2 = id.chars().nth(1).unwrap();
    match face {
        Face::Primary => format!("https://cards.scryfall.io/large/front/{dir1}/{dir2}/{id}.jpg"),
        Face::FaceB => {
            format!("https://cards.scryfall.io/large/back/{dir1}/{dir2}/{id}.jpg")
        }
    }
}
