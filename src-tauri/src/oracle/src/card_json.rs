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
use std::iter;

use data::card_definitions::card_name::CardName;
use data::core::primitives::{CardSupertype, CardType, Color, ManaColor};
use data::printed_cards::card_subtypes::{
    ArtifactSubtype, BattleSubtype, CardSubtypes, CreatureSubtype, DungeonSubtype,
    EnchantmentSubtype, InstantOrSorcerySubtype, LandSubtype, PlaneSubtype, PlaneswalkerSubtype,
};
use data::printed_cards::layout::{CardLayout, FaceLayout};
use data::printed_cards::mana_cost::{ManaCost, ManaCostItem};
use data::printed_cards::printed_card::{
    Face, PrintedCard, PrintedCardFace, PrintedCardFaceVariant,
};
use data::printed_cards::printed_primitives::{
    AttractionLight, PrintedLoyalty, PrintedPower, PrintedToughness,
};
use enumset::EnumSet;
use once_cell::sync::Lazy;
use regex::Regex;
use serde_json::de;
use utils::fail;
use utils::outcome::Value;
use utils::with_error::WithError;
use uuid::Uuid;

use crate::all_printings::AllPrintings;
use crate::set_card::SetCard;

pub static CARDS: Lazy<Value<HashMap<CardName, PrintedCard>>> = Lazy::new(build_cards);
static JSON: &str = include_str!("./cards.json");

fn build_cards() -> Value<HashMap<CardName, PrintedCard>> {
    let all_printings: AllPrintings = de::from_str(JSON).expect("Error deserializing cards.json");
    let mut result = HashMap::new();
    for set_cards in all_printings.data.values() {
        for card in &set_cards.cards {
            let Ok((name, printed)) = build_printed_card(card) else {
                continue;
            };
            result.insert(name, printed);
        }
    }
    Ok(result)
}

fn build_printed_card(card: &SetCard) -> Value<(CardName, PrintedCard)> {
    let name = CardName(parse_uuid(card.identifiers.scryfall_oracle_id.as_ref())?);

    let printed = PrintedCard {
        face: build_face(card, Face::Primary)?,
        face_b: None,
        layout: match card.layout {
            FaceLayout::Adventure => CardLayout::Adventure,
            FaceLayout::Aftermath => CardLayout::Aftermath,
            FaceLayout::Flip => CardLayout::Flip,
            FaceLayout::Meld => CardLayout::Meld,
            FaceLayout::ModalDfc => CardLayout::ModalDfc,
            FaceLayout::Split => CardLayout::Split,
            FaceLayout::Transform => CardLayout::Transform,
            _ => CardLayout::Normal,
        },
    };

    Ok((name, printed))
}

fn build_face(card: &SetCard, face_identifier: Face) -> Value<PrintedCardFace> {
    Ok(PrintedCardFace {
        id: card.uuid,
        displayed_name: card.face_name.clone().unwrap_or_else(|| card.name.clone()),
        face_identifier,
        variants: vec![PrintedCardFaceVariant {
            scryfall_id: parse_uuid(card.identifiers.scryfall_id.as_ref())?,
        }],
        supertypes: supertypes(&card.supertypes)?,
        card_types: types(&card.card_types)?,
        subtypes: subtypes(&card.subtypes)?,
        oracle_text: card.text.clone(),
        colors: colors(&card.colors),
        mana_cost: mana_cost(card.mana_cost.as_ref())?,
        mana_value: card.mana_value.round() as u64,
        power: power(card.power.as_ref())?,
        toughness: toughness(card.toughness.as_ref())?,
        loyalty: loyalty(card.loyalty.as_ref()),
        layout: card.layout,
        attraction_lights: attraction_lights(card.attraction_lights.clone().unwrap_or_default()),
        melds_with: None,
        has_alternative_deck_limit: card.has_alternative_deck_limit.unwrap_or_default(),
    })
}

fn supertypes(types: &[String]) -> Value<EnumSet<CardSupertype>> {
    types
        .iter()
        .map(|s| {
            s.parse::<CardSupertype>().with_error(|| format!("Error deserializing supertype {s}"))
        })
        .collect()
}

