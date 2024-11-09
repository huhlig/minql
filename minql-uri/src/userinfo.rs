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
use std::fmt::Write;

/// URI User Information
#[derive(Debug)]
pub enum UserInfo<'str> {
    /// Unparsed User Information
    Unparsed {
        /// Raw `UserInfo` String
        raw: &'str str,
    },
    /// Parsed User Information
    Parsed {
        /// Raw `UserInfo` String
        raw: &'str str,
        /// Username
        username: &'str str,
        /// Optional Password
        password: Option<&'str str>,
    },
}

impl<'str> UserInfo<'str> {
    /// Get Pct Decoded Raw `UserInfo`.
    ///
    /// # Panics
    /// May panic if parsing has a bug.
    #[must_use]
    pub fn raw(&self) -> String {
        match self {
            UserInfo::Unparsed { raw, .. } | UserInfo::Parsed { raw, .. } => {
                pct_decode(raw).unwrap()
            }
        }
    }
    /// Get Pct Decoded username. If no password is present, raw is assumed to be username.
    ///
    /// # Panics
    /// May panic if parsing has a bug.
    #[must_use]
    pub fn username(&self) -> String {
        match self {
            UserInfo::Unparsed { raw, .. } => pct_decode(raw).unwrap(),
            UserInfo::Parsed { username, .. } => pct_decode(username).unwrap(),
        }
    }
    /// Get Pct Decoded password if present.
    ///
    /// # Panics
    /// May panic if parsing has a bug.
    #[must_use]
    pub fn password(&self) -> Option<String> {
        match self {
            UserInfo::Unparsed { .. } => None,
            UserInfo::Parsed { password, .. } => password.map(|p| pct_decode(p).unwrap()),
        }
    }
    /// Convert a parsed `UserInfo` into a `UserInfoBuilder`
    #[must_use]
    pub fn builder(&self) -> UserInfoBuilder {
        match self {
            UserInfo::Unparsed { raw } => UserInfoBuilder {
                username: pct_decode(raw).unwrap_or_default(),
                password: None,
            },
            UserInfo::Parsed {
                username, password, ..
            } => UserInfoBuilder {
                username: pct_decode(username).unwrap_or_default(),
                password: password.map(|p| pct_decode(p).unwrap_or_default()),
            },
        }
    }
}

impl<'str> std::fmt::Display for UserInfo<'str> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            UserInfo::Unparsed { raw } | UserInfo::Parsed { raw, .. } => write!(f, "{raw}"),
        }
    }
}

/// URI User Info Builder
#[derive(Debug, Default)]
pub struct UserInfoBuilder {
    /// Username
    pub username: String,
    /// Optional Password
    pub password: Option<String>,
}

impl std::fmt::Display for UserInfoBuilder {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        pct_encode(f, self.username.as_str())?;
        if let Some(password) = &self.password {
            f.write_char(':')?;
            pct_encode(f, password.as_str())?;
        }
        Ok(())
    }
}
