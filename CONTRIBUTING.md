# Contributing to clap-version-flag

Thank you for your interest in contributing! 🎉

We love contributions from the community. This document will guide you through the contribution process.

---

## 📋 Table of Contents

- [Code of Conduct](#code-of-conduct)
- [How Can I Contribute?](#how-can-i-contribute)
- [Development Setup](#development-setup)
- [Coding Standards](#coding-standards)
- [Testing Guidelines](#testing-guidelines)
- [Submitting Changes](#submitting-changes)
- [Release Process](#release-process)

---

## 📜 Code of Conduct

This project follows the [Rust Code of Conduct](https://www.rust-lang.org/policies/code-of-conduct). By participating, you agree to uphold this code.

**In short:**
- Be respectful and inclusive
- Be collaborative
- Be patient and understanding
- Focus on what's best for the community

---

## 🤝 How Can I Contribute?

### Reporting Bugs

Found a bug? Please create an issue with:

1. **Clear title** - Describe the problem concisely
2. **Description** - What happened vs what you expected
3. **Steps to reproduce** - Minimal code example
4. **Environment** - OS, Rust version, terminal info
5. **Screenshots** - If applicable

**Example:**
```markdown
**Title:** Colors not showing in Windows Terminal

**Description:**
When I run `myapp --version` in Windows Terminal, I see plain text instead of colors.

**Steps to Reproduce:**
1. Install clap-version-flag 1.0.5
2. Create basic example from README
3. Run in Windows Terminal 1.18

**Environment:**
- OS: Windows 11
- Rust: 1.75.0
- Terminal: Windows Terminal 1.18
- Clap: 4.5.0

**Expected:** Colored output
**Actual:** Plain text output
```

### Suggesting Features

Have an idea? Open an issue with:

1. **Use case** - What problem does it solve?
2. **Proposed solution** - How should it work?
3. **Alternatives** - What other options did you consider?
4. **Examples** - Show code examples if possible

### Improving Documentation

Documentation improvements are always welcome:
- Fix typos or unclear explanations
- Add examples
- Improve API documentation
- Translate to other languages

### Contributing Code

See [Development Setup](#development-setup) below.

---

## 🛠️ Development Setup

### Prerequisites

- Rust 1.70.0 or later
- Git
- A text editor (VS Code, RustRover, etc.)

### Clone the Repository

```bash
git clone https://github.com/cumulus13/clap-version-flag.git
cd clap-version-flag
```

### Build the Project

```bash
# Build
cargo build

# Build with all features
cargo build --all-features

# Build examples
cargo build --examples
```

### Run Tests

```bash
# Run all tests
cargo test

# Run tests with output
cargo test -- --nocapture

# Run specific test
cargo test test_name

# Run ignored tests (examples that need cargo)
cargo test -- --ignored

# Run with all features
cargo test --all-features
```

### Check Code Quality

```bash
# Format code
cargo fmt

# Check formatting
cargo fmt --check

# Run clippy
cargo clippy --all-targets --all-features

# Check documentation
cargo doc --no-deps --all-features
```

### Run Examples

```bash
cargo run --example basic -- --version
cargo run --example custom_colors
cargo run --example full_integration -- --help
```

---

## 📏 Coding Standards

### General Guidelines

1. **Follow Rust conventions**
   - Use `snake_case` for functions and variables
   - Use `PascalCase` for types and traits
   - Use `SCREAMING_SNAKE_CASE` for constants

2. **Write clear, self-documenting code**
   ```rust
   // ✅ Good
   fn parse_hex_color(hex: &str) -> Result<(u8, u8, u8), ColorError>
   
   // ❌ Bad
   fn parse(s: &str) -> Result<(u8, u8, u8), E>
   ```

3. **Keep functions small and focused**
   - Each function should do one thing well
   - If it's too long, split it up

4. **Use descriptive variable names**
   ```rust
   // ✅ Good
   let foreground_color = parse_hex("#FFFFFF")?;
   
   // ❌ Bad
   let fc = parse_hex("#FFFFFF")?;
   ```

### Documentation Standards

Every public item must have documentation:

```rust
/// Brief one-line description
///
/// Longer description if needed. Explain what it does,
/// when to use it, and any important details.
///
/// # Arguments
/// * `param1` - Description of param1
/// * `param2` - Description of param2
///
/// # Returns
/// Description of return value
///
/// # Errors
/// When does it return an error?
///
/// # Examples
/// ```
/// use clap_version_flag::ColorfulVersion;
///
/// let version = ColorfulVersion::new("app", "1.0", "author");
/// assert_eq!(version.package_name(), "app");
/// ```
///
/// # Panics
/// When does it panic? (if applicable)
pub fn my_function(param1: &str, param2: u32) -> Result<String, Error> {
    // Implementation
}
```

### Error Handling

1. **Use Result types** - Don't panic unless truly unrecoverable
   ```rust
   // ✅ Good
   pub fn parse_hex(hex: &str) -> Result<(u8, u8, u8), VersionError>
   
   // ❌ Bad
   pub fn parse_hex(hex: &str) -> (u8, u8, u8)  // Could panic!
   ```

2. **Provide helpful error messages**
   ```rust
   // ✅ Good
   return Err(VersionError::InvalidHexColor(
       format!("Invalid format '{}'. Expected #RRGGBB or #RGB", hex)
   ));
   
   // ❌ Bad
   return Err(VersionError::Invalid);
   ```

3. **Use custom error types** - Already defined in `src/error.rs`

### Testing Standards

1. **Test public APIs** - Every public function needs tests
2. **Test edge cases** - Empty strings, max values, unicode, etc.
3. **Test error cases** - Verify errors are returned correctly
4. **Use descriptive test names**
   ```rust
   #[test]
   fn test_hex_parsing_with_short_format() { }
   
   #[test]
   fn test_hex_parsing_rejects_invalid_length() { }
   ```

---

## 🧪 Testing Guidelines

### Test Structure

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_functionality() {
        // Arrange
        let input = "test";
        
        // Act
        let result = function_under_test(input);
        
        // Assert
        assert_eq!(result, expected);
    }
    
    #[test]
    fn test_error_handling() {
        let result = function_under_test("invalid");
        assert!(result.is_err());
    }
}
```

### Test Coverage

We aim for high test coverage:
- ✅ All public functions tested
- ✅ All error paths tested
- ✅ Edge cases covered
- ✅ Integration tests for main workflows

### Running Coverage

```bash
# Install tarpaulin
cargo install cargo-tarpaulin

# Generate coverage report
cargo tarpaulin --out Html --output-dir coverage
```

---

## 📤 Submitting Changes

### Fork & Clone

1. Fork the repository on GitHub
2. Clone your fork locally
3. Add upstream remote

```bash
git clone https://github.com/YOUR_USERNAME/clap-version-flag.git
cd clap-version-flag
git remote add upstream https://github.com/cumulus13/clap-version-flag.git
```

### Create a Branch

```bash
# Update main branch
git checkout main
git pull upstream main

# Create feature branch
git checkout -b feature/your-feature-name

# Or for bug fixes
git checkout -b fix/bug-description
```

**Branch naming:**
- `feature/` - New features
- `fix/` - Bug fixes
- `docs/` - Documentation only
- `refactor/` - Code refactoring
- `test/` - Adding tests

### Make Your Changes

1. **Write code** following our standards
2. **Add tests** for new functionality
3. **Update documentation** if needed
4. **Run tests** to ensure nothing breaks
   ```bash
   cargo test --all-features
   cargo clippy --all-targets --all-features
   cargo fmt --check
   ```

### Commit Your Changes

Use clear, descriptive commit messages:

```bash
# Format: <type>: <description>
git commit -m "feat: add RGB color support"
git commit -m "fix: correct hex parsing for 3-digit codes"
git commit -m "docs: improve README examples"
git commit -m "test: add edge case tests for color parsing"
```

**Commit types:**
- `feat:` - New feature
- `fix:` - Bug fix
- `docs:` - Documentation
- `test:` - Tests
- `refactor:` - Code refactoring
- `style:` - Formatting
- `chore:` - Maintenance

### Push and Create Pull Request

```bash
# Push to your fork
git push origin feature/your-feature-name
```

Then create a pull request on GitHub with:

1. **Clear title** - What does this PR do?
2. **Description** - Why is this change needed?
3. **Related issues** - Link to issues it fixes
4. **Testing** - How was it tested?
5. **Screenshots** - If UI changes

**PR Template:**
```markdown
## Description
Brief description of what this PR does.

## Related Issues
Fixes #123

## Changes Made
- Added feature X
- Fixed bug Y
- Updated documentation

## Testing
- [ ] All tests pass
- [ ] Added new tests
- [ ] Manually tested

## Checklist
- [ ] Code follows project style
- [ ] Documentation updated
- [ ] Tests added/updated
- [ ] CHANGELOG.md updated (for significant changes)
```

### Review Process

1. Maintainers will review your PR
2. Address any feedback or requested changes
3. Once approved, maintainers will merge

**Tips:**
- Keep PRs focused on a single change
- Respond to feedback promptly
- Be patient - reviews take time

---

## 🚀 Release Process

(For maintainers)

### Version Numbers

We follow [Semantic Versioning](https://semver.org/):
- **Major (X.0.0)** - Breaking changes
- **Minor (0.X.0)** - New features, backwards compatible
- **Patch (0.0.X)** - Bug fixes

### Release Checklist

1. **Update version** in `Cargo.toml`
2. **Update CHANGELOG.md** with release notes
3. **Run full test suite**
   ```bash
   cargo test --all-features
   cargo clippy --all-targets --all-features
   cargo fmt --check
   cargo doc --no-deps --all-features
   ```
4. **Build examples**
   ```bash
   cargo build --examples
   ```
5. **Create git tag**
   ```bash
   git tag -a v1.0.5 -m "Release version 1.0.5"
   git push origin v1.0.5
   ```
6. **Publish to crates.io**
   ```bash
   cargo publish --dry-run
   cargo publish
   ```
7. **Create GitHub release** with CHANGELOG excerpt

---

## 🎯 Areas That Need Help

Looking for ways to contribute? These areas always need attention:

### High Priority
- [ ] More example applications
- [ ] Performance benchmarks
- [ ] Platform-specific testing (Windows, macOS, Linux)
- [ ] Terminal compatibility testing

### Medium Priority
- [ ] Theme presets (Nord, Dracula, Monokai, etc.)
- [ ] JSON/YAML output format support
- [ ] Build date and git hash support
- [ ] More color validation tests

### Nice to Have
- [ ] VS Code extension for color preview
- [ ] Online color picker tool
- [ ] Video tutorials
- [ ] Translations

---

## 💬 Communication

### Where to Ask Questions

- **GitHub Issues** - Bug reports, feature requests
- **GitHub Discussions** - Questions, ideas, showcase
- **Discord/Slack** - Real-time chat (if we create one)

### Getting Help

Stuck? Don't hesitate to ask:
- Open a discussion on GitHub
- Comment on related issues
- Reach out to maintainers

---

## 🏆 Recognition

Contributors will be:
- Listed in CHANGELOG.md
- Mentioned in release notes
- Added to CONTRIBUTORS.md (if we create one)

Significant contributions may earn you:
- Collaborator status
- Reviewer privileges
- Maintainer role

---

## 📚 Additional Resources

### Learning Rust
- [The Rust Book](https://doc.rust-lang.org/book/)
- [Rust by Example](https://doc.rust-lang.org/rust-by-example/)
- [Rustlings](https://github.com/rust-lang/rustlings)

### Clap Documentation
- [Clap Book](https://docs.rs/clap/)
- [Clap Examples](https://github.com/clap-rs/clap/tree/master/examples)

### Terminal Colors
- [ANSI Escape Codes](https://en.wikipedia.org/wiki/ANSI_escape_code)
- [Colored Crate](https://docs.rs/colored/)

---

## 📜 License

By contributing, you agree that your contributions will be dual-licensed under MIT and Apache-2.0, matching the project's license.

---

## 🙏 Thank You!

Every contribution, no matter how small, makes this project better. We appreciate your time and effort!

**Happy coding!** 🚀

---

**Questions?** Open a [GitHub Discussion](https://github.com/cumulus13/clap-version-flag/discussions) or email: cumulus13@gmail.com