# clap-version-flag

[![Crates.io](https://img.shields.io/crates/v/clap-version-flag)](https://crates.io/crates/clap-version-flag)
[![Documentation](https://docs.rs/clap-version-flag/badge.svg)](https://docs.rs/clap-version-flag)
[![License](https://img.shields.io/crates/l/clap-version-flag)](LICENSE)
[![CI Status](https://github.com/cumulus13/clap-version-flag/workflows/CI/badge.svg)](https://github.com/cumulus13/clap-version-flag/actions)

Adding colorful version output to clap applications.

## Features

- 🎨 **Hex Color Support**: Use hex color codes (`#RRGGBB` or `#RGB`) for all text elements
- 🔧 **clap v4 Integration**: Seamlessly works with clap's derive and builder APIs
- 📦 **Automatic Cargo.toml Detection**: Reads package info from environment variables
- 🚀 **Production Ready**: Comprehensive error handling, testing, and documentation
- 🌈 **Graceful Fallback**: Works in terminals with and without color support
- ⚙️ **Feature Flags**: Optional `no-color` feature for environments without color support

## Installation

```toml
[dependencies]
clap-version-flag = "1.0"
clap = { version = "4.4", features = ["derive"] }
```

## Quick Start

```rust
use clap::{Parser, CommandFactory};
use clap_version_flag::{colorful_version, parse_with_version};

#[derive(Parser)]
struct Cli {
    input: String,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let version = colorful_version!();
    let cli: Cli = parse_with_version(Cli::command(), &version)?;
    
    println!("Processing: {}", cli.input);
    Ok(())
}
```

Run with `--version` to see:
```
myapp v1.0.0 by Author Name
```

With colors (when terminal supports it):
- **`myapp`**: White text (`#FFFFFF`) on purple background (`#AA00FF`)
- **`v1.0.0`**: Yellow text (`#FFFF00`)
- **`by Author Name`**: Cyan text (`#00FFFF`)

## Custom Colors

```rust
use clap_version_flag::colorful_version;

// Use custom hex colors
let version = colorful_version!(
    "#FFFFFF",   // Name foreground
    "#AA00FF",   // Name background  
    "#FFFF00",   // Version color
    "#00FFFF"    // Author color
);
```

## Advanced Usage

### Using the Extension Trait

```rust
use clap::{Parser, CommandFactory};
use clap_version_flag::{colorful_version, ColorfulVersionExt};

#[derive(Parser)]
struct Cli {
    // ... fields
}

fn main() {
    let version = colorful_version!();
    let cmd = Cli::command().with_colorful_version(&version);
    
    let matches = cmd.get_matches();
    
    if matches.get_flag("clap_version_flag_version") {
        version.print_and_exit();
    }
    
    // ... rest of your program
}
```

### RGB Colors

```rust
use clap_version_flag::ColorfulVersion;

let version = ColorfulVersion::from_cargo()
    .with_rgb_colors(
        (255, 255, 255),  // Name foreground
        (170, 0, 255),    // Name background
        (255, 255, 0),    // Version
        (0, 255, 255),    // Author
    );
```

## API Reference

### Main Types

- `ColorfulVersion`: Configuration for colorful version output
- `VersionError`: Error type for color parsing and I/O errors
- `ColorfulVersionExt`: Extension trait for `clap::Command`

### Key Methods

- `ColorfulVersion::from_cargo()`: Create from Cargo.toml env vars
- `ColorfulVersion::with_hex_colors()`: Set custom hex colors
- `ColorfulVersion::print_and_exit()`: Print version and exit
- `parse_with_version()`: Parse clap arguments with version handling

### Macros

- `colorful_version!()`: Create `ColorfulVersion` with optional custom colors

## Feature Flags

- **`no-color`**: Disable colored output (uses plain text only)
- **`derive`**: Enable derive macro support (requires `clap/derive`)

## Testing

```bash
# Run all tests
cargo test

# Test without color support
NO_COLOR=1 cargo test

# Test examples
cargo run --example basic -- --version
cargo run --example custom_colors -- --help
```

## Error Handling

All errors are properly typed using `thiserror`:

- `VersionError::InvalidHexColor`: Invalid hex color format
- `VersionError::IoError`: I/O errors during printing

## Contributing

Contributions are welcome! Please ensure all code:
1. Passes `cargo fmt` and `cargo clippy`
2. Includes appropriate tests
3. Updates documentation as needed

## License

Dual-licensed under either:
- MIT License ([LICENSE-MIT](LICENSE-MIT))
- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE))

## Acknowledgments

- Built on top of the excellent [clap](https://crates.io/crates/clap) crate
- Uses [colored](https://crates.io/crates/colored) for terminal colors
- Inspired by the need for more visually distinct CLI version output


## Author

**Hadi Cahyadi**
- Email: cumulus13@gmail.com
- GitHub: [cumulus13](https://github.com/cumulus13)

[![Buy Me a Coffee](https://www.buymeacoffee.com/assets/img/custom_images/orange_img.png)](https://www.buymeacoffee.com/cumulus13)

[![Donate via Ko-fi](https://ko-fi.com/img/githubbutton_sm.svg)](https://ko-fi.com/cumulus13)
 
[Support me on Patreon](https://www.patreon.com/cumulus13)
