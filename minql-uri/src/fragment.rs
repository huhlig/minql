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

/// # URI Fragment
///
/// Per [Wikipedia](https://en.wikipedia.org/wiki/Uniform_Resource_Identifier):
/// > An optional fragment component preceded by a hash (#). The fragment contains a fragment
/// > identifier providing direction to a secondary resource, such as a section heading in an
/// > article identified by the remainder of the URI. When the primary resource is an HTML document,
/// > the fragment is often an id attribute of a specific element, and web browsers will scroll this
/// > element into view.
///
/// ```bnf
/// fragment ::= <non-reserved>
/// ```
///
#[derive(Debug)]
pub struct Fragment<'str> {
    /// Fragment Value
    pub fragment: &'str str,
}

impl<'str> std::fmt::Display for Fragment<'str> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.fragment)
    }
}

impl<'str> Fragment<'str> {
    /// Get Pct Decoded Fragment
    ///
    /// # Panics
    /// May Panic if Parser has a bug.
    #[must_use]
    pub fn fragment(&self) -> String {
        pct_decode(self.fragment).unwrap()
    }
    /// Convert Parsed `Fragment` into a `FragmentBuilder`
    #[must_use]
    pub fn builder(&self) -> FragmentBuilder {
        FragmentBuilder {
            fragment: self.fragment.to_string(),
        }
    }
}

/// URI Fragment Builder
#[derive(Debug, Default)]
pub struct FragmentBuilder {
    /// Fragment Value
    pub fragment: String,
}

impl std::fmt::Display for FragmentBuilder {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        pct_encode(f, self.fragment.as_str())
    }
}
