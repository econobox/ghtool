//
//  util/error.rs
//  ghtool
//
//  Created by Søren Mortensen on 01/03/2018.
//  Copyright © 2018 Søren Mortensen.
//
//  Licensed under the Apache License, Version 2.0 (the "License");
//  you may not use this file except in compliance with the License.
//  You may obtain a copy of the License at
//
//      http://www.apache.org/licenses/LICENSE-2.0
//
//  Unless required by applicable law or agreed to in writing, software
//  distributed under the License is distributed on an "AS IS" BASIS,
//  WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
//  See the License for the specific language governing permissions and
//  limitations under the License.
//

use std::error::Error;
use std::fmt;

/// Errors that could arise in the process of parsing command-line arguments.
#[derive(Debug)]
pub enum ArgError<'a> {
    /// No subcommand was provided, and one is required.
    NoSubcommand,
    /// No value was provided for the argument with name `arg`.
    NoValue { arg: &'a str },
    /// An invalid value `value` was provided for the argument with name `arg`.
    InvalidValue { arg: &'a str, value: &'a str },
}

impl<'a> fmt::Display for ArgError<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ArgError::NoSubcommand => write!(f, "No subcommand was provided"),
            ArgError::NoValue { ref arg } => write!(f, "No value for argument {}", arg),
            ArgError::InvalidValue { ref arg, ref value } => {
                write!(f, "Invalid value {} provided for argument {}", value, arg)
            }
        }
    }
}

impl<'a> Error for ArgError<'a> {
    fn description(&self) -> &str {
        match *self {
            ArgError::NoSubcommand => "NoSubcommand",
            ArgError::NoValue { .. } => "NoValue",
            ArgError::InvalidValue { .. } => "InvalidValue",
        }
    }
}
