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

/// URI Scheme
#[derive(Debug)]
pub enum Scheme<'str> {
    /// HTTP Scheme
    HTTP,
    /// HTTPS Scheme
    HTTPS,
    /// Other Scheme
    Other(&'str str),
}

impl<'str> Scheme<'str> {
    /// Convert a parsed `Scheme` into a `SchemeBuilder`
    #[must_use]
    pub fn builder(&self) -> SchemeBuilder {
        match self {
            Scheme::HTTP => SchemeBuilder::HTTP,
            Scheme::HTTPS => SchemeBuilder::HTTPS,
            Scheme::Other(str) => SchemeBuilder::Other(String::from(*str)),
        }
    }
}

impl<'str> std::fmt::Display for Scheme<'str> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Scheme::HTTP => write!(f, "http"),
            Scheme::HTTPS => write!(f, "https"),
            Scheme::Other(str) => write!(f, "{str}"),
        }
    }
}

impl<'str> AsRef<str> for Scheme<'str> {
    fn as_ref(&self) -> &str {
        match self {
            Scheme::HTTP => "http",
            Scheme::HTTPS => "https",
            Scheme::Other(str) => str,
        }
    }
}

/// URI Scheme Builder
#[derive(Debug)]
pub enum SchemeBuilder {
    /// HTTP Scheme
    HTTP,
    /// HTTPS Scheme
    HTTPS,
    /// Other Scheme
    Other(String),
}

impl Default for SchemeBuilder {
    fn default() -> Self {
        SchemeBuilder::Other(String::from("scheme"))
    }
}

impl std::fmt::Display for SchemeBuilder {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SchemeBuilder::HTTP => write!(f, "http"),
            SchemeBuilder::HTTPS => write!(f, "https"),
            SchemeBuilder::Other(str) => write!(f, "{str}"),
        }
    }
}

impl AsRef<str> for SchemeBuilder {
    fn as_ref(&self) -> &str {
        match self {
            SchemeBuilder::HTTP => "http",
            SchemeBuilder::HTTPS => "https",
            SchemeBuilder::Other(str) => str,
        }
    }
}
