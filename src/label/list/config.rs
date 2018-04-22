//
//  label/list/config.rs
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

use clap::ArgMatches;

use config::Config as ParentConfig;
use util::error::ArgError;
use util::repo::Repo;

/// Configuration for the `label list` command.
pub struct Config {
    pub parent_config: ParentConfig,
    /// The repository to list the labels from.
    pub repo: Repo,
}

impl<'a> Config {
    /// Attempts to create a `Config` by parsing command-line argument matches.
    pub fn from_matches(
        parent_config: ParentConfig,
        matches: &'a ArgMatches,
    ) -> Result<Config, ArgError<'a>> {
        let repo_string = matches
            .value_of("repo")
            .ok_or(ArgError::NoValue { arg: "repo" })?;

        let repo = Repo::from_string(&repo_string[..]).ok_or(ArgError::InvalidValue {
            arg: "repo",
            value: repo_string,
        })?;

        Ok(Config {
            parent_config,
            repo,
        })
    }
}
