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

use crate::hostinfo::HostInfoBuilder;
use crate::userinfo::UserInfoBuilder;
use crate::{hostinfo::HostInfo, userinfo::UserInfo};

/// Uniform Resource Authority
///
/// Per [Wikipedia](https://en.wikipedia.org/wiki/Uniform_Resource_Identifier):
/// > An optional authority component preceded by two slashes (//), comprising:
/// > * An optional userinfo subcomponent followed by an at symbol (@), that may consist of a
/// user-name and an optional password preceded by a colon (:). Use of the format username:password
/// in the userinfo subcomponent is deprecated for security reasons. Applications should not render
/// as clear text any data after the first colon (:) found within a userinfo subcomponent unless the
/// data after the colon is the empty string (indicating no password).
/// > * A host subcomponent, consisting of either a registered name (including but not limited to a
/// hostname) or an IP address. IPv4 addresses must be in dot-decimal notation, and IPv6 addresses
/// must be enclosed in brackets ([]).
/// > * An optional port subcomponent preceded by a colon (:), consisting of decimal digits.  
///
/// ## ABNF Grammar
/// ```abnf
/// authority = [userinfo "@"] host [":" port]
/// ```
#[derive(Debug)]
pub struct Authority<'str> {
    /// Raw unparsed Authority String
    pub raw: &'str str,
    /// Authority User Information
    pub userinfo: Option<UserInfo<'str>>,
    /// Authority Host Information
    pub hostinfo: HostInfo<'str>,
    /// Authority Port Number
    pub port: Option<u16>,
}

impl<'str> Authority<'str> {
    /// Convert Parsed Authority into a Builder
    #[must_use]  
    pub fn builder(&self) -> AuthorityBuilder {
        AuthorityBuilder {
            userinfo: self.userinfo.as_ref().map(UserInfo::builder),
            hostinfo: self.hostinfo.builder(),
            port: self.port,
        }
    }
}

impl<'str> std::fmt::Display for Authority<'str> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.raw)
    }
}

/// URI Authority Builder
#[derive(Debug)]
pub struct AuthorityBuilder {
    /// Authority User Information
    pub userinfo: Option<UserInfoBuilder>,
    /// Authority Host Information
    pub hostinfo: HostInfoBuilder,
    /// Authority Port Number
    pub port: Option<u16>,
}

impl std::fmt::Display for AuthorityBuilder {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Some(ui) = &self.userinfo {
            write!(f, "{ui}@")?;
        }
        write!(f, "{}", self.hostinfo)?;
        if let Some(port) = &self.port {
            write!(f, ":{}", port)?;
        }
        Ok(())
    }
}

impl Default for AuthorityBuilder {
    fn default() -> Self {
        AuthorityBuilder {
            userinfo: None,
            hostinfo: HostInfoBuilder::RegistryName {
                hostname: String::default(),
            },
            port: None,
        }
    }
}
