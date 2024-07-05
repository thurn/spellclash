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

use data::core::primitives::PlayerName;
use data::delegates::game_delegates::GameDelegates;
use data::delegates::layer::Layer;
use data::delegates::query_value::{ChangeText, EnumSets};
use data::game_states::effect_state::EffectState;
use data::game_states::game_state::GameState;
use data::printed_cards::card_subtypes::{LandSubtype, BASIC_LANDS};
use data::text_strings::Text;
use rules::prompt_handling::prompts;
use rules::queries::query_extension::QueryExt;

/// Prompts the player to choose two basic land subtypes to swap.
pub fn choose_basic_land_subtypes(
    game: &mut GameState,
    controller: PlayerName,
) -> (LandSubtype, LandSubtype) {
    let old = prompts::multiple_choice(
        game,
        controller,
        Text::SelectTypeToChange,
        BASIC_LANDS.iter().collect(),
    );
    let new = prompts::multiple_choice(
        game,
        controller,
        Text::SelectNewType,
        BASIC_LANDS.iter().filter(|&subtype| subtype != old).collect(),
    );
    (old, new)
}

/// Changes the text of a card to change instances of a basic land subtype in
/// its rules text.
pub fn change_basic_land_type(
    delegates: &mut GameDelegates,
    state: &'static EffectState<(LandSubtype, LandSubtype)>,
) {
    delegates.change_land_subtype_text.this_turn(|g, c, _| {
        let (old, new) = state.get(g, c.effect_id)?;
        ChangeText::replace(c, old, new)
    });
    delegates.land_subtypes.this_turn(|g, c, _| {
        let (old, new) = state.get(g, c.effect_id)?;
        EnumSets::replace(Layer::TextChangingEffects, c, old, new)
    });
}
