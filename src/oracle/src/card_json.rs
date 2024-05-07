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
use data::core::numerics::ManaValue;
use data::core::primitives::{CardSupertype, CardType, Color, ManaColor};
use data::printed_cards::card_subtypes::CardSubtypes;
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

use crate::set_card::SetCard;

pub static CARDS: Lazy<Value<HashMap<CardName, PrintedCard>>> = Lazy::new(build_cards);
static JSON: &str = include_str!("./cards.json");

fn build_cards() -> Value<HashMap<CardName, PrintedCard>> {
    let set_cards: Vec<SetCard> = de::from_str(JSON).expect("Error deserializing cards.json");
    let mut result = HashMap::new();
    for card in &set_cards {
        let (name, printed) = build_printed_card(card)?;
        result.insert(name, printed);
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
        name: card.face_name.clone().unwrap_or_else(|| card.name.clone()),
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
        mana_value: ManaValue(card.mana_value.round() as u64),
        power: power(card.power.as_ref()),
        toughness: toughness(card.toughness.as_ref()),
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
            let value: Value<CardSupertype> = serde_json::from_str(&format!("\"{s}\""))
                .with_error(|| format!("Error deserializing supertype {s}"));
            value
        })
        .collect()
}

fn types(types: &[String]) -> Value<EnumSet<CardType>> {
    types
        .iter()
        .map(|s| {
            let value: Value<CardType> = serde_json::from_str(&format!("\"{s}\""))
                .with_error(|| format!("Error deserializing card type {s}"));
            value
        })
        .collect()
}

fn subtypes(_types: &[String]) -> Value<CardSubtypes> {
    Ok(CardSubtypes::default())
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

    println!("Mana Cost: {:?}", result);
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

fn power(_power: Option<&String>) -> Option<PrintedPower> {
    None
}

fn toughness(_toughness: Option<&String>) -> Option<PrintedToughness> {
    None
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