fn types(types: &[String]) -> Value<EnumSet<CardType>> {
    types
        .iter()
        .map(|s| s.parse::<CardType>().with_error(|| format!("Error deserializing supertype {s}")))
        .collect()
}

fn subtypes(types: &[String]) -> Value<CardSubtypes> {
    let mut result = CardSubtypes::default();
    for subtype in types {
        if let Ok(s) = subtype.parse::<ArtifactSubtype>() {
            result.artifact.insert(s);
        }
        if let Ok(s) = subtype.parse::<EnchantmentSubtype>() {
            result.enchantment.insert(s);
        }
        if let Ok(s) = subtype.parse::<LandSubtype>() {
            result.land.insert(s);
        }
        if let Ok(s) = subtype.parse::<PlaneswalkerSubtype>() {
            result.planeswalker.insert(s);
        }
        if let Ok(s) = subtype.parse::<InstantOrSorcerySubtype>() {
            result.instant_or_sorcery_subtype.insert(s);
        }
        if let Ok(s) = subtype.parse::<CreatureSubtype>() {
            result.creature.insert(s);
        }
        if let Ok(s) = subtype.parse::<PlaneSubtype>() {
            result.plane.insert(s);
        }
        if let Ok(s) = subtype.parse::<DungeonSubtype>() {
            result.dungeon.insert(s);
        }
        if let Ok(s) = subtype.parse::<BattleSubtype>() {
            result.battle.insert(s);
        }
    }
    Ok(result)
}

fn colors(_colors: &[mtgjson::Color]) -> EnumSet<Color> {
    EnumSet::empty()
}

fn mana_cost(cost: Option<&String>) -> Value<ManaCost> {
    let Some(cost) = cost else {
        return Ok(ManaCost::default());
    };
    let re = Regex::new(r"\{(.*?)}").with_error(|| "Invalid regex")?;
    let mut result = ManaCost::default();
    for capture in re.captures_iter(cost) {
        result
            .items
            .extend(to_mana_item(capture.get(1).with_error(|| "Expected mana symbol")?.as_str())?);
    }

    Ok(result)
}

fn to_mana_item(symbol: &str) -> Value<Vec<ManaCostItem>> {
    Ok(vec![match symbol {
        "W" => ManaCostItem::Colored(ManaColor::White),
        "U" => ManaCostItem::Colored(ManaColor::Blue),
        "B" => ManaCostItem::Colored(ManaColor::Black),
        "R" => ManaCostItem::Colored(ManaColor::Red),
        "G" => ManaCostItem::Colored(ManaColor::Green),
        _ => match symbol.parse::<usize>() {
            Ok(value) => return Ok(iter::repeat(ManaCostItem::Generic).take(value).collect()),
            Err(_) => {
                fail!("Unrecognized mana symbol {:?}", symbol);
            }
        },
    }])
}

fn power(power: Option<&String>) -> Value<Option<PrintedPower>> {
    power
        .map(|p| {
            if let Ok(value) = p.parse::<i64>() {
                Ok(PrintedPower::Number(value))
            } else {
                fail!("TODO: Implement support for non-numeric power");
            }
        })
        .transpose()
}

fn toughness(toughness: Option<&String>) -> Value<Option<PrintedToughness>> {
    toughness
        .map(|t| {
            if let Ok(value) = t.parse::<i64>() {
                Ok(PrintedToughness::Number(value))
            } else {
                fail!("TODO: Implement support for non-numeric toughness");
            }
        })
        .transpose()
}

fn loyalty(_loyalty: Option<&String>) -> Option<PrintedLoyalty> {
    None
}

fn attraction_lights(_lights: Vec<u8>) -> EnumSet<AttractionLight> {
    EnumSet::empty()
}

fn parse_uuid(id: Option<&String>) -> Value<Uuid> {
    Uuid::parse_str(id.with_error(|| "Missing UUID")?).with_error(|| "Error parsing UUID")
}
