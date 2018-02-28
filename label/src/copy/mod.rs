//
//  copy/mod.rs
//  ghtool-label
//
//  Created by Søren Mortensen on 28/02/2018.
//  Copyright © 2018 Søren Mortensen.
//
//  This program is free software: you can redistribute it and/or modify
//  it under the terms of the GNU General Public License as published by
//  the Free Software Foundation, either version 3 of the License, or
//  (at your option) any later version.
//
//  This program is distributed in the hope that it will be useful,
//  but WITHOUT ANY WARRANTY; without even the implied warranty of
//  MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
//  GNU General Public License for more details.
//
//  You should have received a copy of the GNU General Public License
//  along with this program.  If not, see <http://www.gnu.org/licenses/>.
//

pub mod config;

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
        "copy"
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
        "Copy labels from one repository to another"
    }

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
        ]
    }
}
