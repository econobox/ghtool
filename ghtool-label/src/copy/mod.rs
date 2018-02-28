//
//  copy/mod.rs
//  ghtool-label
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
    fn name() -> &'static str { "copy" }

    /// This command's version.
    fn version() -> &'static str { env!("CARGO_PKG_VERSION") }

    /// This command's author(s).
    fn author() -> &'static str { env!("CARGO_PKG_AUTHORS") }

    /// This command's description.
    fn description() -> &'static str { "Copy labels from one repository to another" }

    /// This command's arguments.
    fn args() -> Vec<Arg<'static, 'static>> {
        vec![
            // --from
            Arg::with_name("from")
                .short("f")
                .long("from")
                .value_name("FROM")
                .help("The repository to copy from, in the format \"user/repository\"")
                .takes_value(true)
                .required(true),
            // --to
            Arg::with_name("to")
                .short("t")
                .long("to")
                .value_name("TO")
                .help("The repository to copy to, in the format \"user/repository\"")
                .takes_value(true)
                .required(true),
            // --clear
            Arg::with_name("clear")
                .short("c")
                .long("clear")
                .help(
                    "Clear the existing labels from the repository specified by <TO> before copying the new ones"
                ),
            // --yes
            Arg::with_name("yes")
                .short("y")
                .long("yes")
                .help(
                    "Automatic yes to prompts; assume \"yes\" as an answer to all prompts and run non-interactively.",
                ),
            // -v, -vv, -vvv
            Arg::with_name("v")
                .short("v")
                .multiple(true)
                .help("Sets the level of verbosity (up to -vvv)"),
        ]
    }
}
