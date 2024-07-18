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

use std::collections::BTreeMap;

use enum_kinds::EnumKind;
use serde::{Deserialize, Serialize};
use serde_with::serde_as;

use crate::actions::game_action::GameAction;
use crate::actions::prompt_action::PromptAction;
use crate::core::primitives::PlayerName;
use crate::game_states::game_state::TurnData;
use crate::player_states::player_map::PlayerMap;
use crate::prompts::prompt::PromptResponse;

/// Records a single event which happened during this game.
#[derive(Debug, Clone, EnumKind)]
#[enum_kind(HistoryEventKind)]
pub enum HistoryEvent {
    AttackWithCreature,
}

impl HistoryEvent {
    /// Returns the [HistoryEventKind] for this event
    pub fn kind(&self) -> HistoryEventKind {
        self.into()
    }
}

/// Tuple of [TurnData] and [HistoryEvent].
#[derive(Debug, Clone)]
struct HistoryEntry {
    turn: TurnData,
    event: HistoryEvent,
}

static DEFAULT_COUNTERS: HistoryCounters = HistoryCounters { cards_drawn: 0, lands_played: 0 };

/// Counters for events that happen during a given turn. Each player has their
/// own set of counters for game events.
///
/// All counters default to 0 at start of turn. History counters should always
/// be updated as the final step in any game mutation, for example the "draw
/// cards" event should draw the cards and fire related game events *before*
/// updating the counter.
#[derive(Debug, Clone, Default)]
pub struct HistoryCounters {
    /// Cards drawn so far this turn by this player. This records the actual
    /// number of cards drawn, e.g. if a player attempts to draw from an empty
    /// deck no draw is recorded.
    pub cards_drawn: usize,
    /// Lands played so far this turn by this player.
    pub lands_played: usize,
}

/// A game action taken by a player.
#[derive(Clone, Copy, Eq, PartialEq, Hash, Debug, Ord, PartialOrd, Serialize, Deserialize)]
pub struct TakenGameAction {
    /// The game action taken
    pub action: GameAction,

    /// Whether this action should create a new 'undo' state.
    pub track_for_undo: bool,
}

/// History of events which have happened during this game.
///
/// This operates via a two-phase system where history entries are collected
/// during action resolution, but are not immediately visible in the general
/// history until they are finalized by calling [Self::write_events], usually as
/// the final step of any game action. This helps avoid confusion where events
/// added during the *current* action appear in history, which is typically not
/// desired.

#[derive(Debug, Clone, Default)]
pub struct GameHistory {
    current: Vec<HistoryEntry>,
    entries: BTreeMap<TurnData, Vec<HistoryEvent>>,
    p1_counters: BTreeMap<TurnData, HistoryCounters>,
    p2_counters: BTreeMap<TurnData, HistoryCounters>,

    /// Stores actions taken thus far in the game.
    pub player_actions: PlayerMap<Vec<TakenGameAction>>,

    /// Stores prompt responses thus far in the game.
    pub prompt_responses: PlayerMap<Vec<PromptResponse>>,
}

impl GameHistory {
    /// Returns history events in the provided turn, *before* the current game
    /// event.
    pub fn for_turn(&self, turn: TurnData) -> impl Iterator<Item = &HistoryEvent> {
        self.entries.get(&turn).into_iter().flatten()
    }

    /// Returns a reference to the [HistoryCounters] entry for the provided
    /// turn.
    pub fn counters_for_turn(&self, turn: TurnData, player: PlayerName) -> &HistoryCounters {
        match player {
            PlayerName::One => self.p1_counters.get(&turn).unwrap_or(&DEFAULT_COUNTERS),
            PlayerName::Two => self.p2_counters.get(&turn).unwrap_or(&DEFAULT_COUNTERS),
            _ => todo!("Not implemented"),
        }
    }

    /// Returns a mutable reference to the [HistoryCounters] entry for the
    /// provided turn.
    pub fn counters_for_turn_mut(
        &mut self,
        turn: TurnData,
        player: PlayerName,
    ) -> &mut HistoryCounters {
        match player {
            PlayerName::One => self.p1_counters.entry(turn).or_default(),
            PlayerName::Two => self.p2_counters.entry(turn).or_default(),
            _ => todo!("Not implemented"),
        }
    }

    /// Adds a new history entry to the 'current events' buffer. Events do
    /// not appear in the [Self::for_turn] history until they are finalized by
    /// calling [Self::write_events], which typically happens as the last step
    /// in processing a game action.
    pub fn add_event(&mut self, turn: TurnData, event: HistoryEvent) {
        self.current.push(HistoryEntry { turn, event })
    }

    /// Writes all stored history events to the game history and clears the
    /// 'current events' buffer.
    pub fn write_events(&mut self) {
        for entry in &self.current {
            self.entries.entry(entry.turn).or_default().push(entry.event.clone());
        }

        self.current.clear();
    }
}
