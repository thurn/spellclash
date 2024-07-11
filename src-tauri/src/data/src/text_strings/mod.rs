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

use std::fmt::{Display, Formatter};

use either::Either;
use serde::{Deserialize, Serialize};

use crate::core::primitives::Color;
use crate::printed_cards::card_subtypes::LandType;

/// Canonical text displayed in the user interface, suitable for localization
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum Text {
    HandToTopOfLibraryPrompt,
    SelectNumber,
    SelectTarget,
    Color(Color),
    LandSubtype(LandType),
    SelectTypeToChange,
    SelectNewType,
}

impl<T: Into<Text>, U: Into<Text>> From<Either<T, U>> for Text {
    fn from(value: Either<T, U>) -> Self {
        match value {
            Either::Left(left) => left.into(),
            Either::Right(right) => right.into(),
        }
    }
}

impl Display for Text {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Text::HandToTopOfLibraryPrompt => {
                write!(f, "Choose a card from your hand to put on top of your library.")
            }
            Text::SelectNumber => write!(f, "Select number"),
            Text::SelectTarget => write!(f, "Select target"),
            Text::Color(color) => write!(f, "{}", color),
            Text::LandSubtype(subtype) => write!(f, "{}", subtype),
            Text::SelectTypeToChange => write!(f, "Select type to change"),
            Text::SelectNewType => write!(f, "Select new type"),
        }
    }
}
