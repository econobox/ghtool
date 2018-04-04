//
//  label/list/mod.rs
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

pub mod config;
pub mod error;

use self::error::ListError;
use futures::Stream;
use hubcaps::{Credentials, Github};
use tokio_core::reactor::Core;

pub fn run(config: config::Config) -> Result<(), ListError> {
    info!("Listing labels in {}...", config.repo);

    let mut core = Core::new().map_err(|err| ListError::IoError(err))?;

    let github = Github::new(
        concat!(env!("CARGO_PKG_NAME"), "/", env!("CARGO_PKG_VERSION")),
        Some(Credentials::Token(
            config.parent_config.access_token().clone(),
        )),
        &core.handle(),
    );

    core.run(
        github
            .repo(config.repo.user, config.repo.repo)
            .labels()
            .iter()
            .for_each(|label| {
                println!("{}", label.name);
                Ok(())
            })
    ).map_err(|err| ListError::HubcapsError(err))?;

    Ok(())
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
    }

    /// This command's name.
    fn name() -> &'static str {
        "list"
    }

    /// This command's version.
    fn version() -> &'static str {
        env!("CARGO_PKG_VERSION")
    }

    /// This command's author(s).
    fn author() -> &'static str {
        env!("CARGO_PKG_AUTHORS")
    }

    /// This command's description.
    fn description() -> &'static str {
        "List labels in a repository"
    }

    /// This command's arguments.
    fn args() -> Vec<Arg<'static, 'static>> {
        vec![
            Arg::with_name("repo")
                .index(1)
                .value_name("REPO")
                .help("The repository, in the format \"user/repository\"")
                .takes_value(true)
                .required(true),
        ]
    }
}
