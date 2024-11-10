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
use crate::{FileHandle, FileLockMode, FileSystem, FileSystemResult};
use std::collections::HashMap;
use std::io::{Read, Seek, SeekFrom, Write};
use std::ops::AddAssign;
use std::sync::{Arc, RwLock};

/// Metric Collection Filesystem Wrapper
#[derive(Debug)]
pub struct MetricFileSystem {
    metrics: FileSystemMetrics,
    inner: Arc<dyn DynamicFileSystem>,
}

impl MetricFileSystem {
    /// Create a new Metrics FileSystem
    pub fn new<F: FileSystem>(filesystem: F) -> MetricFileSystem {
        MetricFileSystem {
            metrics: FileSystemMetrics::default(),
            inner: Arc::new(filesystem),
        }
    }
    /// Get Aggregate Filesystem metrics
    pub fn filesystem_metrics(&self) -> MetricsData {
        self.metrics.filesystem_metrics()
    }
    /// Get Individual File Metrics
    pub fn file_metrics(&self) -> HashMap<String, MetricsData> {
        self.metrics.file_metrics()
    }
}

impl FileSystem for MetricFileSystem {
    type FileHandle = MetricsFileHandle;

    #[tracing::instrument(level = "debug")]
    fn exists(&self, path: &str) -> FileSystemResult<bool> {
        DynamicFileSystem::exists(self.inner.as_ref(), path)
    }

    #[tracing::instrument(level = "debug")]
    fn is_file(&self, path: &str) -> FileSystemResult<bool> {
        DynamicFileSystem::is_file(self.inner.as_ref(), path)
    }

    #[tracing::instrument(level = "debug")]
    fn is_directory(&self, path: &str) -> FileSystemResult<bool> {
        DynamicFileSystem::is_directory(self.inner.as_ref(), path)
    }

    #[tracing::instrument(level = "debug")]
    fn filesize(&self, path: &str) -> FileSystemResult<u64> {
        DynamicFileSystem::filesize(self.inner.as_ref(), path)
    }

    #[tracing::instrument(level = "debug")]
    fn create_directory(&self, path: &str) -> FileSystemResult<()> {
        DynamicFileSystem::create_directory(self.inner.as_ref(), path)
    }

    #[tracing::instrument(level = "debug")]
    fn create_directory_all(&self, path: &str) -> FileSystemResult<()> {
        DynamicFileSystem::create_directory_all(self.inner.as_ref(), path)
    }

    #[tracing::instrument(level = "debug")]
    fn list_directory<'a>(&self, path: &str) -> FileSystemResult<Vec<String>> {
        DynamicFileSystem::list_directory(self.inner.as_ref(), path)
    }

    #[tracing::instrument(level = "debug")]
    fn remove_directory(&self, path: &str) -> FileSystemResult<()> {
        DynamicFileSystem::remove_directory(self.inner.as_ref(), path)
    }

    #[tracing::instrument(level = "debug")]
    fn remove_directory_all(&self, path: &str) -> FileSystemResult<()> {
        DynamicFileSystem::remove_directory_all(self.inner.as_ref(), path)
    }

    #[tracing::instrument(level = "debug")]
    fn create_file(&self, path: &str) -> FileSystemResult<Self::FileHandle> {
        Ok(MetricsFileHandle {
            metrics: self.metrics.initialize_file(path),
            inner: DynamicFileSystem::create_file(self.inner.as_ref(), path)?,
        })
    }

    #[tracing::instrument(level = "debug")]
    fn open_file(&self, path: &str) -> FileSystemResult<Self::FileHandle> {
        Ok(MetricsFileHandle {
            metrics: self.metrics.initialize_file(path),
            inner: DynamicFileSystem::open_file(self.inner.as_ref(), path)?,
        })
    }

    #[tracing::instrument(level = "debug")]
    fn remove_file(&self, path: &str) -> FileSystemResult<()> {
        DynamicFileSystem::remove_file(self.inner.as_ref(), path)
    }
}

/// Virtual File Handle
pub struct MetricsFileHandle {
    metrics: FileHandleMetrics,
    inner: Box<dyn FileHandle>,
}

impl std::fmt::Debug for MetricsFileHandle {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Debug::fmt(self.inner.as_ref(), f)
    }
}

impl Read for MetricsFileHandle {
    #[tracing::instrument(level = "debug")]
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        let rv = Read::read(self.inner.as_mut(), buf)?;
        self.metrics.read_bytes(rv as u64);
        Ok(rv)
    }
}

