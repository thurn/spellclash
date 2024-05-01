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

//! Generates cards_all.rs. This used to be automatic via the 'linkme' crate,
//! but it has a bunch of problems on e.g. OSX

use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader, LineWriter, Write};
use std::path::Path;
use std::{env, fs};

use regex::Regex;
use walkdir::WalkDir;

fn main() {
    println!("Generating cards_all.rs in {:?}", env::current_dir().unwrap());

    // crate name -> vec of function names
    let mut functions = HashMap::new();
    for e in WalkDir::new("..") {
        let entry = e.expect("Error loading entry");
        let re = Regex::new(r"../(?P<module>\w+)/src/(?P<file>\w+).rs").expect("Invalid regex");

        if let Some(captures) = re.captures(entry.path().to_str().expect("str")) {
            let found = find_functions(entry.path());
            if !found.is_empty() {
                functions.insert(format!("{}::{}", &captures["module"], &captures["file"]), found);
            }
        }
    }

    let out_path = Path::new("../all_cards/src/card_list.rs");
    if out_path.exists() {
        fs::remove_file(out_path).expect("Error removing file");
    }
    let mut file = LineWriter::new(File::create(out_path).expect("Error creating file"));
    writeln!(file, "//! GENERATED CODE - DO NOT MODIFY\n").expect("Error writing file");

    writeln!(file, "use data::card_definitions::definitions::DEFINITIONS;\n")
        .expect("Error writing file");

    let mut modules = functions
        .iter()
        .filter(|(_, list)| !list.is_empty())
        .map(|(module, _)| module.clone())
        .collect::<Vec<_>>();
    modules.sort();

    writeln!(file, "\npub fn initialize() {{").expect("Error writing file");
    for module in &modules {
        if let Some(list) = functions.get(module) {
            for function in list {
                writeln!(file, "    DEFINITIONS.insert({module}::{function});")
                    .expect("Error writing file");
            }
        }
    }
    writeln!(file, "}}").expect("Error writing file");
}

fn find_functions(path: impl AsRef<Path>) -> Vec<String> {
    let mut result = vec![];
    let re = Regex::new(r"pub fn (?P<name>\w+)\(\) -> CardDefinition").expect("Invalid regex");
    for l in BufReader::new(File::open(path).expect("File not found")).lines() {
        let line = l.expect("Error reading line");
        if let Some(captures) = re.captures(&line) {
            result.push(captures["name"].to_string());
        }
    }

    result
}
