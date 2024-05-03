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

use std::convert::Infallible;
use std::error;
use std::fmt::Display;

use color_eyre::eyre::{ContextCompat, WrapErr};

use crate::outcome::{StopCondition, Value};

/// Equivalent function to color_eyre::bail
///
/// Immediately returns with an Error condition.
#[macro_export]
macro_rules! fail {
    ($msg:literal $(,)?) => {
        use color_eyre::eyre::eyre;
        return std::result::Result::Err($crate::outcome::StopCondition::Error(color_eyre::eyre::eyre!($msg)));
    };
    ($err:expr $(,)?) => {
        use color_eyre::eyre::eyre;
        return std::result::Result::Err($crate::outcome::StopCondition::Error(color_eyre::eyre::eyre!($err)));
    };
    ($fmt:expr, $($arg:tt)*) => {
        use color_eyre::eyre::eyre;
        return std::result::Result::Err($crate::outcome::StopCondition::Error(color_eyre::eyre::eyre!($fmt, $($arg)*)));
    };
}

/// Equivalent function to color_eyre::ensure
///
/// Returns with an error condition if the provided predicate evaluates to false
#[macro_export]
macro_rules! verify {
    ($cond:expr $(,)?) => {
        if !$cond {
            $crate::verify!($cond, concat!("Condition failed: `", stringify!($cond), "`"))
        }
    };
    ($cond:expr, $msg:literal $(,)?) => {
        if !$cond {
            return std::result::Result::Err($crate::outcome::StopCondition::Error(color_eyre::eyre::eyre!($msg)));
        }
    };
    ($cond:expr, $err:expr $(,)?) => {
        if !$cond {
            return std::result::Result::Err($crate::outcome::StopCondition::Error(color_eyre::eyre::eyre!($err)));
        }
    };
    ($cond:expr, $fmt:expr, $($arg:tt)*) => {
        if !$cond {
            return std::result::Result::Err($crate::outcome::StopCondition::Error(color_eyre::eyre::eyre!($fmt, $($arg)*)));
        }
    };
}

// eyre doesn't want you to do this, but it's for reasons that have never been
// adequately explained to me.
impl<T> WithError<T, Infallible> for Option<T> {
    fn with_error<C, F>(self, context: F) -> Value<T>
    where
        C: Display + Send + Sync + 'static,
        F: FnOnce() -> C,
    {
        self.with_context(context).map_err(StopCondition::Error)
    }
}

pub trait WithError<T, E> {
    /// Wrapper around color_eyre::with_context. Wraps the error value with
    /// additional context that is evaluated lazily only once an error does
    /// occur. Can be configured to panic on error.
    fn with_error<C, F>(self, f: F) -> Value<T>
    where
        C: Display + Send + Sync + 'static,
        F: FnOnce() -> C;
}

impl<T, E> WithError<T, E> for Result<T, E>
where
    E: error::Error + Send + Sync + 'static,
{
    fn with_error<C, F>(self, context: F) -> Value<T>
    where
        C: Display + Send + Sync + 'static,
        F: FnOnce() -> C,
    {
        self.with_context(context).map_err(StopCondition::Error)
    }
}
