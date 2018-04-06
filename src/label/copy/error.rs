//
//  label/copy/error.rs
//  ghtool
//
//  Created by Søren Mortensen on 06/04/2018.
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

use hubcaps::errors::Error as HubcapsError;

use std::error::Error;
use std::fmt;
use std::io::Error as IoError;

#[derive(Debug)]
pub enum CopyError {
    IoError(IoError),
    HubcapsError(HubcapsError),
}

impl fmt::Display for CopyError {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        match *self {
            CopyError::IoError(ref io_err) => write!(f, "IO error: {}", io_err),
            CopyError::HubcapsError(ref hc_err) => write!(f, "Hubcaps error: {}", hc_err),
        }
    }
}

impl Error for CopyError {
    fn description(&self) -> &str {
        match *self {
            CopyError::IoError(_) => "IoError",
            CopyError::HubcapsError(_) => "HubcapsError",
        }
    }
}

impl From<HubcapsError> for CopyError {
    fn from(hc_err: HubcapsError) -> Self {
        CopyError::HubcapsError(hc_err)
    }
}
