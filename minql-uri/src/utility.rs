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

pub(crate) fn pct_encode(f: &mut std::fmt::Formatter<'_>, value: &str) -> std::fmt::Result {
    for ch in value.chars() {
        match ch as u8 {
            b'0'..=b'9' | b'A'..=b'Z' | b'a'..=b'z' | b'-' | b'.' | b'_' | b'~' => {
                write!(f, "{ch}")?;
            }
            n => {
                write!(f, "%{n:02X}")?;
            }
        }
    }
    Ok(())
}

/// Decodes a percent-encoded URI component.
///
/// This function takes a percent-encoded string slice and returns a decoded `String`.
/// The function emits errors if the input contains invalid percent-encoding sequences.
///
/// # Errors
///
/// Returns an `Err` if an invalid percent encoding sequence is found.
pub(crate) fn pct_decode(s: &str) -> Result<String, std::num::ParseIntError> {
    let mut result = String::with_capacity(s.len());
    let mut chars = s.chars();

    while let Some(ch) = chars.next() {
        if ch == '%' {
            let hex = chars
                .next()
                .and_then(|c1| chars.next().map(|c2| format!("{c1}{c2}")))
                .unwrap_or_default();
            if hex.len() == 2 {
                let byte = u8::from_str_radix(&hex, 16)?;
                result.push(byte as char);
            } else {
                result.push('%');
                result.push_str(&hex);
            }
        } else {
            result.push(ch);
        }
    }

    Ok(result)
}
