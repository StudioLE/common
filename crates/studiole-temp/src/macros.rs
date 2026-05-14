/// Create a new [`TempDir`] under the caller's crate name, test name, and current datetime.
///
/// - Path layout: `{temp_dir}/{CARGO_PKG_NAME}/{test_name}/{datetime}`
/// - Expands `env!("CARGO_PKG_NAME")` at the call site so the path uses the consuming crate's package name
/// - Creates the directory on disk; returns [`Result`] from [`TempDirBuilder::create`]
///
/// ```rust
/// use studiole_temp::temp_dir;
/// let dir = temp_dir!().expect("should create directory");
/// ```
#[macro_export]
macro_rules! temp_dir {
    () => {
        $crate::TempDirBuilder::new()
            .with(env!("CARGO_PKG_NAME"))
            .with_test_name()
            .with_datetime()
            .create()
    };
}
