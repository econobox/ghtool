//
//  copy/config.rs
//  ghtool
//
//  Created by Søren Mortensen on 28/02/2018.
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

use clap::ArgMatches;
use util::repo::Repo;

use std::error::Error;
use std::fmt;

/// Configuration for the `label copy` command.
pub struct Config {
    /// The repository to copy labels from.
    pub from_repo: Repo,
    /// The repository to copy labels to.
    pub to_repo: Repo,
}

impl<'a> Config {
    /// Attempts to create a `Config` by parsing command-line argument matches.
    pub fn from_matches(matches: &'a ArgMatches) -> Result<Config, ConfigError<'a>> {
        let from_string = matches
            .value_of("from")
            .ok_or(ConfigError::NoValue { arg: "from" })?;

        let to_string = matches
            .value_of("to")
            .ok_or(ConfigError::NoValue { arg: "to" })?;

        let from_repo = Repo::from_string(&from_string[..]).ok_or(ConfigError::InvalidValue {
            arg: "from",
            value: from_string,
        })?;

        let to_repo = Repo::from_string(&to_string[..]).ok_or(ConfigError::InvalidValue {
            arg: "to",
            value: to_string,
        })?;

        Ok(Config { from_repo, to_repo })
    }
}

/// Errors that could arise in the process of parsing arguments passed to `label copy`.
#[derive(Debug)]
pub enum ConfigError<'a> {
    NoValue { arg: &'a str },
    InvalidValue { arg: &'a str, value: &'a str },
}

impl<'a> fmt::Display for ConfigError<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ConfigError::NoValue { ref arg } => write!(f, "No value for argument {}", arg),
            ConfigError::InvalidValue { ref arg, ref value } => {
                write!(f, "Invalid value {} provided for argument {}", value, arg)
            }
        }
    }
}

impl<'a> Error for ConfigError<'a> {
    fn description(&self) -> &str {
        match *self {
            ConfigError::NoValue { .. } => "NoValue",
            ConfigError::InvalidValue { .. } => "InvalidValue",
        }
    }
}
