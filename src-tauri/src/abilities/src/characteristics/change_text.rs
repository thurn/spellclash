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
use data::card_states::zones::ZoneQueries;
use data::core::primitives::{Color, HasSource, PermanentId, PlayerName, SpellId, COLORS};
use data::delegates::delegate_type::DelegateType;
use data::delegates::game_delegates::GameDelegates;
use data::delegates::layer::Layer;
use data::delegates::query_value::{ChangeText, EnumSets};
use data::delegates::scope::EffectContext;
use data::game_states::effect_state::EffectState;
use data::game_states::game_state::GameState;
use data::printed_cards::card_subtypes::{LandType, BASIC_LANDS};
use data::properties::card_modifier::CardModifier;
use data::properties::duration::Duration;
use data::properties::query_names::{
    ChangeColorTextQuery, ChangeLandTypeTextQuery, ColorsQuery, LandTypesQuery,
};
use data::text_strings::Text;
use either::Either;
use rules::prompt_handling::prompts;

use crate::core::effects;

pub type LandSubtypesOrColors = Either<(LandType, LandType), (Color, Color)>;

pub fn change_basic_land_types_or_colors_this_turn(
    game: &mut GameState,
    context: EffectContext,
    target: Either<SpellId, PermanentId>,
) {
    let choice = choose_basic_land_types_or_colors(game, context.controller());
    match (choice, target) {
        (LandSubtypesOrColors::Left((old_type, new_type)), Either::Left(spell_id)) => {
            change_spell_land_type_text(game, context, spell_id, new_type, old_type);
        }
        (LandSubtypesOrColors::Left((old_type, new_type)), Either::Right(permanent_id)) => {
            change_permanent_land_type_text(game, context, permanent_id, old_type, new_type);
        }
        (LandSubtypesOrColors::Right((old_color, new_color)), Either::Left(spell_id)) => {
            change_spell_color_text(game, context, spell_id, old_color, new_color);
        }
        (LandSubtypesOrColors::Right((old_color, new_color)), Either::Right(permanent_id)) => {
            change_permanent_color_text(game, context, permanent_id, old_color, new_color);
        }
    }
}

fn change_permanent_color_text(
    game: &mut GameState,
    context: EffectContext,
    permanent_id: PermanentId,
    old_color: Color,
    new_color: Color,
) {
    effects::modify_permanent_this_turn::<ChangeColorTextQuery>(
        game,
        context,
        permanent_id,
        ChangeText::replace(context.effect_id, old_color, new_color),
    );
}

fn change_spell_color_text(
    game: &mut GameState,
    context: EffectContext,
    spell_id: SpellId,
    old_color: Color,
    new_color: Color,
) {
    if let Some(card) = game.card_mut(spell_id) {
        card.properties.change_color_text.add(CardModifier {
            source: context.source(),
            duration: Duration::WhileOnStackOrBattlefield(spell_id),
            delegate_type: DelegateType::Effect,
            effect: ChangeText::replace(context.effect_id, old_color, new_color),
        });
    }
}

fn change_permanent_land_type_text(
    game: &mut GameState,
    context: EffectContext,
    permanent_id: PermanentId,
    old_type: LandType,
    new_type: LandType,
) {
    effects::modify_permanent_this_turn::<LandTypesQuery>(
        game,
        context,
        permanent_id,
        EnumSets::replace(Layer::TextChangingEffects, context.effect_id, old_type, new_type),
    );
    effects::modify_permanent_this_turn::<ChangeLandTypeTextQuery>(
        game,
        context,
        permanent_id,
        ChangeText::replace(context.effect_id, old_type, new_type),
    );
}

fn change_spell_land_type_text(
    game: &mut GameState,
    context: EffectContext,
    spell_id: SpellId,
    new_type: LandType,
    old_type: LandType,
) {
    if let Some(card) = game.card_mut(spell_id) {
        card.properties.land_types.add(CardModifier {
            source: context.source(),
            duration: Duration::WhileOnStackOrBattlefield(spell_id),
            delegate_type: DelegateType::Effect,
            effect: EnumSets::replace(
                Layer::TextChangingEffects,
                context.effect_id,
                old_type,
                new_type,
            ),
        });
        card.properties.change_land_type_text.add(CardModifier {
            source: context.source(),
            duration: Duration::WhileOnStackOrBattlefield(spell_id),
            delegate_type: DelegateType::Effect,
            effect: ChangeText::replace(context.effect_id, old_type, new_type),
        });
    }
}

fn choose_basic_land_types_or_colors(
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
