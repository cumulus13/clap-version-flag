// Project: clap-version-flag
// File: src/macros.rs
// Author: Hadi Cahyadi <cumulus13@gmail.com>
// Date: 2025-12-12
// Description: Main and helper macros for clap-version-flag
// License: MIT

//! Macros for creating and working with `ColorfulVersion`
//!
//! This module contains the main `colorful_version!` macro and
//! additional helper macros for ease of use.

/// Main macro for easy creation of ColorfulVersion from Cargo.toml
///
/// **IMPORTANT**: This macro uses `env!()` which expands at the caller's location,
/// so it will correctly pick up the caller's package information, not this library's.
///
/// # Examples
/// ```
/// use clap_version_flag::colorful_version;
///
/// // With default colors - gets info from YOUR Cargo.toml
/// let version = colorful_version!();
/// println!("{}", version); // Prints: your-app-name v1.0.0 by Your Name
///
/// // With custom hex colors
/// let version = colorful_version!("#FFFFFF", "#AA00FF", "#FFFF00", "#00FFFF");
/// ```
#[macro_export]
macro_rules! colorful_version {
    () => {
        // env!() expands at the CALLER's location, not here!
        // This means it will read from the caller's Cargo.toml
        $crate::ColorfulVersion::new(
            env!("CARGO_PKG_NAME"),
            env!("CARGO_PKG_VERSION"),
            env!("CARGO_PKG_AUTHORS"),
        )
    };

    ($name_fg:expr, $name_bg:expr, $version:expr, $author:expr) => {
        $crate::ColorfulVersion::new(
            env!("CARGO_PKG_NAME"),
            env!("CARGO_PKG_VERSION"),
            env!("CARGO_PKG_AUTHORS"),
        )
        .with_hex_colors($name_fg, $name_bg, $version, $author)
        .unwrap_or_else(|e| panic!("clap-version-flag: Invalid hex color format: {}", e))
    };
}

/// Macro to create a colorful version with complete configuration
///
/// # Examples
///
/// ## Create with custom values
/// ```
/// use clap_version_flag::colorful_version_full;
///
/// let version = colorful_version_full!("myapp", "1.0.0", "John Doe");
/// assert_eq!(version.package_name(), "myapp");
/// ```
///
/// ## Create with custom values and colors
/// ```
/// use clap_version_flag::colorful_version_full;
///
/// let version = colorful_version_full!(
///     "myapp", "1.0.0", "John Doe",
///     "#FFFFFF", "#AA00FF", "#FFFF00", "#00FFFF"
/// );
/// ```
#[macro_export]
macro_rules! colorful_version_full {
    ($name:expr, $version:expr, $author:expr) => {
        $crate::ColorfulVersion::new($name, $version, $author)
    };

    ($name:expr, $version:expr, $author:expr,
     $name_fg:expr, $name_bg:expr, $version_color:expr, $author_color:expr) => {
        $crate::ColorfulVersion::new($name, $version, $author)
            .with_hex_colors($name_fg, $name_bg, $version_color, $author_color)
            .expect("Invalid hex color format")
    };
}

/// Macro to create a colorful version with RGB colors
///
/// # Examples
/// ```
/// use clap_version_flag::colorful_version_rgb;
///
/// let version = colorful_version_rgb!(
///     (255, 255, 255),  // name foreground
///     (170, 0, 255),    // name background
///     (255, 255, 0),    // version color
///     (0, 255, 255)     // author color
/// );
/// ```
#[macro_export]
macro_rules! colorful_version_rgb {
    ($name_fg:expr, $name_bg:expr, $version:expr, $author:expr) => {
        $crate::ColorfulVersion::new(
            env!("CARGO_PKG_NAME"),
            env!("CARGO_PKG_VERSION"),
            env!("CARGO_PKG_AUTHORS"),
        )
        .with_rgb_colors($name_fg, $name_bg, $version, $author)
    };
}

/*
NOTE: derive_colorful_version macro is commented out for now.
It requires more complex implementation with trait bounds.
Will be added in future version if there's demand.

/// Macro for derive helper to add colorful version to your CLI struct
///
/// This is useful if you want to add version functionality directly to your CLI struct.
///
/// # Examples
/// ```ignore
/// use clap::Parser;
/// use clap_version_flag::derive_colorful_version;
///
/// #[derive(Parser)]
/// struct Cli {
///     name: String,
/// }
///
/// derive_colorful_version!(Cli);
///
/// fn main() {
///     let version = Cli::colorful_version();
///     version.print();
/// }
/// ```
#[macro_export]
macro_rules! derive_colorful_version {
    ($struct_name:ident) => {
        impl $struct_name {
            /// Get colorful version for this CLI application
            ///
            /// This method returns a `ColorfulVersion` instance using the
            /// information from your `Cargo.toml` file.
            pub fn colorful_version() -> $crate::ColorfulVersion {
                $crate::colorful_version!()
            }

            /// Parse command with automatic colorful version handling
            ///
            /// This method combines parsing and version handling in one call.
            /// If the user passes `--version` or `-V`, it will print the
            /// colorful version and exit.
            ///
            /// # Errors
            ///
            /// Returns an error if clap fails to parse the arguments.
            pub fn parse_with_colorful_version() -> Result<Self, clap::Error>
            where
                Self: clap::FromArgMatches + clap::CommandFactory,
            {
                let version = Self::colorful_version();
                $crate::parse_with_version(Self::command(), &version)
            }
        }
    };
}
*/

#[cfg(test)]
mod tests {
    // use crate::ColorfulVersion;

    #[test]
    fn test_colorful_version_full_without_colors() {
        let version = colorful_version_full!("testapp", "1.0.0", "Test Author");
        assert_eq!(version.package_name(), "testapp");
        assert_eq!(version.version(), "1.0.0");
        assert_eq!(version.author(), "Test Author");
    }

    #[test]
    fn test_colorful_version_full_with_colors() {
        let version = colorful_version_full!(
            "testapp",
            "1.0.0",
            "Test Author",
            "#FFFFFF",
            "#AA00FF",
            "#FFFF00",
            "#00FFFF"
        );
        assert_eq!(version.package_name(), "testapp");
    }

    #[test]
    fn test_colorful_version_rgb() {
        let version = colorful_version_rgb!((255, 0, 0), (0, 255, 0), (0, 0, 255), (255, 255, 0));
        // This will use env! so it gets the crate's own name in tests
        assert_eq!(version.package_name(), env!("CARGO_PKG_NAME"));
    }
}
