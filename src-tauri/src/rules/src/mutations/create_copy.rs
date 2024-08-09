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

use data::card_states::card_kind::CardKind;
use data::card_states::card_state::CardFacing;
use data::card_states::play_card_plan::PlayCardChoices;
use data::card_states::zones::{ToCardId, ZoneQueries};
use data::game_states::game_state::GameState;
use data::printed_cards::printed_card::Face;
use primitives::game_primitives::{
    CardId, HasSource, PermanentId, PlayerName, Source, SpellId, Zone,
};
use tracing::Instrument;
use utils::outcome;
use utils::outcome::Outcome;

use crate::core::initialize_card;
use crate::mutations::spells;
use crate::queries::player_queries;

/// Creates a copy of a permanent currently on the battlefield as a token owned
/// by the [PlayerName] player.
///
/// > 707.2. When copying an object, the copy acquires the copiable values of
/// > the original object's characteristics and, for an object on the stack,
/// > choices made when casting or activating it (mode, targets, the value of X,
/// > whether it was kicked, how it will affect multiple targets, and so on).
/// > The copiable values are the values derived from the text printed on the
/// > object (that text being name, mana cost, color indicator, card type,
/// > subtype, supertype, rules text, power, toughness, and/or loyalty), as
/// > modified by other copy effects, by its face-down status, and by "as . . .
/// > enters" and "as . . . is turned face up" abilities that set power and
/// > toughness (and may also set additional characteristics). Other effects
/// > (including type-changing and text-changing effects), status, counters, and
/// > stickers are not copied.
///
/// <https://yawgatog.com/resources/magic-rules/#R7072>
pub fn of_permanent(
    game: &mut GameState,
    source: impl HasSource,
    id: PermanentId,
    owner: PlayerName,
) -> Outcome {
    of_card_in_zone(
        game,
        source,
        id,
        owner,
        Zone::Battlefield,
        game.card(id)?.facing,
        game.card(id)?.cast_choices.clone(),
    )?;
    outcome::OK
}

/// Creates a copy of a card, which does not need to currently be on the
/// battlefield, as a token owned by the [PlayerName] player in the given
/// [Zone].
pub fn of_card_in_zone(
    game: &mut GameState,
    _: impl HasSource,
    id: impl ToCardId,
    owner: PlayerName,
    zone: Zone,
    facing: CardFacing,
    cast_choices: Option<PlayCardChoices>,
) -> Option<CardId> {
    let all_players = player_queries::all_players(game);
    let reference = game.oracle().card(game.card(id)?.printed_card_id);
    let turn = game.turn;
    let new_card_id =
        game.zones.create_card_in_zone(reference, zone, CardKind::TokenOrStackCopy, owner, turn);
    game.card_mut(new_card_id)?.facing = facing;
    game.card_mut(new_card_id)?.revealed_to = all_players;
    game.card_mut(new_card_id)?.cast_choices = cast_choices;
    initialize_card::run(game, new_card_id)?;
    Some(new_card_id)
}

/// Can the player creating a copy of a spell choose new targets for it?
pub enum ChooseNewTargets {
    Yes,
    No,
}

/// Creates a copy of a spell on the stack and optionally choose new targets for
/// it.
///
/// > 707.10. To copy a spell, activated ability, or triggered ability means to
/// > put a copy of it onto the stack; a copy of a spell isn't cast and a copy
/// > of an activated ability isn't activated. A copy of a spell or ability
/// > copies both the characteristics of the spell or ability and all decisions
/// > made for it, including modes, targets, the value of X, and additional or
/// > alternative costs. (See rule 601, "Casting Spells.") Choices that are
/// > normally made on resolution are not copied. If an effect of the copy
/// > refers to objects used to pay its costs, it uses the objects used to pay
/// > the costs of the original spell or ability. A copy of a spell is owned by
/// > the player under whose control it was put on the stack. A copy of a spell
/// > or ability is controlled by the player under whose control it was put on
/// > the stack. A copy of a spell is itself a spell, even though it has no
/// > spell card associated with it. A copy of an ability is itself an ability.
///
/// <https://yawgatog.com/resources/magic-rules/#R70710>
pub fn of_spell(
    game: &mut GameState,
    source: impl HasSource,
    id: SpellId,
    owner: PlayerName,
    choose_new_targets: ChooseNewTargets,
) -> Outcome {
    let source = source.source();
    let new_card_id = of_card_in_zone(
        game,
        source,
        id,
        owner,
        Zone::Stack,
        game.card(id)?.facing,
        game.card(id)?.cast_choices.clone(),
    )?;

    if let ChooseNewTargets::Yes = choose_new_targets {
        spells::choose_new_targets(game, source, owner, new_card_id)
    } else {
        outcome::OK
    }
}
