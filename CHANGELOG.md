# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [1.0.5] - 2025-12-12

### Fixed
- **CRITICAL BUG FIX**: Fixed macro expansion to correctly use caller's package information
  - Removed `from_cargo()` method that was incorrectly using library's own package info
  - Changed `ColorfulVersion` fields from `&'static str` to `String` for proper macro expansion
  - `colorful_version!()` macro now correctly expands `env!()` at caller's location
  - This ensures users see their own app name, not "clap-version-flag"

### Changed
- `ColorfulVersion::new()` now accepts `impl Into<String>` for more flexibility
- Improved documentation with clear examples and explanations
- Added comprehensive doc comments to all public APIs
- Added `.disable_version_flag(true)` in `with_colorful_version()` to prevent conflicts

### Added
- More comprehensive test coverage
- Better examples in documentation
- Added tests for getters and custom colors

## [1.0.4] - 2025-12-11

### Added
- Initial public release
- Colorful version output for clap applications
- Support for hex colors (#RRGGBB, #RGB)
- Support for RGB tuple colors
- Default beautiful color scheme
- `colorful_version!()` macro for easy usage
- `ColorfulVersion` struct with builder pattern
- Extension trait `ColorfulVersionExt` for clap::Command
- Helper function `parse_with_version()`
- Comprehensive documentation
- Unit tests

### Known Issues (Fixed in 1.0.5)
- ❌ Bug: Shows "clap-version-flag" as package name instead of user's app name
  - **Status**: Fixed in version 1.0.5

## [Unreleased]

### Planned
- Support for more output formats (JSON, YAML)
- Support for additional information (build date, git hash)
- Theme presets (dark, light, high-contrast)
- Optional disable of colored output via environment variable

---

## Migration Guide

### From 1.0.4 to 1.0.5

**If you're using the macro (recommended way):**
```rust
// No changes needed! Just update your dependency:
// [dependencies]
// clap-version-flag = "1.0.5"

let version = colorful_version!();
```

**If you're using `from_cargo()` method:**
```rust
// OLD (1.0.4) - This was buggy!
let version = ColorfulVersion::from_cargo();

// NEW (1.0.5) - Use the macro instead
let version = colorful_version!();

// Or create manually if you need custom values
let version = ColorfulVersion::new(
    env!("CARGO_PKG_NAME"),
    env!("CARGO_PKG_VERSION"),
    env!("CARGO_PKG_AUTHORS")
);
```

## Support

If you encounter any issues or have questions:
- Open an issue on [GitHub](https://github.com/cumulus13/clap-version-flag/issues)
- Check the [documentation](https://docs.rs/clap-version-flag)
- See examples in the repository

---

[1.0.5]: https://github.com/cumulus13/clap-version-flag/compare/v1.0.4...v1.0.5
[1.0.4]: https://github.com/cumulus13/clap-version-flag/releases/tag/v1.0.4
[Unreleased]: https://github.com/cumulus13/clap-version-flag/compare/v1.0.5...HEAD