use crate::TempDir;
use chrono::Local;
use std::env::temp_dir;
use std::fs::create_dir_all;
use std::io;
use std::path::PathBuf;
use std::sync::atomic::{AtomicU64, Ordering};
use std::thread;

/// Counter appended to the final path segment by [`TempDirBuilder::build`]
/// and [`TempDirBuilder::create`] to guarantee unique paths within a process.
///
/// - The first call returns `0` and no suffix is appended
/// - Subsequent calls append `-{n}` to the final path segment
static COUNTER: AtomicU64 = AtomicU64::new(0);

/// Builder for a [`TempDir`] rooted at the system temp directory.
pub struct TempDirBuilder {
    path: PathBuf,
    remove: bool,
}

impl TempDirBuilder {
    /// Create a new [`TempDirBuilder`] rooted at the system temp directory.
    #[must_use]
    pub fn new() -> Self {
        Self {
            path: temp_dir(),
            remove: true,
        }
    }

    /// Append a path component.
    #[must_use]
    pub fn with(mut self, name: &str) -> Self {
        self.path.push(name);
        self
    }

    /// Append the current test name as a path component.
    #[must_use]
    pub fn with_test_name(mut self) -> Self {
        self.path.push(get_test_name_underscored());
        self
    }

    /// Append the current datetime as a path component.
    #[must_use]
    pub fn with_datetime(mut self) -> Self {
        self.path.push(get_datetime());
        self
    }

    /// Disable cleanup on drop, useful for debugging test failures.
    #[must_use]
    pub fn without_remove(mut self) -> Self {
        self.remove = false;
        self
    }

    /// Append the [`COUNTER`] value to the final path segment when non-zero, then increment it.
    fn append_number(&mut self) {
        let counter = COUNTER.fetch_add(1, Ordering::Relaxed);
        if counter == 0 {
            return;
        }
        let mut file_name = self
            .path
            .file_name()
            .expect("should have file name")
            .to_os_string();
        file_name.push(format!("-{counter}"));
        self.path.set_file_name(file_name);
    }

    /// Build the [`TempDir`] RAII handle without touching the filesystem.
    ///
    /// - Appends `-{n}` to the final path segment when [`COUNTER`] is non-zero
    /// - See [`TempDirBuilder::create`] to also create the directory on disk
    pub fn build(mut self) -> Result<TempDir, io::Error> {
        self.append_number();
        Ok(TempDir::new(self.path, self.remove))
    }

    /// Create the directory on disk and return the [`TempDir`] RAII handle.
    ///
    /// - Appends `-{n}` to the final path segment when [`COUNTER`] is non-zero
    /// - See [`TempDirBuilder::build`] to skip filesystem creation
    pub fn create(mut self) -> Result<TempDir, io::Error> {
        self.append_number();
        create_dir_all(&self.path)?;
        Ok(TempDir::new(self.path, self.remove))
    }
}

fn get_datetime() -> String {
    Local::now().format("%Y-%m-%dT%H.%M.%S").to_string()
}

fn get_test_name_underscored() -> String {
    thread::current()
        .name()
        .expect("should be able to get test name")
        .replace(':', "_")
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::env::temp_dir;

    #[test]
    fn temp_dir_builder_with() {
        let builder = TempDirBuilder::new().with("foo").with("bar");
        assert_eq!(builder.path, temp_dir().join("foo").join("bar"));
    }

    #[test]
    fn temp_dir_builder_with_test_name() {
        let builder = TempDirBuilder::new().with_test_name();
        assert!(
            builder
                .path
                .ends_with("temp_dir_builder__tests__temp_dir_builder_with_test_name")
        );
    }

    #[test]
    fn temp_dir_builder_with_datetime() {
        // Act
        let builder = TempDirBuilder::new().with_datetime();
        let component = builder
            .path
            .file_name()
            .expect("should have file name")
            .to_str()
            .expect("should be valid utf-8");
        // Assert
        assert!(component.contains('T'));
        assert!(component.contains('.'));
    }

    #[test]
    fn temp_dir_builder_create() {
        // Act
        let dir = TempDirBuilder::new()
            .with("studiole-temp-test")
            .with_test_name()
            .with_datetime()
            .create()
            .expect("should create directory");
        // Assert
        assert!(dir.is_dir());
    }

    #[test]
    fn temp_dir_builder_build_once() {
        // Act
        let dir = TempDirBuilder::new()
            .with("studiole-temp-test")
            .with_test_name()
            .with_datetime()
            .without_remove()
            .build()
            .expect("should build directory");
        // Assert
        assert!(!dir.exists());
    }

    #[test]
    fn temp_dir_builder_build_twice() {
        // Arrange
        let first = TempDirBuilder::new()
            .with("studiole-temp-test")
            .with_test_name()
            .without_remove()
            .build()
            .expect("should build directory");
        // Act
        let second = TempDirBuilder::new()
            .with("studiole-temp-test")
            .with_test_name()
            .without_remove()
            .build()
            .expect("should build directory");
        // Assert
        assert_ne!(first.to_path_buf(), second.to_path_buf());
    }
}
