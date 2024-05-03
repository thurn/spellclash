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

use dioxus::desktop::{Config, WindowBuilder};
use dioxus::prelude::*;

pub fn launch() {
    LaunchBuilder::desktop()
        .with_cfg(
            Config::new()
                .with_window(WindowBuilder::default().with_title("✦ Spellclash ✦"))
                .with_custom_head(
                    "<script src=\"https://cdn.tailwindcss.com\"></script>".to_string(),
                ),
        )
        .launch(App);
}

fn App() -> Element {
    rsx! {
        style { {include_str!("../assets/style.css")} }
        div { id: "wrapper",
            div { class: "w-32 h-32 bg-lime-500 rounded"}
            div { style: "margin-left: 10px" }
            div { class: "card blue" }
        }
    }
}
