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

use serde::{Deserialize, Serialize};
use uuid::{uuid, Uuid};

/// Identifies a named oracle instance of a card, i.e. something a player can
/// pick when resolving a "choose a card name" effect.
///
/// This is a wrapper around the Scryfall Oracle ID:
///
/// > A unique ID for this card’s oracle identity. This value is consistent
/// > across reprinted card editions, and unique among different cards with the
/// > same name (tokens, Unstable variants, etc). Always present except for the
/// > reversible_card layout where it will be absent; oracle_id will be found on
/// > each face instead.
///
/// You can find this information on a Scryfall card page by clicking
/// "Copy-pasteable JSON" and copying the oracle_id field.
///
/// See <https://scryfall.com/docs/api/cards>
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash, Ord, PartialOrd, Serialize, Deserialize)]
pub struct CardName(pub Uuid);

pub const BRAINSTORM: CardName = CardName(uuid!("36cd2364-d113-47d1-b2c4-b088d9eb88dd"));
pub const DANDAN: CardName = CardName(uuid!("88929373-b2c8-4a81-a809-fed87fd5b0d7"));
pub const FOREST: CardName = CardName(uuid!("b34bb2dc-c1af-4d77-b0b3-a0fb342a5fc6"));
pub const GRIZZLY_BEARS: CardName = CardName(uuid!("14c8f55d-d177-4c25-a931-ebeb9e6062a0"));
pub const ISLAND: CardName = CardName(uuid!("b2c6aa39-2d2a-459c-a555-fb48ba993373"));
pub const MOUNTAIN: CardName = CardName(uuid!("a3fb7228-e76b-4e96-a40e-20b5fed75685"));
pub const PLAINS: CardName = CardName(uuid!("bc71ebf6-2056-41f7-be35-b2e5c34afa99"));
pub const SWAMP: CardName = CardName(uuid!("56719f6a-1a6c-4c0a-8d21-18f7d7350b68"));
pub const LEATHERBACK_BALOTH: CardName = CardName(uuid!("0ee14128-3bec-4b65-8ee4-619337d4ed45"));
pub const ALPINE_GRIZZLY: CardName = CardName(uuid!("86e3c86c-d50e-4aa9-b080-f68b6afee87c"));
pub const KALONIAN_TUSKER: CardName = CardName(uuid!("df7f697e-6886-4897-a024-61ae225c1b34"));
pub const GOLDEN_BEAR: CardName = CardName(uuid!("6ba7ab42-918e-472a-b364-06073cc354d2"));
pub const TERRAIN_ELEMENTAL: CardName = CardName(uuid!("3a9b9fe0-1846-41fc-b7c3-ed9abef27a6a"));
pub const GIGANTOSAURUS: CardName = CardName(uuid!("e666bae7-dd51-4921-8b89-7e8d423caba0"));
pub const ANCIENT_BRONTODON: CardName = CardName(uuid!("745be03c-69dc-4239-a5e7-38587b56eac0"));
pub const GARRUKS_GOREHORN: CardName = CardName(uuid!("c8384392-1914-4423-9f49-92f5643d15dc"));
pub const PRIMORDIAL_WURM: CardName = CardName(uuid!("ab9778da-f5e3-4c17-a3da-ffef6ed7a7a1"));
pub const VORSTCLAW: CardName = CardName(uuid!("aa944cbe-fdba-4001-97d5-c722fe744dcc"));
pub const ORAZCA_FRILLBACK: CardName = CardName(uuid!("2da32ec3-f42f-4edc-b5b3-e5f39fcf370c"));
pub const SWORDWISE_CENTAUR: CardName = CardName(uuid!("5c2e0762-4994-4c34-b33f-909c2b3f0f43"));
pub const QUILLED_SLAGWURM: CardName = CardName(uuid!("5114f3e0-339d-46d6-9d44-97fa0afbd7c5"));
pub const ELVISH_WARRIOR: CardName = CardName(uuid!("da587161-da7e-48ae-8af6-7f02ee71afd8"));
pub const NYXBORN_COLOSSUS: CardName = CardName(uuid!("4a74cccb-5d75-4362-9c9e-2622096544fe"));
pub const RUMBLING_BALOTH: CardName = CardName(uuid!("3191c6ca-4d25-4ba3-bfe1-4aeab1295573"));
pub const GRIZZLED_OUTRIDER: CardName = CardName(uuid!("0d3d0da2-73fe-4f8a-b13c-23981eb7e2ee"));
pub const CENTAUR_COURSER: CardName = CardName(uuid!("2f5bf099-2e01-4e1c-9ebf-0ce0ac66939e"));
pub const GORILLA_WARRIOR: CardName = CardName(uuid!("c0e6ae0f-6cf7-48ca-acb8-73d4c38b9005"));
pub const SILVERBACK_APE: CardName = CardName(uuid!("a72f9437-d652-4a64-99df-a307a6dd9a0d"));
pub const PANTHER_WARRIORS: CardName = CardName(uuid!("ef7788af-8edc-46df-a5b6-895c734ea423"));
pub const FEROCIOUS_ZHENG: CardName = CardName(uuid!("a45e0854-28c7-41f5-a9fe-5b76a8070c5b"));
pub const ELVISH_RANGER: CardName = CardName(uuid!("092f2c34-1f75-4998-b219-2cf1ca73656d"));
pub const ENORMOUS_BALOTH: CardName = CardName(uuid!("6189cd17-61aa-420a-ba53-5ddaf2bbc2ba"));
pub const CRAW_WURM: CardName = CardName(uuid!("6a462a69-3e42-41de-a3aa-a488d9f38d69"));
pub const BROODHUNTER_WURM: CardName = CardName(uuid!("02d50c11-51e3-4b3f-bf3d-3b1775d514f6"));
pub const AXEBANE_STAG: CardName = CardName(uuid!("a45d7135-c042-40d9-818c-74fa1c7af8ad"));
pub const SPINED_WURM: CardName = CardName(uuid!("05425f2f-7228-4bf5-8fe1-6fe99107e8e0"));
pub const SCALED_WURM: CardName = CardName(uuid!("009a8e38-74d0-4b4a-b8ca-62c9f3183531"));
pub const ALPHA_TYRRANAX: CardName = CardName(uuid!("8bed1349-4d0a-4544-ba66-c38e72cd2863"));
pub const WHIPTAIL_WURM: CardName = CardName(uuid!("50fa6a63-e031-47cb-8fd5-a6c235203722"));
pub const CANOPY_GORGER: CardName = CardName(uuid!("1d614b37-d578-4f35-8fd8-3c7800d5f742"));
pub const VASTWOOD_GORGER: CardName = CardName(uuid!("a17a86a3-9f9c-4e09-93e6-e543a70733bc"));
pub const PHERES_BAND_CENTAURS: CardName = CardName(uuid!("4aebea34-a05b-4626-97bc-f15fa177ef05"));
pub const DANCE_OF_THE_SKYWISE: CardName = CardName(uuid!("5ab0ab9b-c143-4315-8983-f645fb21f8b3"));
pub const RAY_OF_COMMAND: CardName = CardName(uuid!("adf41d29-403d-4936-9689-3148d103e700"));
pub const GIANT_GROWTH: CardName = CardName(uuid!("5748ebf1-24e3-499d-ab7c-c2cebd462a24"));
pub const CRYSTAL_SPRAY: CardName = CardName(uuid!("fe371e3a-976f-41d8-ad58-4b3b61415d71"));
