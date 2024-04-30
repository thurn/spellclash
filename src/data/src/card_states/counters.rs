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

use serde::{Deserialize, Serialize};

use crate::core::numerics::Loyalty;

/// Represents counters currently on a card or player
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Counters {
    /// The quantity of +1/+1 counters on this object
    pub p1p1: u32,
    /// The quantity of -1/-1 counters on this object
    pub m1m1: u32,
    /// The quantity of Loyalty counters on this object,
    pub loyalty: Loyalty,
    /// Quantity of counters other than the above options on this player
    pub other_counters: HashMap<CounterType, u32>,
}

#[derive(Debug, Clone, Copy, Hash, Eq, PartialEq, Serialize, Deserialize)]
pub enum CounterType {
    Acorn,
    Aegis,
    Age,
    Aim,
    Arrow,
    Arrowhead,
    Awakening,
    Blaze,
    Blessing,
    Blood,
    Bloodline,
    Bloodstain,
    Book,
    Bore,
    Bounty,
    Bribery,
    Brick,
    Burden,
    Cage,
    Carrion,
    Charge,
    Chip,
    Chorus,
    Coin,
    Collection,
    Component,
    Contested,
    Corpse,
    Corruption,
    Credit,
    Croak,
    Crystal,
    Cube,
    Currency,
    Death,
    Deathtouch,
    Defense,
    Delay,
    Depletion,
    Descent,
    Despair,
    Devotion,
    Discovery,
    Divinity,
    Doom,
    DoubleStrike,
    Dread,
    Dream,
    Echo,
    Egg,
    Elixir,
    Ember,
    Energy,
    Enlightened,
    Eon,
    Experience,
    Eyeball,
    Fade,
    Fate,
    Feather,
    Fetch,
    Filibuster,
    Finality,
    FirstStrike,
    Flame,
    Flood,
    Flying,
    Foreshadow,
    Funk,
    Fury,
    Fungus,
    Fuse,
    Gem,
    Ghostform,
    Globe,
    Glyph,
    Gold,
    Growth,
    Harmony,
    Haste,
    Hatchling,
    Healing,
    Hexproof,
    Hit,
    Hoofprint,
    Hone,
    Hope,
    Hour,
    Hourglass,
    Hunger,
    Ice,
    Impostor,
    Incarnation,
    Indestructible,
    Infection,
    Influence,
    Ingenuity,
    Intel,
    Intervention,
    Invitation,
    Isolation,
    Javelin,
    Judgment,
    Knowledge,
    Ki,
    Kick,
    Landmark,
    Level,
    Lifelink,
    Loot,
    Lore,
    Luck,
    Manifestation,
    Mannequin,
    Matrix,
    Menace,
    M0m1,
    M0m2,
    M1m0,
    M2m1,
    M2m2,
    Mine,
    Mining,
    Mire,
    Music,
    Muster,
    Necrodermis,
    Net,
    Night,
    Oil,
    Omen,
    Ore,
    P0p1,
    P1p0,
    P1p2,
    P2p2,
    Page,
    Pain,
    Palliation,
    Paralyzation,
    Petal,
    Petrification,
    Phylactery,
    Phyresis,
    Pin,
    Plague,
    Plot,
    Polyp,
    Point,
    Poison,
    Pressure,
    Prey,
    Pupa,
    Rad,
    Reach,
    Rejection,
    Repair,
    Reprieve,
    Ribbon,
    Ritual,
    Rope,
    Rust,
    Quest,
    Silver,
    Scream,
    Shadow,
    Shell,
    Shield,
    Shred,
    Skewer,
    Sleep,
    Slime,
    Slumber,
    Soot,
    Soul,
    Spite,
    Spore,
    Stash,
    Storage,
    Story,
    Strife,
    Study,
    Stun,
    Suspect,
    Task,
    Theft,
    Tide,
    Time,
    Tower,
    Training,
    Trample,
    Trap,
    Treasure,
    Unity,
    Unlock,
    Valor,
    Velocity,
    Verse,
    Vigilance,
    Vitality,
    Void,
    Vortex,
    Vow,
    Voyage,
    Wage,
    Winch,
    Wind,
    Wish,
}
