# clap-version-flag

[![Crates.io](https://img.shields.io/crates/v/clap-version-flag.svg)](https://crates.io/crates/clap-version-flag)
[![Documentation](https://docs.rs/clap-version-flag/badge.svg)](https://docs.rs/clap-version-flag)
[![License](https://img.shields.io/crates/l/clap-version-flag.svg)](https://github.com/cumulus13/clap-version-flag#license)

Adding colorful version output to clap applications.

## Features

- 🎨 **Hex Color Support**: Use hex color codes (`#RRGGBB` or `#RGB`) for all text elements
- 🔧 **clap Integration**: Seamlessly works with clap's derive and builder APIs
- 📦 **Automatic Cargo.toml Detection**: Reads package info from environment variables
- 🚀 **Production Ready**: Comprehensive error handling, testing, and documentation
- 🌈 **Graceful Fallback**: Works in terminals with and without color support
- ⚙️ **Feature Flags**: Optional `no-color` feature for environments without color support
  🔧 **Flexible integration** - Works with clap derive and builder patterns
 🚀 **Zero-config defaults** - Beautiful colors out of the box

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
clap = { version = "4.5", features = ["derive"] }
clap-version-flag = "1.0"
```

## Quick Start

### Basic Usage

```rust
use clap::Parser;
use clap_version_flag::colorful_version;

#[derive(Parser)]
#[command(name = "myapp")]
struct Cli {
    #[arg(short, long)]
    input: String,
}

fn main() {
    // Create version info from your Cargo.toml
    let version = colorful_version!();
    
    // Parse arguments
    let cli = Cli::parse();
    
    // Your app logic here
    println!("Input: {}", cli.input);
}
```

When a user runs `myapp --version`, they'll see:

```
myapp v1.0.0 by Your Name
```

With colors (when terminal supports it):
- **`myapp`**: White text (`#FFFFFF`) on purple background (`#AA00FF`)
- **`v1.0.0`**: Yellow text (`#FFFF00`)
- **`by Author Name`**: Cyan text (`#00FFFF`)

### Custom Colors

```rust
use clap_version_flag::colorful_version;

fn main() {
    // Custom hex colors: name_fg, name_bg, version, author
    let version = colorful_version!(
        "#FF0000",  // Red foreground for name
        "#0000FF",  // Blue background for name
        "#00FF00",  // Green for version
        "#FFFF00"   // Yellow for author
    );
    
    version.print();
}
```

### Manual Version Creation

```rust
use clap_version_flag::ColorfulVersion;

fn main() {
    let version = ColorfulVersion::new("myapp", "1.0.0", "John Doe")
        .with_hex_colors("#FFFFFF", "#AA00FF", "#FFFF00", "#00FFFF")
        .expect("Invalid hex colors");
    
    version.print();
}
```

### Using RGB Colors

```rust
use clap_version_flag::ColorfulVersion;

fn main() {
    let version = ColorfulVersion::new("myapp", "1.0.0", "John Doe")
        .with_rgb_colors(
            (255, 255, 255),  // White foreground
            (170, 0, 255),    // Purple background
            (255, 255, 0),    // Yellow version
            (0, 255, 255)     // Cyan author
        );
    
    version.print();
}
```

## Integration with Clap

### Method 1: Using `parse_with_version` (Recommended)

This automatically handles the `--version` flag:

```rust
use clap::{Parser, CommandFactory};
use clap_version_flag::{colorful_version, parse_with_version};

#[derive(Parser)]
struct Cli {
    #[arg(short, long)]
    name: String,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let version = colorful_version!();
    let cli: Cli = parse_with_version(Cli::command(), &version)?;
    
    println!("Hello, {}!", cli.name);
    Ok(())
}
```

### Method 2: Manual Flag Checking

```rust
use clap::{Parser, CommandFactory};
use clap_version_flag::{colorful_version, ColorfulVersionExt};

#[derive(Parser)]
struct Cli {
    #[arg(short, long)]
    name: String,
}

fn main() {
    let version = colorful_version!();
    
    let matches = Cli::command()
        .with_colorful_version(&version)
        .get_matches();
    
    // Check if version flag was used
    version.check_and_exit(&matches);
    
    // Continue with normal parsing
    let cli = Cli::from_arg_matches(&matches).unwrap();
    println!("Hello, {}!", cli.name);
}
```

### Method 3: Builder Pattern

```rust
use clap::Command;
use clap_version_flag::{colorful_version, ColorfulVersionExt};

fn main() {
    let version = colorful_version!();
    
    let matches = Command::new("myapp")
        .with_colorful_version(&version)
        .arg(clap::arg!(-n --name <NAME> "Your name"))
        .get_matches();
    
    version.check_and_exit(&matches);
    
    if let Some(name) = matches.get_one::<String>("name") {
        println!("Hello, {}!", name);
    }
}
```

## Default Colors

The default color scheme is designed for maximum readability:

- **Package Name**: White text (`#FFFFFF`) on purple background (`#AA00FF`)
- **Version**: Yellow text (`#FFFF00`)
- **Author**: Cyan text (`#00FFFF`)

## API Reference

### `colorful_version!()`

Macro to create a `ColorfulVersion` using information from your `Cargo.toml`.

**Important**: This macro uses `env!()` which expands at the **caller's location**, so it correctly picks up your package information, not this library's.

```rust
// With default colors
let version = colorful_version!();

// With custom colors
let version = colorful_version!("#FFFFFF", "#AA00FF", "#FFFF00", "#00FFFF");
```

### `ColorfulVersion`

Main struct for version configuration.

#### Methods

- `new(name, version, author)` - Create with custom values
- `with_hex_colors(name_fg, name_bg, version, author)` - Set colors using hex codes
- `with_rgb_colors(name_fg, name_bg, version, author)` - Set colors using RGB tuples
- `print()` - Print colored version to stdout
- `print_and_exit()` - Print and exit with code 0
- `as_plain_string()` - Get plain text version
- `to_colored_string()` - Get colored version with ANSI codes
- `package_name()`, `version()`, `author()` - Getters

### `ColorfulVersionExt`

Trait extension for `clap::Command`.

```rust
use clap_version_flag::ColorfulVersionExt;

let cmd = Command::new("myapp").with_colorful_version(&version);
```

### `parse_with_version()`

Helper function to parse command-line arguments with automatic version handling.

```rust
use clap_version_flag::parse_with_version;

let cli: YourCli = parse_with_version(YourCli::command(), &version)?;
```

## Color Format Support

### Hex Colors

- **6-digit**: `#RRGGBB` (e.g., `#FF0000` for red)
- **3-digit**: `#RGB` (e.g., `#F00` for red, expands to `#FF0000`)
- **Without #**: `RRGGBB` (automatically prepended)

### RGB Tuples

```rust
(255, 0, 0)  // Red
(0, 255, 0)  // Green
(0, 0, 255)  // Blue
```

## Testing

Run tests:

```bash
cargo test
```

Run tests with output:

```bash
cargo test -- --nocapture
```

## Examples

Check the `examples/` directory for complete working examples:

```bash
cargo run --example basic
cargo run --example custom_colors
cargo run --example derive_pattern
```

## Why This Crate?

Clap's built-in `--version` flag outputs plain text. This crate enhances the user experience with:

1. **Professional appearance** - Colored output makes your CLI tool stand out
2. **Consistent branding** - Use your brand colors in the version output
3. **Zero boilerplate** - Automatic extraction from `Cargo.toml`
4. **Terminal compatibility** - Uses `colored` crate with proper terminal detection

## Comparison

**Without clap-version-flag:**
```
$ myapp --version
myapp 1.0.0
```

**With clap-version-flag:**
```
$ myapp --version
myapp v1.0.0 by Your Name
```
(But with beautiful colors that respect your terminal theme!)

## How It Works

The `colorful_version!()` macro uses Rust's `env!()` macro which expands at compile time **at the caller's location**. This means:

1. When you use `colorful_version!()` in your project
2. The macro expands to read `CARGO_PKG_NAME`, `CARGO_PKG_VERSION`, and `CARGO_PKG_AUTHORS`
3. These environment variables are set by Cargo during compilation
4. They contain **your** project's information, not this library's

This is why the macro correctly picks up your package information!

## Common Issues

### Issue: Wrong package name displayed

**Cause**: Using an old version that had the bug.

**Solution**: Update to version 1.0.4 or later:
```toml
clap-version-flag = "1.0.4"
```

### Issue: Version flag not working

**Cause**: Need to disable clap's built-in version flag.

**Solution**: The `with_colorful_version()` extension trait automatically calls `.disable_version_flag(true)`.

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

1. Fork the repository
2. Create your feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -m 'Add some amazing feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request

## License

This project is dual-licensed under either:

- MIT License ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)
- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)

at your option.

## Acknowledgments

- [clap](https://github.com/clap-rs/clap) - Command Line Argument Parser for Rust
- [colored](https://github.com/mackwic/colored) - Coloring terminal output

## Author

**Hadi Cahyadi**
- Email: cumulus13@gmail.com
- GitHub: [cumulus13](https://github.com/cumulus13)

[![Buy Me a Coffee](https://www.buymeacoffee.com/assets/img/custom_images/orange_img.png)](https://www.buymeacoffee.com/cumulus13)

[![Donate via Ko-fi](https://ko-fi.com/img/githubbutton_sm.svg)](https://ko-fi.com/cumulus13)
 
[Support me on Patreon](https://www.patreon.com/cumulus13)
