//
// Copyright 2024 Hans W. Uhlig. All Rights Reserved.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//      http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.
//

use std::string::FromUtf8Error;

/// URI Parser Result type
pub type URIResult<T> = Result<T, URIError>;

/// URI Parser Error Type
#[derive(Debug, Default)]
pub enum URIError {
    /// Unknown Error
    #[default]
    Unknown,
    /// UTF8 Error
    UTF8(FromUtf8Error),
    /// Parsing Error
    Parsing(String),
}

impl std::fmt::Display for URIError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Debug::fmt(self, f)
    }
}

impl std::error::Error for URIError {}
