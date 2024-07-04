//! GENERATED CODE - DO NOT MODIFY

use enumset::EnumSet;

use crate::core::primitives::{AbilityId, Zone};
use crate::delegates::game_delegates::GameDelegates;
use crate::delegates::stores_delegates::StoresDelegates;

pub fn run(delegates: &mut GameDelegates, id: AbilityId, zones: EnumSet<Zone>) {
    delegates.state_triggered_ability.apply_writes(id, zones);
    delegates.permanent_controller_changed.apply_writes(id, zones);
    delegates.can_attack_target.apply_writes(id, zones);
    delegates.can_be_blocked.apply_writes(id, zones);
    delegates.has_haste.apply_writes(id, zones);
    delegates.has_flying.apply_writes(id, zones);
    delegates.power.apply_writes(id, zones);
    delegates.base_power.apply_writes(id, zones);
    delegates.toughness.apply_writes(id, zones);
    delegates.base_toughness.apply_writes(id, zones);
    delegates.colors.apply_writes(id, zones);
    delegates.creature_subtypes.apply_writes(id, zones);
}
