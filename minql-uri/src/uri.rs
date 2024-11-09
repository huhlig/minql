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

use crate::{
    authority::Authority, fragment::Fragment, path::Path, query::Query, scheme::Scheme,
    AuthorityBuilder, FragmentBuilder, PathBuilder, QueryBuilder, SchemeBuilder,
};

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

impl<'str> URIReference<'str> {
    /// Convert Reference to a Builder
    pub fn builder(&self) -> URIReferenceBuilder {
        match self {
            URIReference::Absolute(uri) => URIReferenceBuilder::Absolute(uri.builder()),
            URIReference::Relative(uri) => URIReferenceBuilder::Relative(uri.builder()),
        }
    }
}

impl<'str> std::fmt::Display for URIReference<'str> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            URIReference::Absolute(uri) => std::fmt::Display::fmt(uri, f),
            URIReference::Relative(uri) => std::fmt::Display::fmt(uri, f),
        }
    }
}

/// URI Reference Builder
#[derive(Debug)]
pub enum URIReferenceBuilder {
    /// Absolute URI
    Absolute(URIBuilder),
    /// URI Relative Reference
    Relative(URIRelativeReferenceBuilder),
}

impl std::fmt::Display for URIReferenceBuilder {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            URIReferenceBuilder::Absolute(uri) => std::fmt::Display::fmt(uri, f),
            URIReferenceBuilder::Relative(uri) => std::fmt::Display::fmt(uri, f),
        }
    }
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
    pub raw: &'str str,
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

impl<'str> URI<'str> {
    /// Convert a parsed `URI` into a `URIBuilder`
    #[must_use]
    pub fn builder(&self) -> URIBuilder {
        URIBuilder {
            scheme: self.scheme.builder(),
            authority: self.authority.as_ref().map(Authority::builder),
            path: self.path.builder(),
            query: self.query.as_ref().map(Query::builder),
            fragment: self.fragment.as_ref().map(Fragment::builder),
        }
    }
}

impl<'str> std::fmt::Display for URI<'str> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}:", self.scheme)?;
        if let Some(authority) = self.authority.as_ref() {
            write!(f, "{}", authority)?;
        }
        write!(f, "{}", self.path)?;
        if let Some(query) = self.query.as_ref() {
            write!(f, "?{}", query)?;
        }
        if let Some(fragment) = self.fragment.as_ref() {
            write!(f, "#{}", fragment)?;
        }
        Ok(())
    }
}

/// URI Builder
#[derive(Debug, Default)]
pub struct URIBuilder {
    /// URI String
    pub scheme: SchemeBuilder,
    /// URI Authority
    pub authority: Option<AuthorityBuilder>,
    /// URI Path
    pub path: PathBuilder,
    /// URI Query
    pub query: Option<QueryBuilder>,
    /// URI Fragment
    pub fragment: Option<FragmentBuilder>,
}

impl std::fmt::Display for URIBuilder {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}:", self.scheme)?;
        if let Some(authority) = self.authority.as_ref() {
            write!(f, "{}", authority)?;
        }
        write!(f, "{}", self.path)?;
        if let Some(query) = self.query.as_ref() {
            write!(f, "?{}", query)?;
        }
        if let Some(fragment) = self.fragment.as_ref() {
            write!(f, "#{}", fragment)?;
        }
        Ok(())
    }
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
    pub raw: &'str str,
    /// URI Authority
    pub authority: Option<Authority<'str>>,
    /// URI Path
    pub path: Path<'str>,
    /// URI Query
    pub query: Option<Query<'str>>,
    /// URI Fragment
    pub fragment: Option<Fragment<'str>>,
}

impl<'str> URIRelativeReference<'str> {
    /// Convert a parsed `URIRelativeReference` into a `URIRelativeReferenceBuilder`
    #[must_use]
    pub fn builder(&self) -> URIRelativeReferenceBuilder {
        URIRelativeReferenceBuilder {
            authority: self.authority.as_ref().map(Authority::builder),
            path: self.path.builder(),
            query: self.query.as_ref().map(Query::builder),
            fragment: self.fragment.as_ref().map(Fragment::builder),
        }
    }
}

impl<'str> std::fmt::Display for URIRelativeReference<'str> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Some(authority) = self.authority.as_ref() {
            write!(f, "{}", authority)?;
        }
        write!(f, "{}", self.path)?;
        if let Some(query) = self.query.as_ref() {
            write!(f, "?{}", query)?;
        }
        if let Some(fragment) = self.fragment.as_ref() {
            write!(f, "#{}", fragment)?;
        }
        Ok(())
    }
}

/// URI Relative Reference Builder
#[derive(Debug, Default)]
pub struct URIRelativeReferenceBuilder {
    /// URI Authority
    pub authority: Option<AuthorityBuilder>,
    /// URI Path
    pub path: PathBuilder,
    /// URI Query
    pub query: Option<QueryBuilder>,
    /// URI Fragment
    pub fragment: Option<FragmentBuilder>,
}

impl std::fmt::Display for URIRelativeReferenceBuilder {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Some(authority) = self.authority.as_ref() {
            write!(f, "{}", authority)?;
        }
        write!(f, "{}", self.path)?;
        if let Some(query) = self.query.as_ref() {
            write!(f, "?{}", query)?;
        }
        if let Some(fragment) = self.fragment.as_ref() {
            write!(f, "#{}", fragment)?;
        }
        Ok(())
    }
}
