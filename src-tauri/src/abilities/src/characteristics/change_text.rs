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

use std::io::Read;

use data::card_states::card_state::ControlChangingEffect;
use data::core::primitives::{Color, PlayerName, COLORS};
use data::delegates::game_delegates::GameDelegates;
use data::delegates::layer::Layer;
use data::delegates::query_value::{ChangeText, EnumSets};
use data::game_states::effect_state::EffectState;
use data::game_states::game_state::GameState;
use data::printed_cards::card_subtypes::{LandSubtype, BASIC_LANDS};
use data::text_strings::Text;
use either::Either;
use rules::prompt_handling::prompts;
use rules::queries::query_extension::QueryExt;

pub type LandSubtypesOrColors = Either<(LandSubtype, LandSubtype), (Color, Color)>;

pub fn choose_basic_land_types_or_colors(
    game: &mut GameState,
    controller: PlayerName,
) -> LandSubtypesOrColors {
    let choices = BASIC_LANDS
        .iter()
        .map(Either::Left)
        .chain(COLORS.iter().map(Either::Right))
        .collect::<Vec<_>>();
    let old = prompts::multiple_choice(game, controller, Text::SelectTypeToChange, choices);
    match old {
        Either::Left(old_subtype) => {
            let new_subtype = prompts::multiple_choice(
                game,
                controller,
                Text::SelectNewType,
                BASIC_LANDS.iter().filter(|&subtype| subtype != old_subtype).collect(),
            );
            Either::Left((old_subtype, new_subtype))
        }
        Either::Right(old_color) => {
            let new_color = prompts::multiple_choice(
                game,
                controller,
                Text::SelectNewType,
                COLORS.iter().filter(|&color| color != old_color).collect(),
            );
            Either::Right((old_color, new_color))
        }
    }
}

/// Prompts the player to choose two basic land subtypes to swap.
pub fn choose_basic_land_types(
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
pub fn change_basic_land_type_or_color(
    delegates: &mut GameDelegates,
    state: &'static EffectState<LandSubtypesOrColors>,
) {
    delegates.change_land_subtype_text.this_turn(|g, c, _| {
        if let Either::Left((old, new)) = state.get(g, c.effect_id)? {
            ChangeText::replace(c, old, new)
        } else {
            None
        }
    });
    delegates.land_subtypes.this_turn(|g, c, _| {
        if let Either::Left((old, new)) = state.get(g, c.effect_id)? {
            EnumSets::replace(Layer::TextChangingEffects, c, old, new)
        } else {
            None
        }
    });
    delegates.change_color_text.this_turn(|g, c, _| {
        if let Either::Right((old, new)) = state.get(g, c.effect_id)? {
            ChangeText::replace(c, old, new)
        } else {
            None
        }
    });
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
