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

use data::card_states::card_state::{CardFacing, CardState};
use data::game_states::game_state::GameState;
use data::printed_cards::printed_card::{Face, PrintedCard};
use data::printed_cards::printed_card_id::PrintedCardId;
use primitives::game_primitives::CardId;

/// Provides the context in which a card view is being displayed, i.e. either
/// during an active game or in a deck editor.
pub enum CardViewContext<'a> {
    Default(&'a PrintedCard, PrintedCardId, CardId),
    Game(&'a PrintedCard, &'a GameState, &'a CardState),
}

impl<'a> CardViewContext<'a> {
    pub fn printed(&self) -> &PrintedCard {
        match self {
            Self::Default(d, _, _) => d,
            Self::Game(d, _, _) => d,
        }
    }

    pub fn game(&self) -> Option<&GameState> {
        match self {
            Self::Default(..) => None,
            Self::Game(_, game, _) => Some(game),
        }
    }

    pub fn card(&self) -> Option<&CardState> {
        match self {
            Self::Default(..) => None,
            Self::Game(_, _, card) => Some(card),
        }
    }

    /// Returns the [Face] of this card whose image should currently be
    /// displayed.
    pub fn image_face(&self) -> Face {
        match self {
            Self::Default(_, _, _) => Face::Primary,
            Self::Game(_, _, card) => match card.facing {
                CardFacing::FaceUp(Face::FaceB) => Face::FaceB,
                _ => Face::Primary,
            },
        }
    }

    pub fn card_id(&self) -> CardId {
        match self {
            Self::Default(_, _, id) => *id,
            Self::Game(_, _, card) => card.id,
        }
    }

    pub fn printed_card_id(&self) -> PrintedCardId {
        match self {
            Self::Default(_, id, _) => *id,
            Self::Game(_, _, card) => card.printed_card_id,
        }
    }

    /// Invokes the provided `game` function to produce a value in the active
    /// game context, otherwise returns some `default`.
    pub fn query_or<T>(&self, default: T, fun: impl Fn(&GameState, &CardState) -> T) -> T {
        match self {
            Self::Default(..) => default,
            Self::Game(_, state, card) => fun(state, card),
        }
    }

    /// Equivalent to `query_or` which uses `None` as the default value.
    pub fn query_or_none<T>(&self, fun: impl Fn(&GameState, &CardState) -> T) -> Option<T> {
        match self {
            Self::Default(..) => None,
            Self::Game(_, state, card) => Some(fun(state, card)),
        }
    }

    /// Equivalent to `query_or` which passed the [CardId] to the callback
    /// function.
    pub fn query_id_or<T>(&self, default: T, fun: impl Fn(&GameState, CardId) -> T) -> T {
        match self {
            Self::Default(..) => default,
            Self::Game(_, state, card) => fun(state, card.id),
        }
    }
}
