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

//! URI Parsing Library
//! ```rust
//! // TODO: Improve Intro Example
//! use minql_uri::URI;
//!
//! let uri = URI::parse("https://www.example.com/").unwrap();
//! println!("{:?}", uri);
//! ```
//!
#![doc = include_str!("uri_abnf.md")]
//!
//! * TODO: Improve Documentation and Examples
//! * TODO: Add builder pattern to manipulate URIs
//!

#![forbid(unsafe_code)]
#![warn(
    clippy::cargo,
    missing_docs,
    clippy::pedantic,
    future_incompatible,
    rust_2018_idioms
)]
#![allow(
    clippy::option_if_let_else,
    clippy::module_name_repetitions,
    clippy::missing_errors_doc
)]

pub use self::authority::{Authority, AuthorityBuilder};
pub use self::fragment::{Fragment, FragmentBuilder};
pub use self::hostinfo::{HostInfo, HostInfoBuilder};
pub use self::path::{Path, PathBuilder};
pub use self::query::{Query, QueryBuilder};
pub use self::result::{URIError, URIResult};
pub use self::scheme::{Scheme, SchemeBuilder};
pub use self::uri::{
    URIBuilder, URIReference, URIReferenceBuilder, URIRelativeReference,
    URIRelativeReferenceBuilder, URI,
};
pub use self::userinfo::{UserInfo, UserInfoBuilder};

mod authority;
mod fragment;
mod hostinfo;
mod parser;
mod path;
mod query;
mod result;
mod scheme;
mod uri;
mod userinfo;
mod utility;
