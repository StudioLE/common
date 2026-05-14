# studiole-common

Shared utility crates for StudioLE Rust projects.

## [`studiole-temp`](crates/studiole-temp)

`temp_dir!()` macro for creating unique temporary directories following a standard pattern:

`{temp_dir}/{CARGO_PKG_NAME}/{test_name}/{datetime}`
