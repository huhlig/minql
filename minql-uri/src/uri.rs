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

use crate::{Authority, Fragment, Path, Query, Scheme};

/// Uniform Resource Identifier
///
/// ```rust
/// // TODO: Improve URIReference Example
/// use minql_uri::URIReference;
///
/// let uri_ref = URIReference::parse("https://example.com:12345/path/to/my/resource").unwrap();
/// println!("{:?}", uri_ref); 
/// ```
#[derive(Debug)]
pub enum URIReference<'str> {
    /// Absolute URI
    Absolute(URI<'str>),
    /// URI Relative Reference
    Relative(URIRelativeReference<'str>),
}

/// Uniform Resource Identifier
/// 
/// ```rust
/// // TODO: Improve URI Example
/// use minql_uri::URI;
///
/// let uri = URI::parse("https://example.com:12345/path/to/my/resource").unwrap();
/// println!("{:?}", uri); 
/// ```
#[derive(Debug)]
pub struct URI<'str> {
    /// Unparsed URI String
    pub string: &'str str,
    /// URI String
    pub scheme: Scheme<'str>,
    /// URI Authority
    pub authority: Option<Authority<'str>>,
    /// URI Path
    pub path: Path<'str>,
    /// URI Query
    pub query: Option<Query<'str>>,
    /// URI Fragment
    pub fragment: Option<Fragment<'str>>,
}

/// Uniform Resource Identifier Relative Reference
///
/// ```rust
/// // TODO: Improve URI Example
/// use minql_uri::URIRelativeReference;
///
/// let uri = URIRelativeReference::parse("//example.com:12345/path/to/my/resource").unwrap();
/// println!("{:?}", uri); 
/// ```
#[derive(Debug)]
pub struct URIRelativeReference<'str> {
    /// Unparsed URI String
    pub string: &'str str,
    /// URI Authority
    pub authority: Option<Authority<'str>>,
    /// URI Path
    pub path: Path<'str>,
    /// URI Query
    pub query: Option<Query<'str>>,
    /// URI Fragment
    pub fragment: Option<Fragment<'str>>,
}