impl Write for MetricsFileHandle {
    #[tracing::instrument(level = "debug")]
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        let rv = Write::write(self.inner.as_mut(), buf)?;
        self.metrics.write_bytes(rv as u64);
        Ok(rv)
    }

    #[tracing::instrument(level = "debug")]
    fn flush(&mut self) -> std::io::Result<()> {
        Write::flush(self.inner.as_mut())
    }
}

impl Seek for MetricsFileHandle {
    #[tracing::instrument(level = "debug")]
    fn seek(&mut self, pos: SeekFrom) -> std::io::Result<u64> {
        Seek::seek(self.inner.as_mut(), pos)
    }
}

impl FileHandle for MetricsFileHandle {
    #[tracing::instrument(level = "debug")]
    fn path(&self) -> &str {
        FileHandle::path(self.inner.as_ref())
    }

    #[tracing::instrument(level = "debug")]
    fn get_size(&self) -> FileSystemResult<u64> {
        FileHandle::get_size(self.inner.as_ref())
    }

    #[tracing::instrument(level = "debug")]
    fn set_size(&mut self, new_size: u64) -> FileSystemResult<()> {
        FileHandle::set_size(self.inner.as_mut(), new_size)
    }

    #[tracing::instrument(level = "debug")]
    fn sync_all(&mut self) -> FileSystemResult<()> {
        FileHandle::sync_all(self.inner.as_mut())
    }

    #[tracing::instrument(level = "debug")]
    fn sync_data(&mut self) -> FileSystemResult<()> {
        FileHandle::sync_data(self.inner.as_mut())
    }

    #[tracing::instrument(level = "debug")]
    fn get_lock_status(&self) -> FileSystemResult<FileLockMode> {
        FileHandle::get_lock_status(self.inner.as_ref())
    }

    #[tracing::instrument(level = "debug")]
    fn set_lock_status(&mut self, mode: FileLockMode) -> FileSystemResult<()> {
        FileHandle::set_lock_status(self.inner.as_mut(), mode)
    }
}

/// Collection of Metrics for FileSystem
#[derive(Debug, Default)]
struct FileSystemMetrics {
    inner: Arc<RwLock<HashMap<String, FileHandleMetrics>>>,
}

impl FileSystemMetrics {
    /// Get Aggregate `FileSystem` metrics
    fn filesystem_metrics(&self) -> MetricsData {
        let mut metrics = MetricsData::default();
        for metric in self.inner.read().expect("Mutex Poisoned").values() {
            metrics.bytes_read += metric.bytes_read();
            metrics.bytes_written += metric.bytes_written();
        }
        metrics
    }
    /// Get file `MetricsData`
    fn file_metrics(&self) -> HashMap<String, MetricsData> {
        let mut metrics = HashMap::new();
        for (path, metric) in self.inner.read().expect("Mutex Poisoned").iter() {
            metrics.insert(path.clone(), metric.metrics());
        }
        metrics
    }
    /// Initialize a file, no-op if it already exists
    fn initialize_file(&self, path: &str) -> FileHandleMetrics {
        self.inner
            .write()
            .expect("Mutex Poisoned")
            .entry(path.to_string())
            .or_default()
            .clone()
    }
}

#[derive(Clone, Debug, Default)]
struct FileHandleMetrics {
    inner: Arc<RwLock<MetricsData>>,
}

impl FileHandleMetrics {
    fn metrics(&self) -> MetricsData {
        self.inner.read().expect("Mutex Poisoned").clone()
    }
    fn bytes_read(&self) -> u64 {
        self.inner.read().expect("Mutex Poisoned").bytes_read
    }
    fn bytes_written(&self) -> u64 {
        self.inner.read().expect("Mutex Poisoned").bytes_written
    }
    fn read_bytes(&self, bytes: u64) {
        self.inner
            .write()
            .expect("Mutex Poisoned")
            .bytes_read
            .add_assign(bytes);
    }
    fn write_bytes(&self, bytes: u64) {
        self.inner
            .write()
            .expect("Mutex Poisoned")
            .bytes_written
            .add_assign(bytes);
    }
}

/// Metrics Data
#[derive(Clone, Debug, Default)]
pub struct MetricsData {
    bytes_written: u64,
    bytes_read: u64,
}

#[cfg(test)]
mod test {
    use crate::MemoryFileSystem;
    use std::time::{SystemTime, UNIX_EPOCH};

    #[test]
    #[tracing_test::traced_test]
    fn test_metrics_filesystem() {
        use crate::{FileHandle, FileSystem, FileSystemError, FileSystemResult, MetricFileSystem};
        use std::io::{Read, Seek, SeekFrom, Write};

        let fs = MetricFileSystem::new(MemoryFileSystem::default());
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
