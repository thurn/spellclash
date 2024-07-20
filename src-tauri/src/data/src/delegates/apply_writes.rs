//! GENERATED CODE - DO NOT MODIFY

use enumset::EnumSet;

use crate::core::primitives::{AbilityId, Zone};
use crate::delegates::game_delegates::GameDelegates;
use crate::delegates::stores_delegates::StoresDelegates;

pub fn run(delegates: &mut GameDelegates, id: AbilityId, zones: EnumSet<Zone>) {
    delegates.will_enter_battlefield.apply_writes(id, zones);
    delegates.permanent_controller_changed.apply_writes(id, zones);
}
