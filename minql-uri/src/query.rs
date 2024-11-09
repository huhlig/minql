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

use crate::utility::{pct_decode, pct_encode};

/// Query
///
/// Per [Wikipedia](https://en.wikipedia.org/wiki/Uniform_Resource_Identifier):
/// > An optional query component preceded by a question mark (?), consisting of a query string of
/// > non-hierarchical data. Its syntax is not well-defined, but by convention is most often a
/// > sequence of attribute–value pairs separated by a delimiter.
///
/// ```bnf
/// query     ::= parameter [ [';' | '&'] parameter]
/// parameter ::= key '=' value
/// key       ::= non-reserved
/// value     ::= non-reserved
/// ```
#[derive(Debug)]
pub struct Query<'str> {
    /// Raw Unparsed Query String
    pub raw: &'str str,
    /// Query Parameters Split by `&` or ';' and parameters split by `=`
    pub parameters: Vec<(&'str str, Option<&'str str>)>,
}

impl<'str> Query<'str> {
    /// Get Pct Decoded Raw `Query`.
    ///
    /// # Panics
    /// May panic if parsing has a bug.
    #[must_use]
    pub fn raw(&self) -> String {
        pct_decode(self.raw).unwrap()
    }
    /// Get Pct Decoded `Query` parameters.
    ///
    /// # Panics
    /// May panic if parsing has a bug.
    #[must_use]
    pub fn parameters(&self) -> Vec<(String, Option<String>)> {
        self.parameters
            .iter()
            .map(|(k, v)| (pct_decode(k).unwrap(), v.map(|v| pct_decode(v).unwrap())))
            .collect()
    }
    /// Convert a parsed `Query` into a `QueryBuilder`
    #[must_use]
    pub fn builder(&self) -> QueryBuilder {
        QueryBuilder {
            parameters: self
                .parameters
                .iter()
                .map(|(key, value)| ((*key).to_string(), value.map(ToString::to_string)))
                .collect(),
        }
    }
}

impl<'str> std::fmt::Display for Query<'str> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.raw)
    }
}

/// Query Builder
#[derive(Debug, Default)]
pub struct QueryBuilder {
    /// Query Parameters Split by `&` or ';' and parameters split by `=`
    pub parameters: Vec<(String, Option<String>)>,
}

impl std::fmt::Display for QueryBuilder {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut iter = self.parameters.iter().peekable();
        while let Some((key, value)) = iter.next() {
            pct_encode(f, key)?;
            if let Some(value) = value {
                write!(f, "=")?;
                pct_encode(f, value)?;
            }
            if iter.peek().is_some() {
                write!(f, "&")?;
            }
        }
        Ok(())
    }
}
