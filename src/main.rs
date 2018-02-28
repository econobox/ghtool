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
extern crate ghtool_label as label;
extern crate ghtool_util as util;

fn main() {
    let _ = details::app().get_matches();
}

/// Details about this app.
mod details {
    use clap::{App, Arg};
    use label;
    use util::config::Config;

    /// This command's app definition.
    pub fn app() -> App<'static, 'static> {
        App::new( name())
            .version(version())
            .author(author())
            .about(description())
            .args(&args()[..])
            .subcommand(
                label::details::app()
            )
    }

    /// This app's name.
    fn name() -> &'static str { env!("CARGO_PKG_NAME") }

    /// This app's version.
    fn version() -> &'static str { env!("CARGO_PKG_VERSION") }

    /// This app's author(s).
    fn author() -> &'static str { env!("CARGO_PKG_AUTHORS") }

    /// This app's description.
    fn description() -> &'static str { env!("CARGO_PKG_DESCRIPTION") }

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
                .required(!Config::file_exists()),
        ]
    }
}
