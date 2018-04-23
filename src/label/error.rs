//
//  label/error.rs
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

use std::error;
use std::fmt;

use super::copy::error::CopyError;
use super::list::error::ListError;
use util::error::ArgError;

#[derive(Debug)]
pub enum Error<'a> {
    NoSubcommand,
    ArgError(ArgError<'a>),
    ListError(ListError),
    CopyError(CopyError),
}

impl<'a> fmt::Display for Error<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Error::NoSubcommand => write!(f, "No subcommand provided"),
            Error::ArgError(ref arg_err) => write!(f, "Argument error: {}", arg_err),
            Error::ListError(ref list_err) => write!(f, "List error: {}", list_err),
            Error::CopyError(ref copy_err) => write!(f, "Copy error: {}", copy_err),
        }
    }
}

impl<'a> error::Error for Error<'a> {
    fn description(&self) -> &str {
        match *self {
            Error::NoSubcommand => "NoSubcommand",
            Error::ArgError(_) => "ArgError",
            Error::ListError(_) => "ListError",
            Error::CopyError(_) => "CopyError",
        }
    }
}
