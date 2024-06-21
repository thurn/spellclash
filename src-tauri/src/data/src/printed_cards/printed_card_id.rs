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

use crate::card_definitions::card_name::CardName;

/// A unique ID for a specific printing of card.
///
/// This is the identifier for a card in the scryfall database. If you click
/// "copy-pasteable JSON' on scryfall, this is the identifier in the "id" field.
///
/// See <https://scryfall.com/docs/api/cards>
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub struct PrintedCardId(pub Uuid);

pub const GRIZZLY_BEARS: PrintedCardId =
    PrintedCardId(uuid!("409f9b88-f03e-40b6-9883-68c14c37c0de"));
pub const FOREST: PrintedCardId = PrintedCardId(uuid!("8c13cafb-3078-4856-a5b0-c38aace8a34a"));
pub const ISLAND: PrintedCardId = PrintedCardId(uuid!("64daf0ac-678b-4683-9351-a6daf9c9f849"));
pub const GIGANTOSAURUS: PrintedCardId =
    PrintedCardId(uuid!("c1db84d8-d426-4c0d-b44e-5be7b0f5f5bf"));
pub const ALPINE_GRIZZLY: PrintedCardId =
    PrintedCardId(uuid!("38bbf983-df71-4403-86f3-2e86aa8765b8"));
pub const LEATHERBACK_BALOTH: PrintedCardId =
    PrintedCardId(uuid!("55f97b4c-42c7-4986-a150-0b8de11f0537"));
pub const KALONIAN_TUSKER: PrintedCardId =
    PrintedCardId(uuid!("135946fc-fe67-401f-821d-d7145c63f030"));
pub const ANCIENT_BRONTODON: PrintedCardId =
    PrintedCardId(uuid!("39421ce8-86d5-4739-b6fd-78d63c0bb258"));
pub const GARRUKS_GOREHORN: PrintedCardId =
    PrintedCardId(uuid!("3928bbce-87b7-4b28-9af4-20362935c909"));
pub const GOLDEN_BEAR: PrintedCardId = PrintedCardId(uuid!("d7dfc789-7ea0-4eb8-8c3b-2c50fd52cbab"));
pub const PRIMORDIAL_WURM: PrintedCardId =
    PrintedCardId(uuid!("fc0e8298-adac-4922-8824-a1fafa089f72"));
pub const VORSTCLAW: PrintedCardId = PrintedCardId(uuid!("79719ed0-468d-4946-8dfc-fb7e2b2e305e"));
pub const TERRAIN_ELEMENTAL: PrintedCardId =
    PrintedCardId(uuid!("32b89e5c-ffb4-406f-99d1-ec2797aca061"));
pub const ORAZCA_FRILLBACK: PrintedCardId =
    PrintedCardId(uuid!("20471a3b-90f9-4463-9b43-fc7b9b28f5d1"));
pub const SWORDWISE_CENTAUR: PrintedCardId =
    PrintedCardId(uuid!("1776ebd7-91fc-49e1-a978-f2012162d1cf"));
pub const QUILLED_SLAGWURM: PrintedCardId =
    PrintedCardId(uuid!("12c597b9-5024-42bd-b500-5ef6a3accda6"));
pub const ELVISH_WARRIOR: PrintedCardId =
    PrintedCardId(uuid!("c3d0485a-209d-4040-94ab-856bdee83b81"));
pub const NYXBORN_COLOSSUS: PrintedCardId =
    PrintedCardId(uuid!("8b4f003c-1e99-4e53-ad6d-81ff3c592b2c"));
pub const RUMBLING_BALOTH: PrintedCardId =
    PrintedCardId(uuid!("93a56610-482b-4ddf-88e1-e4a2edf4fa0f"));
pub const GRIZZLED_OUTRIDER: PrintedCardId =
    PrintedCardId(uuid!("4a1d4473-5317-4bdd-9cb9-93670acf52e9"));
pub const CENTAUR_COURSER: PrintedCardId =
    PrintedCardId(uuid!("e8b67ee8-3189-4426-8b1a-b540267768fd"));
pub const GORILLA_WARRIOR: PrintedCardId =
    PrintedCardId(uuid!("d6997a75-42c9-4706-ac34-69fa34011eca"));
pub const SILVERBACK_APE: PrintedCardId =
    PrintedCardId(uuid!("025b3156-975d-4f64-b19c-172cb21266c5"));
pub const PANTHER_WARRIORS: PrintedCardId =
    PrintedCardId(uuid!("ba165e25-5328-40f4-b87c-9d02590f9d38"));
pub const FEROCIOUS_ZHENG: PrintedCardId =
    PrintedCardId(uuid!("7a6d1184-15e0-4b41-ba2d-4f68e91c61d4"));
pub const ELVISH_RANGER: PrintedCardId =
    PrintedCardId(uuid!("26caff65-3a96-46f2-8f0b-e5091b632a2e"));
pub const ENORMOUS_BALOTH: PrintedCardId =
    PrintedCardId(uuid!("54069e65-eef4-4fb8-bb0d-932a4c9889b3"));
pub const CRAW_WURM: PrintedCardId = PrintedCardId(uuid!("3875f73d-6108-488b-bd34-4cf2c23ce6b3"));
pub const BROODHUNTER_WURM: PrintedCardId =
    PrintedCardId(uuid!("c11c852d-9c7c-4d9b-8e79-70ea5ac865df"));
pub const AXEBANE_STAG: PrintedCardId =
    PrintedCardId(uuid!("bfce7c02-ccc3-44cd-8087-627eaa6a072e"));
pub const SPINED_WURM: PrintedCardId = PrintedCardId(uuid!("c5334998-3c84-4b04-a5c2-d66c9d99e93b"));
pub const SCALED_WURM: PrintedCardId = PrintedCardId(uuid!("bd17b2c1-c3dd-4f6f-a44c-dc81c6bc1c94"));
pub const ALPHA_TYRRANAX: PrintedCardId =
    PrintedCardId(uuid!("4a2e5279-f28c-4a78-9f8a-16c9f72f8d38"));
pub const WHIPTAIL_WURM: PrintedCardId =
    PrintedCardId(uuid!("aefe8c78-c07b-427c-9e8d-fad5ea2bfca9"));
pub const CANOPY_GORGER: PrintedCardId =
    PrintedCardId(uuid!("cbc8957d-769c-4630-9544-56cea8c847c2"));
pub const VASTWOOD_GORGER: PrintedCardId =
    PrintedCardId(uuid!("72f53dc9-5397-49e1-97d4-3b0b6858f2b2"));
pub const PHERES_BAND_CENTAURS: PrintedCardId =
    PrintedCardId(uuid!("2168fcf4-cf87-4ab8-9710-6ec672750a9a"));
pub const BRAINSTORM: PrintedCardId = PrintedCardId(uuid!("84479779-d570-4eee-9982-f6e918b4d75b"));
pub const DANDAN: PrintedCardId = PrintedCardId(uuid!("ac2e32d0-f172-4934-9d73-1bc2ab86586e"));
pub const DANCE_OF_THE_SKYWISE: PrintedCardId =
    PrintedCardId(uuid!("c548d140-3d81-4b33-9985-87703d316a83"));
pub const RAY_OF_COMMAND: PrintedCardId =
    PrintedCardId(uuid!("638abe5f-2a8a-42ca-bcdf-a52a3df66946"));
