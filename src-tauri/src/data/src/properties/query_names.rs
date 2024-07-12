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

use crate::core::card_tags::CardTag;
use crate::core::primitives::Color;
use crate::delegates::query_value::{ChangeText, EnumSets};
use crate::printed_cards::card_subtypes::LandType;
use crate::properties::card_properties::CardProperties;
use crate::properties::card_query::CardArgumentProperty;
use crate::properties::query_name::QueryName;

pub struct TagsQuery;

impl QueryName for TagsQuery {
    type Arg = ();
    type Modifier = EnumSets<CardTag>;

    fn query(queries: &CardProperties) -> &CardArgumentProperty<(), EnumSets<CardTag>> {
        &queries.tags
    }

    fn query_mut(queries: &mut CardProperties) -> &mut CardArgumentProperty<(), EnumSets<CardTag>> {
        &mut queries.tags
    }
}

pub struct ColorsQuery;

impl QueryName for ColorsQuery {
    type Arg = ();
    type Modifier = EnumSets<Color>;

    fn query(queries: &CardProperties) -> &CardArgumentProperty<(), EnumSets<Color>> {
        &queries.colors
    }

    fn query_mut(queries: &mut CardProperties) -> &mut CardArgumentProperty<(), EnumSets<Color>> {
        &mut queries.colors
    }
}

pub struct LandTypesQuery;

impl QueryName for LandTypesQuery {
    type Arg = ();
    type Modifier = EnumSets<LandType>;

    fn query(queries: &CardProperties) -> &CardArgumentProperty<(), EnumSets<LandType>> {
        &queries.land_types
    }

    fn query_mut(
        queries: &mut CardProperties,
    ) -> &mut CardArgumentProperty<(), EnumSets<LandType>> {
        &mut queries.land_types
    }
}

pub struct ChangeLandTypeTextQuery;

impl QueryName for ChangeLandTypeTextQuery {
    type Arg = ();
    type Modifier = ChangeText<LandType>;

    fn query(queries: &CardProperties) -> &CardArgumentProperty<(), ChangeText<LandType>> {
        &queries.change_land_type_text
    }

    fn query_mut(
        queries: &mut CardProperties,
    ) -> &mut CardArgumentProperty<(), ChangeText<LandType>> {
        &mut queries.change_land_type_text
    }
}

pub struct ChangeColorTextQuery;

impl QueryName for ChangeColorTextQuery {
    type Arg = ();
    type Modifier = ChangeText<Color>;

    fn query(queries: &CardProperties) -> &CardArgumentProperty<(), ChangeText<Color>> {
        &queries.change_color_text
    }

    fn query_mut(queries: &mut CardProperties) -> &mut CardArgumentProperty<(), ChangeText<Color>> {
        &mut queries.change_color_text
    }
}