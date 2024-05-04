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

use dioxus::prelude::*;

#[component]
pub fn ButtonComponent(children: Element) -> Element {
    let class = "m-8 align-middle select-none font-sans font-bold text-center uppercase\
     transition-all disabled:opacity-50 disabled:shadow-none disabled:pointer-events-none\
     text-xs py-3 px-6 rounded-lg bg-gray-900 text-white shadow-md shadow-gray-900/10\
     hover:shadow-lg hover:shadow-gray-900/20 focus:opacity-[0.85] focus:shadow-none\
     active:opacity-[0.85] active:shadow-none";

    rsx! {
        button {
            class: class,
            {children}
        }
    }
}
