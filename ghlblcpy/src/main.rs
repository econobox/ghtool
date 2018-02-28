//
//  main.rs
//  ghlblcpy
//
//  Created by Søren Mortensen on 27/02/2018.
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

#[macro_use]
mod lib;

use lib::config::{Config, RuntimeConfig};
use clap::{App, Arg};
use hubcaps::{Credentials, Github};
use regex::Regex;
use tokio_core::reactor::Core;
use std::io::Write;

fn main() {
    // Set up the arguments.
    let app = App::new("ghlabelcpy")
        .version("0.1.0")
        .author("Søren Mortensen <soren@sorenmortensen.com>")
        .about("Copies labels from one GitHub repository to another")
        .arg(
            Arg::with_name("from")
                .short("f")
                .long("from")
                .value_name("FROM")
                .help("The repository to copy from, in the format \"user/repository\"")
                .takes_value(true)
                .required(true),
        )
        .arg(
            Arg::with_name("to")
                .short("t")
                .long("to")
                .value_name("TO")
                .help("The repository to copy to, in the format \"user/repository\"")
                .takes_value(true)
                .required(true),
        )
        .arg(
            Arg::with_name("token")
                .long("token")
                .value_name("TOKEN")
                .help(
                    "GitHub personal access token that provides access to the \n\
                     repositories specified by <FROM> and <TO>. Overrides any \n\
                     existing value from ~/.config/ghlabelcpy/config.toml",
                )
                .takes_value(true)
                .required(!Config::file_exists()),
        )
        .arg(Arg::with_name("clear").short("c").long("clear").help(
            "Clear the existing labels from the repository specified by \n\
             <TO> before copying the new ones",
        ))
        .arg(Arg::with_name("yes").short("y").long("yes").help(
            "Automatic yes to prompts; assume \"yes\" as an answer to all\n\
             prompts and run non-interactively.",
        ))
        .arg(
            Arg::with_name("v")
                .short("v")
                .multiple(true)
                .help("Sets the level of verbosity (up to -vvv)"),
        );

    // Parse the matches.
    let matches = app.get_matches();

    // Set the level of verbosity based on the -v flag.
    match matches.occurrences_of("v") {
        0 => pretty_logger::init_level(log::LogLevelFilter::Warn),
        1 => pretty_logger::init_level(log::LogLevelFilter::Info),
        2 => pretty_logger::init_level(log::LogLevelFilter::Debug),
        3 => pretty_logger::init_level(log::LogLevelFilter::Trace),
        _ => {
            error!("Invalid verbosity level (maximum is -vvv)");
            return;
        }
    }.expect("Could not initialise logging.");

    info!("Using verbosity level: {}", log::max_log_level());

    if Config::file_exists() {
        info!("--token argument not required: config file found at default location");
    }

    // Extract the values of the <FROM> and <TO> URLs. It's okay to unwrap these because they're required arguments,
    // and `clap` would already have dealt with them and printed out the help screen if they hadn't been provided.
    let from = matches.value_of("from").unwrap();
    let to = matches.value_of("to").unwrap();

    // Create a regular expression for matching "user/repo"-style repository paths.
    let repo_path = Regex::new(r"^([A-Za-z\-_]+)/([A-Za-z\-_]+)").unwrap();

    let (from_user, from_repo) = match repo_path.captures(from) {
        Some(captures) => match (captures.get(1), captures.get(2)) {
            (Some(user), Some(repo)) => (user.as_str(), repo.as_str()),
            _ => {
                error!("Invalid <FROM> path {}", from);
                return;
            }
        },
        None => {
            error!("Invalid <FROM> path {}", from);
            return;
        }
    };

    info!(
        "Will copy labels from repository: {}/{}",
        from_user, from_repo
    );

    let (to_user, to_repo) = match repo_path.captures(to) {
        Some(captures) => match (captures.get(1), captures.get(2)) {
            (Some(user), Some(repo)) => (user.as_str(), repo.as_str()),
            _ => {
                error!("Invalid <TO> path {}", to);
                return;
            }
        },
        None => {
            error!("Invalid <TO> path {}", to);
            return;
        }
    };

    info!("Will copy labels to repository: {}/{}", to_user, to_repo);

    let config = RuntimeConfig {
        config: match (matches.value_of("token"), Config::try_load()) {
            (Some(token), _) => {
                if Config::file_exists() {
                    info!("Overriding access token in configuration file with value from --token argument");
                } else {
                    info!("Using access token provided by --token argument");
                }

                Config {
                    access_token: String::from(token),
                }
            }
            (None, Ok(config)) => {
                info!("Using access token from configuration file");
                config
            }
            (None, Err(err)) => {
                error!("Could not read configuration file: {}", err);
                return;
            }
        },
        clear: matches.args.contains_key("clear"),
        assume_yes: matches.args.contains_key("yes"),
    };

    let mut core = match Core::new() {
        Result::Ok(core) => core,
        Result::Err(err) => {
            error!("Error while initialising core: {}", err);
            return;
        }
    };

    let github = Github::new(
        concat!(env!("CARGO_PKG_NAME"), "/", env!("CARGO_PKG_VERSION")),
        Some(Credentials::Token(config.access_token().clone())),
        &core.handle(),
    );

    info!("Reading labels from repository {}/{}", from_user, from_repo);

    let labels = match core.run(github.repo(from_user, from_repo).labels().list()) {
        Result::Ok(labels) => labels,
        Result::Err(err) => {
            error!(
                "Could not load labels from {}/{}: {}",
                from_user, from_repo, err
            );
            return;
        }
    };

    info!(
        "Found labels: {:?}",
        labels.iter().map(|l| &l.name).collect::<Vec<&String>>()
    );

    // Check that the user is okay with clearing the labels from the destination repository.
    if config.clear && !config.assume_yes {
        match confirm(format!(
            "Clearing labels from {}/{} before copying.",
            to_user, to_repo
        )) {
            true => info!("User confirmed label clearing. Continuing."),
            false => {
                info!("User declined to clear labels. Exiting.");
                return;
            }
        }
    } else if config.clear && config.assume_yes {
        info!("Assuming response \"yes\" to clearing labels from {}/{} before copying because -y flag was specified",
              to_user, to_repo);
    }
}

fn confirm(message: String) -> bool {
    let mut response: String;
    loop {
        print!("{} Continue? [Y/n] ", message);
        let _ = std::io::stdout().flush();
        response = String::new();

        match std::io::stdin().read_line(&mut response) {
            Result::Ok(_) => match &response[..] {
                "y\n" | "Y\n" => break true,
                "n\n" | "N\n" => break false,
                _ => (),
            },
            Result::Err(_) => (),
        }
    }
}
