//
//  label/mod.rs
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

pub mod copy;
pub mod error;
pub mod list;

use clap::ArgMatches;

use self::error::Error;
use config::Config;

pub fn run<'a>(parent_config: Config, matches: &'a ArgMatches) -> Result<(), Error<'a>> {
    match matches.subcommand() {
        ("list", Some(list_matches)) => {
            let config = list::config::Config::from_matches(parent_config, &list_matches)
                .map_err(|err| Error::ArgError(err))?;
            list::run(config).map_err(|err| Error::ListError(err))
        }
        ("copy", Some(copy_matches)) => {
            let config = copy::config::Config::from_matches(parent_config, &copy_matches)
                .map_err(|err| Error::ArgError(err))?;
            copy::run(config).map_err(|err| Error::CopyError(err))
        }
        ("", None) => {
            let _ = details::app().print_help();
            Err(Error::NoSubcommand)
        }
        _ => unreachable!(),
    }
}

/// Details about this command.
pub mod details {
    use clap::{App, Arg};

    /// This command's app definition.
    pub fn app() -> App<'static, 'static> {
        App::new(name())
            .version(version())
            .author(author())
            .about(description())
            .args(&args()[..])
            .subcommand(super::list::details::app())
            .subcommand(super::copy::details::app())
    }

    /// This command's name.
    fn name() -> &'static str {
        "label"
    }

    /// This command's version.
    fn version() -> &'static str {
        env!("CARGO_PKG_VERSION")
    }

    /// This command's author(s).
    fn author() -> &'static str {
        env!("CARGO_PKG_AUTHORS")
    }

    /// This command's arguments.
    fn args() -> Vec<Arg<'static, 'static>> {
        vec![]
    }

    /// This command's description.
    fn description() -> &'static str {
        "Make modifications to the issue labels in a GitHub repository"
    }
}
