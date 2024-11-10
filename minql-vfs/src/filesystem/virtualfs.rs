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

use crate::filesystem::{DynamicFileSystem, DynamicFileSystemProvider, FileSystemProvider};
use crate::{FileHandle, FileLockMode, FileSystem, FileSystemError, FileSystemResult};
use minql_uri::URI;
use std::collections::HashMap;
use std::io::{Read, Seek, SeekFrom, Write};
use std::sync::{Arc, RwLock};

/// Virtual FileSystem Manager
#[derive(Debug, Default)]
pub struct VirtualFileSystemManager(RwLock<HashMap<String, Arc<dyn DynamicFileSystemProvider>>>);

impl VirtualFileSystemManager {
    /// Register a new Filesystem Provider
    #[tracing::instrument(level = "trace")]
    pub fn register<T: FileSystemProvider>(&self, provider: T) -> FileSystemResult<()> {
        let mut lock = self.0.write().unwrap();
        let provider = Arc::new(provider);
        for scheme in provider.schemes().iter() {
            lock.insert(scheme.to_string(), provider.clone());
        }
        Ok(())
    }

    /// Get Filesystem for Path
    #[tracing::instrument(level = "trace")]
    pub fn get(&self, path: &str) -> FileSystemResult<VirtualFileSystem> {
        let lock = self.0.read().unwrap();
        let uri = URI::parse(path).map_err(|a| FileSystemError::WrappedError(Box::new(a)))?;
        let provider = lock
            .get(uri.scheme.to_string().as_str())
            .ok_or(FileSystemError::UnknownFileSystem)?;
        Ok(VirtualFileSystem(provider.provision(path)?))
    }
}

/// Virtual `FileSystem` Handle
#[derive(Debug)]
pub struct VirtualFileSystem(Arc<dyn DynamicFileSystem>);

impl VirtualFileSystem {
    /// Create a new Virtual Filesystem around a Filesystem implementation.
    #[inline]
    #[tracing::instrument(level = "trace")]
    pub fn new<F: FileSystem>(filesystem: F) -> VirtualFileSystem {
        Self(Arc::new(filesystem))
    }
}

impl FileSystem for VirtualFileSystem {
    type FileHandle = VirtualFileHandle;

    #[inline]
    #[tracing::instrument(level = "trace")]
    fn exists(&self, path: &str) -> FileSystemResult<bool> {
        DynamicFileSystem::exists(self.0.as_ref(), path)
    }

    #[inline]
    #[tracing::instrument(level = "trace")]
    fn is_file(&self, path: &str) -> FileSystemResult<bool> {
        DynamicFileSystem::is_file(self.0.as_ref(), path)
    }

    #[inline]
    #[tracing::instrument(level = "trace")]
    fn is_directory(&self, path: &str) -> FileSystemResult<bool> {
        DynamicFileSystem::is_directory(self.0.as_ref(), path)
    }

    #[inline]
    #[tracing::instrument(level = "trace")]
    fn filesize(&self, path: &str) -> FileSystemResult<u64> {
        DynamicFileSystem::filesize(self.0.as_ref(), path)
    }

    #[inline]
    #[tracing::instrument(level = "trace")]
    fn create_directory(&self, path: &str) -> FileSystemResult<()> {
        DynamicFileSystem::create_directory(self.0.as_ref(), path)
    }

    #[inline]
    #[tracing::instrument(level = "trace")]
    fn create_directory_all(&self, path: &str) -> FileSystemResult<()> {
        DynamicFileSystem::create_directory_all(self.0.as_ref(), path)
    }

    #[inline]
    #[tracing::instrument(level = "trace")]
    fn list_directory<'a>(&self, path: &str) -> FileSystemResult<Vec<String>> {
        DynamicFileSystem::list_directory(self.0.as_ref(), path)
    }

    #[inline]
    #[tracing::instrument(level = "trace")]
    fn remove_directory(&self, path: &str) -> FileSystemResult<()> {
        DynamicFileSystem::remove_directory(self.0.as_ref(), path)
    }

    #[inline]
    #[tracing::instrument(level = "trace")]
    fn remove_directory_all(&self, path: &str) -> FileSystemResult<()> {
        DynamicFileSystem::remove_directory_all(self.0.as_ref(), path)
    }

    #[inline]
    #[tracing::instrument(level = "trace")]
    fn create_file(&self, path: &str) -> FileSystemResult<Self::FileHandle> {
        Ok(VirtualFileHandle(DynamicFileSystem::create_file(
            self.0.as_ref(),
            path,
        )?))
    }

    #[inline]
    #[tracing::instrument(level = "trace")]
    fn open_file(&self, path: &str) -> FileSystemResult<Self::FileHandle> {
        Ok(VirtualFileHandle(DynamicFileSystem::open_file(
            self.0.as_ref(),
            path,
        )?))
    }

    #[inline]
    #[tracing::instrument(level = "trace")]
    fn remove_file(&self, path: &str) -> FileSystemResult<()> {
        DynamicFileSystem::remove_file(self.0.as_ref(), path)
    }
}

/// Virtual File Handle
pub struct VirtualFileHandle(Box<dyn FileHandle>);

impl std::fmt::Debug for VirtualFileHandle {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Debug::fmt(self.0.as_ref(), f)
    }
}

