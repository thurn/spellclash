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

use ratatui::prelude::Color;

pub fn selected() -> Color {
    dark_gray()
}

pub fn black() -> Color {
    color("#140C1C")
}

pub fn dark_red() -> Color {
    color("#442434")
}

pub fn dark_blue() -> Color {
    color("#30346D")
}

pub fn dark_gray() -> Color {
    color("#4E4A4F")
}

pub fn brown() -> Color {
    color("#854C30")
}

pub fn dark_green() -> Color {
    color("#346524")
}

pub fn red() -> Color {
    color("#D04648")
}

pub fn light_gray() -> Color {
    color("#757161")
}

pub fn light_blue() -> Color {
    color("#597DCE")
}

pub fn orange() -> Color {
    color("#D27D2C")
}

pub fn blue_gray() -> Color {
    color("#8595A1")
}

pub fn light_green() -> Color {
    color("#6DAA2C")
}

pub fn peach() -> Color {
    color("#D2AA99")
}

pub fn cyan() -> Color {
    color("#6DC2CA")
}

pub fn yellow() -> Color {
    color("#DAD45E")
}

pub fn white() -> Color {
    color("#DEEED6")
}

fn color(s: &'static str) -> Color {
    s.parse().expect("Invalid color literal")
}
