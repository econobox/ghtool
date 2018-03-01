//
//  main.rs
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

extern crate clap;
extern crate hubcaps;
#[macro_use]
extern crate log;
extern crate pretty_logger;
extern crate regex;
#[macro_use]
extern crate serde_derive;
extern crate toml;

pub mod config;
pub mod label;
pub mod util;

use config::{Config, IntoConfig, StoredConfig};

fn main() {
    let matches = details::app().get_matches();

    // Set the level of verbosity based on the -v flag.
    match matches.occurrences_of("v") {
        0 => pretty_logger::init_level(log::LogLevelFilter::Warn),
        1 => pretty_logger::init_level(log::LogLevelFilter::Info),
        2 => pretty_logger::init_level(log::LogLevelFilter::Debug),
        3 => pretty_logger::init_level(log::LogLevelFilter::Trace),
        _ => {
            let _ = pretty_logger::init_to_defaults();
            error!("Invalid verbosity level (maximum is -vvv)");
            return;
        }
    }.expect("Could not initialise logging.");

    info!("Using verbosity level: {}", log::max_log_level());

    if StoredConfig::file_exists() {
        info!("--token argument not required: config file found at default location");
    }

    let config = match (matches.value_of("token"), StoredConfig::try_load()) {
        (Some(token), Ok(_)) => {
            info!("Overriding access token in configuration file with value from --token argument");
            Config {
                access_token: token.to_owned(),
            }
        },
        (Some(token), Err(_)) => {
            info!("Using access token provided by --token argument");
            Config {
                access_token: token.to_owned(),
            }
        },
        (None, Ok(stored_config)) => IntoConfig::from(stored_config).build(),
        (None, Err(err)) => {
            error!("Could not read configuration file: {}", err);
            return;
        }
    };

    // Now go into the subcommand. Exit with an error if no subcommand was specified.
    match matches.subcommand() {
        ("label", Some(label_matches)) => {
            let _ = label::run(config, label_matches);
        }
        ("", None) => {
            let _ = details::app().print_help();
            return;
        }
        _ => unreachable!(),
    }
}

/// Details about this app.
mod details {
    use clap::{App, Arg};
    use config::StoredConfig;
    use label;

    /// This command's app definition.
    pub fn app() -> App<'static, 'static> {
        App::new(name())
            .version(version())
            .author(author())
            .about(description())
            .args(&args()[..])
            .subcommand(label::details::app())
    }

    /// This app's name.
    fn name() -> &'static str {
        env!("CARGO_PKG_NAME")
    }

    /// This app's version.
    fn version() -> &'static str {
        env!("CARGO_PKG_VERSION")
    }

    /// This app's author(s).
    fn author() -> &'static str {
        env!("CARGO_PKG_AUTHORS")
    }

    /// This app's description.
    fn description() -> &'static str {
        env!("CARGO_PKG_DESCRIPTION")
    }

    /// This app's arguments.
    fn args() -> Vec<Arg<'static, 'static>> {
        vec![
            Arg::with_name("token")
                .long("token")
                .short("T")
                .value_name("TOKEN")
                .help(
                    "GitHub personal access token that provides access to the repositories specified by <FROM> and \
                    <TO>. Overrides any existing value from ~/.config/ghtool/config.toml. Not required if a \
                    configuration file is found."
                )
                .takes_value(true)
                .required(!StoredConfig::file_exists()),
            Arg::with_name("v")
                .short("v")
                .multiple(true)
                .help("Sets the level of verbosity (up to -vvv)"),
        ]
    }
}