impl Read for VirtualFileHandle {
    #[inline]
    #[tracing::instrument(level = "trace")]
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        Read::read(self.0.as_mut(), buf)
    }
}

impl Write for VirtualFileHandle {
    /// Write Data to a `VirtualFilesystem` `FileHandle`
    #[inline]
    #[tracing::instrument(level = "trace")]
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        Write::write(self.0.as_mut(), buf)
    }

    #[inline]
    #[tracing::instrument(level = "trace")]
    fn flush(&mut self) -> std::io::Result<()> {
        Write::flush(self.0.as_mut())
    }
}

impl Seek for VirtualFileHandle {
    #[inline]
    #[tracing::instrument(level = "trace")]
    fn seek(&mut self, pos: SeekFrom) -> std::io::Result<u64> {
        Seek::seek(self.0.as_mut(), pos)
    }
}

impl FileHandle for VirtualFileHandle {
    #[inline]
    #[tracing::instrument(level = "trace")]
    fn path(&self) -> &str {
        FileHandle::path(self.0.as_ref())
    }

    #[inline]
    #[tracing::instrument(level = "trace")]
    fn get_size(&self) -> FileSystemResult<u64> {
        FileHandle::get_size(self.0.as_ref())
    }

    #[inline]
    #[tracing::instrument(level = "trace")]
    fn set_size(&mut self, new_size: u64) -> FileSystemResult<()> {
        FileHandle::set_size(self.0.as_mut(), new_size)
    }

    #[inline]
    #[tracing::instrument(level = "trace")]
    fn sync_all(&mut self) -> FileSystemResult<()> {
        FileHandle::sync_all(self.0.as_mut())
    }

    #[inline]
    #[tracing::instrument(level = "trace")]
    fn sync_data(&mut self) -> FileSystemResult<()> {
        FileHandle::sync_data(self.0.as_mut())
    }

    #[inline]
    #[tracing::instrument(level = "trace")]
    fn get_lock_status(&self) -> FileSystemResult<FileLockMode> {
        FileHandle::get_lock_status(self.0.as_ref())
    }

    #[inline]
    #[tracing::instrument(level = "trace")]
    fn set_lock_status(&mut self, mode: FileLockMode) -> FileSystemResult<()> {
        FileHandle::set_lock_status(self.0.as_mut(), mode)
    }
}

#[cfg(test)]
mod test {
    use crate::MemoryFileSystem;
    use std::time::{SystemTime, UNIX_EPOCH};

    #[test]
    #[tracing_test::traced_test]
    fn test_virtual_filesystem() {
        use crate::{
            FileHandle, FileSystem, FileSystemError, FileSystemResult, VirtualFileSystem,
            VirtualFileSystemManager,
        };
        use std::io::{Read, Seek, SeekFrom, Write};

        let fs = VirtualFileSystem::new(MemoryFileSystem::default());
        let filename = format!(
            "./test-{}.tst",
            SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .expect("Time went backwards")
                .as_nanos()
        );
        {
            // Create new File
            let mut file = fs
                .create_file(filename.as_str())
                .expect("Error Creating File");
            assert_eq!(file.get_size().unwrap(), 0, "File size wasn't zero");

            // Write to File
            file.write_all(b"Hello, World!").unwrap();
            assert_eq!(file.get_size().unwrap(), 13, "File size wasn't 13");

            // Read full File Contents and compare
            let mut buf = Vec::new();
            file.seek(SeekFrom::Start(0))
                .expect("Error Seeking to beginning of file");
            file.read_to_end(&mut buf).expect("Error Reading File");
            assert_eq!(buf, b"Hello, World!");

            // Shrink file to size 5 and test
            file.set_size(5).expect("Error Setting File Size");
            assert_eq!(file.get_size().unwrap(), 5);

            // Seek to start and read full file
            let mut buf = Vec::new();
            file.seek(SeekFrom::Start(0)).expect("Error Seeking File");
            file.read_to_end(&mut buf).expect("Error Reading File");
            assert_eq!(buf, b"Hello");

            // Set file size to zero and test
            file.set_size(0).unwrap();
            assert_eq!(file.get_size().expect("Unable to get file size"), 0);

            // Write new data to file and test
            file.seek(SeekFrom::Start(0))
                .expect("Error Seeking to beginning of file");
            file.write_all(b"Goodbye!").expect("Error Writing File");
            assert_eq!(file.get_size().expect("Unable to get file size"), 8);

            // Seek to start and read full file
            let mut buf = Vec::new();
            file.seek(SeekFrom::Start(0)).expect("Error Seeking File");
            file.read_to_end(&mut buf).expect("Error Reading File");
            assert_eq!(buf, b"Goodbye!");
        }
        {
            // Open existing file and test
            let mut file = fs.open_file(filename.as_str()).unwrap();
            assert_eq!(file.get_size().unwrap(), 8);

            // Seek to start and read full file
            let mut buf = Vec::new();
            file.seek(SeekFrom::Start(0)).expect("Error Seeking File");
            file.read_to_end(&mut buf).expect("Error Reading File");
            assert_eq!(buf, b"Goodbye!");
        }

        // Remove file and test
        fs.remove_file(filename.as_str())
            .expect("Error Removing File");
        assert!(!fs
            .exists(filename.as_str())
            .expect("Error Checking File Existence"));
    }
}
