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
// TODO: Remove These before 1.0
#![allow(unused_imports, unused_variables, dead_code, unused_mut)]

mod filesystem;
mod result;

pub use self::filesystem::{
    FileHandle, FileLockMode, FileSystem, FileSystemProvider, LocalFileHandle, LocalFileSystem,
    MemoryFileHandle, MemoryFileSystem, MetricFileSystem, MetricsFileHandle, VirtualFileHandle,
    VirtualFileSystem, VirtualFileSystemManager,
};

pub use self::result::{FileSystemError, FileSystemResult};

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
