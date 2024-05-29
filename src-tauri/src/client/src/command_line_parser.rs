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

use clap::Parser;
use utils::command_line::{CommandLine, TracingStyle};

use crate::initialize::version;

#[derive(Parser, Debug)]
#[command(version = version(), about)]
pub struct CommandLineParser {
    #[arg(
        long,
        value_enum,
        default_value_t = TracingStyle::Forest,
        help = "Configuration for capturing program traces")]
    pub tracing_style: TracingStyle,
}

impl CommandLineParser {
    pub fn build(self) -> CommandLine {
        CommandLine { tracing_style: self.tracing_style }
    }
}
