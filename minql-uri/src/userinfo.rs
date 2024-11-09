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
    /// Convert a parsed `UserInfo` into a `UserInfoBuilder`
    #[must_use]
    pub fn builder(&self) -> UserInfoBuilder {
        match self {
            UserInfo::Unparsed { raw } => UserInfoBuilder {
                username: String::from(*raw),
                password: None,
            },
            UserInfo::Parsed {
                username, password, ..
            } => UserInfoBuilder {
                username: String::from(*username),
                password: password.map(|p| String::from(p)),
            },
        }
    }
}

impl<'str> std::fmt::Display for UserInfo<'str> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            UserInfo::Unparsed { raw } => write!(f, "{raw}"),
            UserInfo::Parsed { raw, .. } => write!(f, "{raw}"),
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
        write!(f, "{}", self.username)?;
        if let Some(password) = &self.password {
            write!(f, ":{}", password)?;
        }
        Ok(())
    }
}
