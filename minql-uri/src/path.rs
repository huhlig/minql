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

/// URI Path
///
/// Per [Wikipedia](https://en.wikipedia.org/wiki/Uniform_Resource_Identifier):
/// > A path component, consisting of a sequence of path segments separated by a slash (/). A path
/// > is always defined for a URI, though the defined path may be empty (zero length). A segment
/// > may also be empty, resulting in two consecutive slashes (//) in the path component. A path
/// > component may resemble or map exactly to a file system path but does not always imply a
/// > relation to one. If an authority component is defined, then the path component must either
/// > be empty or begin with a slash (/). If an authority component is undefined, then the path
/// > cannot begin with an empty segment—that is, with two slashes (//)—since the following
/// > characters would be interpreted as an authority component.
/// >
/// > By convention, in http and https URIs, the last part of a path is named pathinfo and it is
/// > optional. It is composed by zero or more path segments that do not refer to an existing
/// > physical resource name (e.g. a file, an internal module program or an executable program)
/// > but to a logical part (e.g. a command or a qualifier part) that has to be passed separately
/// > to the first part of the path that identifies an executable module or program managed by a
/// > web server; this is often used to select dynamic content (a document, etc.) or to tailor it
/// > as requested (see also: CGI and PATH_INFO, etc.).
///
/// ## Example:
/// ```rust
/// // TODO: Improve Examples
/// use minql_uri::Path;
///
/// let path = Path::parse("/path/to/my/file").unwrap();
/// println!("{:?}", path);
/// ```
///
/// ## ABNF Form:
/// ```abnf
/// path          = path-abempty    ; begins with "/" or is empty
///               / path-absolute   ; begins with "/" but not "//!"
///               / path-noscheme   ; begins with a non-colon segment
///               / path-rootless   ; begins with a segment
///               / path-empty      ; zero characters
/// ```
#[derive(Debug, Default)]
pub enum Path<'str> {
    /// Zero Characters
    #[default]
    Empty,
    /// Path begins with "/" or is empty
    AbEmpty {
        /// Raw String
        raw: &'str str,
        /// Path Segments
        segments: Vec<&'str str>,
    },
    /// Path begins with "/" but not "//!"
    Absolute {
        /// Raw String
        raw: &'str str,
        /// Path Segments
        segments: Vec<&'str str>,
    },
    /// Path begins with a non-colon segment
    NoScheme {
        /// Raw String
        raw: &'str str,
        /// Path Segments
        segments: Vec<&'str str>,
    },
    /// Path begins with a segment
    Rootless {
        /// Raw String
        raw: &'str str,
        /// Path Segments
        segments: Vec<&'str str>,
    },
}

impl<'str> std::fmt::Display for Path<'str> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Path::Empty => write!(f, ""),
            Path::AbEmpty { raw, .. } => write!(f, "{raw}"),
            Path::Absolute { raw, .. } => write!(f, "{raw}"),
            Path::NoScheme { raw, .. } => write!(f, "{raw}"),
            Path::Rootless { raw, .. } => write!(f, "{raw}"),
        }
    }
}

impl<'str> Path<'str> {
    /// Convert the parsed `Path` into a `PathBuilder`
    #[must_use]
    pub fn builder(&self) -> PathBuilder {
        match self {
            Path::Empty => PathBuilder::Empty,
            Path::AbEmpty { segments, .. } => PathBuilder::Absolute {
                segments: segments.iter().map(ToString::to_string).collect(),
            },
            Path::Absolute { segments, .. } => PathBuilder::Absolute {
                segments: segments.iter().map(ToString::to_string).collect(),
            },
            Path::NoScheme { segments, .. } => PathBuilder::Absolute {
                segments: segments.iter().map(ToString::to_string).collect(),
            },
            Path::Rootless { segments, .. } => PathBuilder::Absolute {
                segments: segments.iter().map(ToString::to_string).collect(),
            },
        }
    }
}

/// URI Path Builder
#[derive(Debug, Default)]
pub enum PathBuilder {
    /// Empty Path Builder
    #[default]
    Empty,
    /// Absolute Path starting with '/'
    Absolute {
        /// Path Segments
        segments: Vec<String>,
    },
    /// Relative Path starting with './' or Empty
    Relative {
        /// Path Segments
        segments: Vec<String>,
    },
}

impl PathBuilder {
    /// Get Slices of Segments in Path
    #[must_use]
    pub fn segments(&self) -> &[String] {
        match self {
            PathBuilder::Empty => &[],
            PathBuilder::Absolute { segments, .. } => segments.as_slice(),
            PathBuilder::Relative { segments, .. } => segments.as_slice(),
        }
    }

    /// Return back parent path
    #[must_use]
    pub fn parent(&self) -> PathBuilder {
        match self {
            PathBuilder::Empty => PathBuilder::Empty,
            PathBuilder::Absolute { segments, .. } => {
                let mut segments = segments.clone();
                segments.pop();
                PathBuilder::Absolute { segments }
            }
            PathBuilder::Relative { segments, .. } => {
                let mut segments = segments.clone();
                if segments.is_empty() {
                    segments.push(String::from(".."));
                } else {
                    segments.pop();
                }
                PathBuilder::Relative { segments }
            }
        }
    }
    /// Return back a child path
    #[must_use]
    pub fn child(&self, child: &str) -> PathBuilder {
        match self {
            PathBuilder::Empty => PathBuilder::Empty,
            PathBuilder::Absolute { segments, .. } => {
                let mut segments = segments.clone();
                segments.push(String::from(child));
                PathBuilder::Absolute { segments }
            }
            PathBuilder::Relative { segments, .. } => {
                let mut segments = segments.clone();
                segments.push(String::from(child));
                PathBuilder::Relative { segments }
            }
        }
    }
}

impl std::fmt::Display for PathBuilder {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PathBuilder::Empty => write!(f, "")?,
            PathBuilder::Absolute { segments } => {
                write!(f, "/")?;
                for segment in segments {
                    write!(f, "{segment}")?;
                }
            }
            PathBuilder::Relative { segments } => {
                write!(f, "./")?;
                for segment in segments {
                    write!(f, "{segment}")?;
                }
            }
        }
        Ok(())
    }
}
