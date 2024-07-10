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

use std::fs;
use std::fs::File;
use std::io::{BufRead, BufReader, LineWriter, Write};
use std::path::Path;

use regex::Regex;

fn main() {
    let mut result = vec![];
    let re = Regex::new(r"pub (?P<name>\w+)").expect("Invalid regex");
    for l in BufReader::new(File::open("src/delegates/game_delegates.rs").expect("File not found"))
        .lines()
    {
        let line = l.expect("Error reading line");
        if let Some(captures) = re.captures(&line) {
            result.push(captures["name"].to_string());
        }
    }

    let mut output = "//! GENERATED CODE - DO NOT MODIFY

use enumset::EnumSet;

use crate::core::primitives::{AbilityId, Zone};
use crate::delegates::game_delegates::GameDelegates;
use crate::delegates::stores_delegates::StoresDelegates;

pub fn run(delegates: &mut GameDelegates, id: AbilityId, zones: EnumSet<Zone>) {
"
    .to_string();

    for line in &result {
        if line != "struct" {
            output.push_str(&format!("    delegates.{line}.apply_writes(id, zones);\n"));
        }
    }

    output.push_str("}\n");

    let out_path = Path::new("src/delegates/apply_writes.rs");
    if out_path.exists() {
        let mut text = fs::read_to_string(out_path).expect("Error reading file");
        text.pop(); // Remove EOF
        if text == output {
            return;
        }
        fs::remove_file(out_path).expect("Error removing file");
    }
    let mut file = LineWriter::new(File::create(out_path).expect("Error creating file"));
    writeln!(file, "{}", output).expect("Error writing to file");
}
