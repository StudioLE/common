use std::fs::remove_dir_all;
use std::ops::Deref;
use std::path::{Path, PathBuf};

/// RAII handle for a temporary directory with automatic cleanup on drop.
pub struct TempDir {
    path: PathBuf,
    remove: bool,
}

impl TempDir {
    /// Create a new [`TempDir`] from a path and cleanup flag.
    pub(crate) fn new(path: PathBuf, remove: bool) -> TempDir {
        Self { path, remove }
    }
}

impl Deref for TempDir {
    type Target = Path;

    fn deref(&self) -> &Self::Target {
        &self.path
    }
}

impl AsRef<Path> for TempDir {
    fn as_ref(&self) -> &Path {
        &self.path
    }
}

impl Drop for TempDir {
    fn drop(&mut self) {
        if self.remove {
            let _ = remove_dir_all(&self.path);
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::TempDirBuilder;
    use crate::temp_dir;
    use std::fs::remove_dir_all;

    #[test]
    fn temp_dir_drop() {
        // Arrange
        let path;
        {
            let dir = TempDirBuilder::new()
                .with("studiole-temp-test")
                .with_test_name()
                .with_datetime()
                .create()
                .expect("should create directory");
            path = dir.to_path_buf();
            assert!(path.exists());
        }
        // Assert
        assert!(!path.exists());
    }

    #[test]
    fn temp_dir_without_remove() {
        // Arrange
        let dir = TempDirBuilder::new()
            .with("studiole-temp-test")
            .with_test_name()
            .with_datetime()
            .without_remove()
            .create()
            .expect("should create directory");
        let path = dir.to_path_buf();
        // Act
        drop(dir);
        // Assert
        assert!(path.exists());
        let _ = remove_dir_all(&path);
    }

    #[test]
    fn temp_dir_macro() {
        let dir = temp_dir!().expect("should create directory");
        let components: Vec<_> = dir.iter().collect();
        assert!(
            components
                .iter()
                .any(|c| c.to_str().is_some_and(|s| s.starts_with("studiole-temp")))
        );
    }
}
