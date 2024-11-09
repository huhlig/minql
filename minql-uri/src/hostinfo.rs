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

use std::net::{Ipv4Addr, Ipv6Addr};

/// URI Host Information
///
/// Hostname or IP Address of Authority
#[derive(Debug)]
pub enum HostInfo<'str> {
    /// Named Host
    RegistryName {
        /// DNS Named Host
        raw: &'str str,
    },
    /// IPv4 Address
    IPv4Address {
        /// Raw String Address
        raw: &'str str,
        /// Parsed IPv4 Address
        ipaddr: Ipv4Addr,
    },
    /// IPv6 Address
    IPv6Address {
        /// Raw String Address
        raw: &'str str,
        /// Parsed IPv6 Address
        ipaddr: Ipv6Addr,
    },
    /// `IPvFuture` Address
    IPvFutureAddress {
        /// Raw String Address
        raw: &'str str,
    },
}

impl<'str> std::fmt::Display for HostInfo<'str> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            HostInfo::RegistryName { raw } => write!(f, "{raw}"),
            HostInfo::IPv4Address { raw, .. } => write!(f, "{raw}"),
            HostInfo::IPv6Address { raw, .. } => write!(f, "{raw}"),
            HostInfo::IPvFutureAddress { raw } => write!(f, "{raw}"),
        }
    }
}

impl<'str> HostInfo<'str> {
    /// Convert a parsed `HostInfo` into a `HostInfoBuilder`
    #[must_use]
    pub fn builder(&self) -> HostInfoBuilder {
        match self {
            HostInfo::RegistryName { raw: string } => HostInfoBuilder::RegistryName {
                hostname: (*string).to_string(),
            },
            HostInfo::IPv4Address { ipaddr, .. } => {
                HostInfoBuilder::IPv4Address { ipaddr: *ipaddr }
            }
            HostInfo::IPv6Address { ipaddr, .. } => {
                HostInfoBuilder::IPv6Address { ipaddr: *ipaddr }
            }
            HostInfo::IPvFutureAddress { raw: string } => HostInfoBuilder::IPvFutureAddress {
                address: (*string).to_string(),
            },
        }
    }
}

/// URI Host Info Builder
#[derive(Debug)]
pub enum HostInfoBuilder {
    /// Named Host
    RegistryName {
        /// DNS Named Host
        hostname: String,
    },
    /// IPv4 Address
    IPv4Address {
        /// Parsed IPv4 Address
        ipaddr: Ipv4Addr,
    },
    /// IPv6 Address
    IPv6Address {
        /// Parsed IPv6 Address
        ipaddr: Ipv6Addr,
    },
    /// `IPvFuture` Address
    IPvFutureAddress {
        /// Raw String Address
        address: String,
    },
}

impl Default for HostInfoBuilder {
    fn default() -> Self {
        HostInfoBuilder::RegistryName {
            hostname: "localhost".to_string(),
        }
    }
}

impl std::fmt::Display for HostInfoBuilder {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            HostInfoBuilder::RegistryName { hostname } => write!(f, "{hostname}"),
            HostInfoBuilder::IPv4Address { ipaddr } => write!(f, "{ipaddr}"),
            HostInfoBuilder::IPv6Address { ipaddr } => write!(f, "[{ipaddr}]"),
            HostInfoBuilder::IPvFutureAddress { address } => write!(f, "[{address}]"),
        }
    }
}
