# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [1.0.0] - 2025-12-10

### Added
- Initial release
- Support for hex colors (#RRGGBB and #RGB formats)
- Automatic Cargo.toml detection via env macros
- Integration with Clap v4
- Proper error handling with thiserror
- Comprehensive test suite
- Feature flags (no-color support)
- CI/CD with GitHub Actions
- Examples and documentation

### Features
- `colorful_version!()` macro for easy initialization
- `ColorfulVersionExt` trait for Command integration
- `parse_with_version()` helper function
- Custom hex color validation
- Graceful fallback for terminals without color support