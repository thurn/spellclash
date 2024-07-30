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

use std::iter;

use data::card_definitions::card_name::CardName;
use data::printed_cards::card_subtypes::{
    ArtifactSubtype, BattleSubtype, CardSubtypes, CreatureType, DungeonSubtype, EnchantmentSubtype,
    InstantOrSorcerySubtype, LandType, PlaneSubtype, PlaneswalkerSubtype,
};
use data::printed_cards::database_card::DatabaseCardFace;
use data::printed_cards::layout::{CardLayout, FaceLayout};
use data::printed_cards::mana_cost::{ManaCost, ManaCostItem};
use data::printed_cards::printed_card::{
    Face, PrintedCard, PrintedCardFace,
};
use data::printed_cards::printed_primitives::{
    PrintedPower, PrintedToughness,
};
use enumset::EnumSet;
use regex::Regex;
use primitives::game_primitives::{CardSupertype, CardType, Color, ManaColor};

/// Turns a [DatabaseCardFace] list into a [PrintedCard].
///
/// This translates the raw data coming from MTGJSON into a more useful rust
/// data structure.
///
/// Panics if the card doesn't follow the expected format.
pub fn parse(faces: Vec<DatabaseCardFace>) -> PrintedCard {
    let (primary, secondary) = if faces.is_empty() || faces.len() > 2 {
        panic!("Expected 1 or 2 card faces, got {}", faces.len());
    } else if faces.len() == 1 {
        (&faces[0], None)
    } else {
        let (Some(one), Some(two)) = (&faces[0].side, &faces[1].side) else {
            panic!("Expected 'side' designation on card face");
        };

        if one == "a" {
            (&faces[0], Some(&faces[1]))
        } else if two == "a" {
            (&faces[1], Some(&faces[0]))
        } else {
            panic!("Expected at least one face to be designated side 'a'");
        }
    };

    let name = CardName(primary.scryfall_oracle_id);
    let primary_layout = layout(&primary.layout);
    let card_layout = match primary_layout {
        FaceLayout::Adventure => CardLayout::Adventure,
        FaceLayout::Aftermath => CardLayout::Aftermath,
        FaceLayout::Flip => CardLayout::Flip,
        FaceLayout::Meld => CardLayout::Meld,
        FaceLayout::ModalDfc => CardLayout::ModalDfc,
        FaceLayout::Split => CardLayout::Split,
        FaceLayout::Transform => CardLayout::Transform,
        _ => CardLayout::Normal,
    };

    PrintedCard {
        name,
        layout: card_layout,
        face: parse_face(primary, Face::Primary),
        face_b: secondary.map(|face| parse_face(face, Face::FaceB)),
    }
}

fn parse_face(face: &DatabaseCardFace, face_identifier: Face) -> PrintedCardFace {
    PrintedCardFace {
        displayed_name: face.face_name.clone().unwrap_or_else(|| face.name.clone()),
        face_identifier,
        supertypes: supertypes(split(&face.supertypes)),
        card_types: types(split(&face.types)),
        subtypes: subtypes(split(&face.subtypes)),
        oracle_text: face.text.clone(),
        mana_cost: mana_cost(&face.mana_cost),
        mana_value: face.mana_value.round() as u64,
        power: power(face.power.as_ref()),
        toughness: toughness(face.toughness.as_ref()),
        layout: layout(&face.layout),
        colors: colors(split(&face.colors)),
    }
}

fn split(s: &Option<String>) -> Vec<&str> {
    s.as_ref().map_or(vec![], |s| {
        s.split(',').map(|s| s.trim()).filter(|s| !s.is_empty()).collect::<Vec<_>>()
    })
}

fn supertypes(types: Vec<&str>) -> EnumSet<CardSupertype> {
    types
        .iter()
        .map(|s| {
            s.parse::<CardSupertype>()
                .unwrap_or_else(|e| panic!("Error deserializing supertype '{s}' {e:?}"))
        })
        .collect()
}

fn types(types: Vec<&str>) -> EnumSet<CardType> {
    types
        .iter()
        .map(|s| {
            s.parse::<CardType>()
                .unwrap_or_else(|e| panic!("Error deserializing card type '{s}' {e:?}"))
        })
        .collect()
}

fn subtypes(types: Vec<&str>) -> CardSubtypes {
    let mut result = CardSubtypes::default();
    for subtype in types {
        if let Ok(s) = subtype.parse::<ArtifactSubtype>() {
            result.artifact.insert(s);
        }
        if let Ok(s) = subtype.parse::<EnchantmentSubtype>() {
            result.enchantment.insert(s);
        }
        if let Ok(s) = subtype.parse::<LandType>() {
            result.land.insert(s);
        }
        if let Ok(s) = subtype.parse::<PlaneswalkerSubtype>() {
            result.planeswalker.insert(s);
        }
        if let Ok(s) = subtype.parse::<InstantOrSorcerySubtype>() {
            result.instant_or_sorcery_subtype.insert(s);
        }
        if let Ok(s) = subtype.parse::<CreatureType>() {
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
    result
}

fn mana_cost(cost: &Option<String>) -> ManaCost {
    let Some(cost) = cost else {
        return ManaCost::default();
    };
    let re = Regex::new(r"\{(.*?)}").expect("Invalid regex");
    let mut result = ManaCost::default();
    for capture in re.captures_iter(cost) {
        result.items.extend(to_mana_item(capture.get(1).expect("Expected mana symbol").as_str()));
    }
    result
}

fn to_mana_item(symbol: &str) -> Vec<ManaCostItem> {
    vec![match symbol {
        "W" => ManaCostItem::Colored(ManaColor::White),
        "U" => ManaCostItem::Colored(ManaColor::Blue),
        "B" => ManaCostItem::Colored(ManaColor::Black),
        "R" => ManaCostItem::Colored(ManaColor::Red),
        "G" => ManaCostItem::Colored(ManaColor::Green),
        _ => match symbol.parse::<usize>() {
            Ok(value) => return iter::repeat(ManaCostItem::Generic).take(value).collect(),
            Err(_) => {
                panic!("Unrecognized mana symbol {:?}", symbol);
            }
        },
    }]
}

fn power(power: Option<&String>) -> Option<PrintedPower> {
    power.map(|p| {
        if let Ok(value) = p.parse::<i64>() {
            PrintedPower::Number(value)
        } else {
            todo!("Implement support for non-numeric power");
        }
    })
}

fn toughness(toughness: Option<&String>) -> Option<PrintedToughness> {
    toughness.map(|t| {
        if let Ok(value) = t.parse::<i64>() {
            PrintedToughness::Number(value)
        } else {
            todo!("Implement support for non-numeric toughness");
        }
    })
}

fn layout(string: &str) -> FaceLayout {
    string
        .parse::<FaceLayout>()
        .unwrap_or_else(|e| panic!("Unknown card layout: {:?} {e:?}", string))
}

fn colors(colors: Vec<&str>) -> EnumSet<Color> {
    colors
        .iter()
        .map(|s| {
            s.parse::<Color>().unwrap_or_else(|e| panic!("Error deserializing color '{s}' {e:?}"))
        })
        .collect()
}
