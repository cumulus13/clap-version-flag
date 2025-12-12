// Project: clap-version-flag
// File: src/lib.rs
// Author: Hadi Cahyadi <cumulus13@gmail.com>
// Date: 2025-12-12
// Description: Production-ready version with all fixes
// License: MIT

//! A production-ready crate for adding colorful version output to clap applications.
//!
//! This crate provides utilities to override the default `-V`/`--version` flag behavior
//! in clap applications with colorful output using hex color codes.
//!
//! # Examples
//!
//! ## Basic usage with derive
//! ```no_run
//! use clap::Parser;
//! use clap_version_flag::colorful_version;
//!
//! #[derive(Parser)]
//! #[command(name = "myapp")]
//! struct Cli {
//!     #[arg(short, long)]
//!     input: String,
//! }
//!
//! fn main() {
//!     let version = colorful_version!();
//!     let cli = Cli::parse();
//!     
//!     // Check version flag manually if needed
//!     // version.print(); // This would print colored version
//! }
//! ```
//!
//! ## Using with custom colors
//! ```no_run
//! use clap_version_flag::colorful_version;
//!
//! let version = colorful_version!("#FF0000", "#0000FF", "#00FF00", "#FFFF00");
//! version.print();
//! ```

mod error;
pub mod macros;

pub use error::VersionError;

use clap::{Arg, ArgAction, ArgMatches, Command, FromArgMatches};
use colored::Colorize;
use std::fmt;
use std::process;

/// Configuration for colorful version output
#[derive(Clone, Debug)]
pub struct ColorfulVersion {
    package_name: String,
    version: String,
    author: String,
    colors: Colors,
}

#[derive(Clone, Debug)]
struct Colors {
    name_fg: (u8, u8, u8),       // RGB for name foreground
    name_bg: (u8, u8, u8),       // RGB for name background
    version_color: (u8, u8, u8), // RGB for version
    author_color: (u8, u8, u8),  // RGB for author
}

impl Default for Colors {
    fn default() -> Self {
        Self {
            // Default colors as specified in requirements:
            // name: light #FFFFFF on #AA00FF
            // version: light #FFFF00
            // author: light #00FFFF
            name_fg: (255, 255, 255),     // #FFFFFF
            name_bg: (170, 0, 255),       // #AA00FF
            version_color: (255, 255, 0), // #FFFF00
            author_color: (0, 255, 255),  // #00FFFF
        }
    }
}

impl ColorfulVersion {
    /// Creates a new ColorfulVersion with custom values
    ///
    /// # Examples
    /// ```
    /// use clap_version_flag::ColorfulVersion;
    ///
    /// let version = ColorfulVersion::new("myapp", "1.0.0", "John Doe");
    /// assert_eq!(version.package_name(), "myapp");
    /// ```
    #[must_use]
    pub fn new(
        package_name: impl Into<String>,
        version: impl Into<String>,
        author: impl Into<String>,
    ) -> Self {
        Self {
            package_name: package_name.into(),
            version: version.into(),
            author: author.into(),
            colors: Colors::default(),
        }
    }

    /// Sets custom hex colors for the version output
    ///
    /// # Arguments
    /// * `name_fg` - Hex color for package name foreground (e.g., "#FFFFFF")
    /// * `name_bg` - Hex color for package name background (e.g., "#AA00FF")
    /// * `version` - Hex color for version text (e.g., "#FFFF00")
    /// * `author` - Hex color for author text (e.g., "#00FFFF")
    ///
    /// # Errors
    /// Returns `VersionError::InvalidHexColor` if any hex color is invalid
    ///
    /// # Examples
    /// ```
    /// use clap_version_flag::ColorfulVersion;
    ///
    /// let version = ColorfulVersion::new("myapp", "1.0.0", "John Doe")
    ///     .with_hex_colors("#FFFFFF", "#AA00FF", "#FFFF00", "#00FFFF")
    ///     .expect("Invalid hex colors");
    /// ```
    pub fn with_hex_colors(
        mut self,
        name_fg: &str,
        name_bg: &str,
        version: &str,
        author: &str,
    ) -> Result<Self, VersionError> {
        self.colors.name_fg = parse_hex(name_fg)?;
        self.colors.name_bg = parse_hex(name_bg)?;
        self.colors.version_color = parse_hex(version)?;
        self.colors.author_color = parse_hex(author)?;
        Ok(self)
    }

    /// Sets custom RGB colors for the version output
    ///
    /// # Examples
    /// ```
    /// use clap_version_flag::ColorfulVersion;
    ///
    /// let version = ColorfulVersion::new("myapp", "1.0.0", "John Doe")
    ///     .with_rgb_colors((255, 0, 0), (0, 0, 255), (0, 255, 0), (255, 255, 0));
    /// ```
    #[must_use]
    pub fn with_rgb_colors(
        mut self,
        name_fg: (u8, u8, u8),
        name_bg: (u8, u8, u8),
        version: (u8, u8, u8),
        author: (u8, u8, u8),
    ) -> Self {
        self.colors.name_fg = name_fg;
        self.colors.name_bg = name_bg;
        self.colors.version_color = version;
        self.colors.author_color = author;
        self
    }

    /// Prints the colorful version to stdout and exits the process
    ///
    /// # Examples
    /// ```no_run
    /// use clap_version_flag::colorful_version;
    ///
    /// let version = colorful_version!();
    /// version.print_and_exit(); // Prints and exits with code 0
    /// ```
    pub fn print_and_exit(&self) -> ! {
        self.print();
        process::exit(0);
    }

    /// Prints the colorful version to stdout
    /// Format: "{package_name} v{version} by {author}"
    ///
    /// # Examples
    /// ```
    /// use clap_version_flag::ColorfulVersion;
    ///
    /// let version = ColorfulVersion::new("myapp", "1.0.0", "John Doe");
    /// version.print(); // Prints colored output
    /// ```
    pub fn print(&self) {
        let name = self
            .package_name
            .truecolor(
                self.colors.name_fg.0,
                self.colors.name_fg.1,
                self.colors.name_fg.2,
            )
            .on_truecolor(
                self.colors.name_bg.0,
                self.colors.name_bg.1,
                self.colors.name_bg.2,
            );

        let version_text = format!(" v{}", self.version).truecolor(
            self.colors.version_color.0,
            self.colors.version_color.1,
            self.colors.version_color.2,
        );

        let author_text = format!(" by {}", self.author).truecolor(
            self.colors.author_color.0,
            self.colors.author_color.1,
            self.colors.author_color.2,
        );

        println!("{name}{version_text}{author_text}");
    }

    /// Returns a plain text version string (for clap's version flag)
    /// Format: "{package_name} v{version} by {author}"
    ///
    /// # Examples
    /// ```
    /// use clap_version_flag::ColorfulVersion;
    ///
    /// let version = ColorfulVersion::new("myapp", "1.0.0", "John Doe");
    /// assert_eq!(version.as_plain_string(), "myapp v1.0.0 by John Doe");
    /// ```
    #[must_use]
    pub fn as_plain_string(&self) -> String {
        format!("{} v{} by {}", self.package_name, self.version, self.author)
    }

    /// Returns a colored version string if terminal supports colors
    /// Format: "{package_name} v{version} by {author}"
    ///
    /// # Examples
    /// ```
    /// use clap_version_flag::ColorfulVersion;
    ///
    /// let version = ColorfulVersion::new("myapp", "1.0.0", "John Doe");
    /// let colored = version.to_colored_string();
    /// // colored string contains ANSI color codes
    /// ```
    #[must_use]
    pub fn to_colored_string(&self) -> String {
        format!(
            "{}{}{}",
            self.package_name
                .truecolor(
                    self.colors.name_fg.0,
                    self.colors.name_fg.1,
                    self.colors.name_fg.2
                )
                .on_truecolor(
                    self.colors.name_bg.0,
                    self.colors.name_bg.1,
                    self.colors.name_bg.2
                ),
            format!(" v{}", self.version).truecolor(
                self.colors.version_color.0,
                self.colors.version_color.1,
                self.colors.version_color.2
            ),
            format!(" by {}", self.author).truecolor(
                self.colors.author_color.0,
                self.colors.author_color.1,
                self.colors.author_color.2
            )
        )
    }

    /// Checks if the version flag was used and handles it
    ///
    /// This method should be called after parsing command-line arguments.
    /// If the version flag is found, it prints the colorful version and exits.
    ///
    /// # Examples
    /// ```no_run
    /// use clap::Command;
    /// use clap_version_flag::{ColorfulVersion, ColorfulVersionExt};
    ///
    /// let version = ColorfulVersion::new("myapp", "1.0.0", "John Doe");
    /// let matches = Command::new("myapp")
    ///     .with_colorful_version(&version)
    ///     .get_matches();
    ///     
    /// version.check_and_exit(&matches);
    /// ```
    pub fn check_and_exit(&self, matches: &ArgMatches) {
        if matches.get_flag("clap_version_flag_version") {
            self.print_and_exit();
        }
    }

    /// Returns the package name
    ///
    /// # Examples
    /// ```
    /// use clap_version_flag::ColorfulVersion;
    ///
    /// let version = ColorfulVersion::new("myapp", "1.0.0", "John Doe");
    /// assert_eq!(version.package_name(), "myapp");
    /// ```
    #[must_use]
    pub fn package_name(&self) -> &str {
        &self.package_name
    }

    /// Returns the version
    ///
    /// # Examples
    /// ```
    /// use clap_version_flag::ColorfulVersion;
    ///
    /// let version = ColorfulVersion::new("myapp", "1.0.0", "John Doe");
    /// assert_eq!(version.version(), "1.0.0");
    /// ```
    #[must_use]
    pub fn version(&self) -> &str {
        &self.version
    }

    /// Returns the author
    ///
    /// # Examples
    /// ```
    /// use clap_version_flag::ColorfulVersion;
    ///
    /// let version = ColorfulVersion::new("myapp", "1.0.0", "John Doe");
    /// assert_eq!(version.author(), "John Doe");
    /// ```
    #[must_use]
    pub fn author(&self) -> &str {
        &self.author
    }
}

/// Implement Display trait for ColorfulVersion
impl fmt::Display for ColorfulVersion {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{} v{} by {}",
            self.package_name, self.version, self.author
        )
    }
}

/// Parses a hex color string to RGB values
///
/// # Arguments
/// * `hex` - Hex color string (e.g., "#FFFFFF", "#FFF", or "FFFFFF")
///
/// # Errors
/// Returns `VersionError::InvalidHexColor` if the hex string is invalid
///
/// # Examples
/// ```
/// # use clap_version_flag::ColorfulVersion;
/// // This is an internal function, example shown for completeness
/// let version = ColorfulVersion::new("app", "1.0", "author")
///     .with_hex_colors("#FFF", "#000", "#F00", "#0F0")
///     .unwrap();
/// ```
fn parse_hex(hex: &str) -> Result<(u8, u8, u8), VersionError> {
    let hex = hex.trim_start_matches('#');

    match hex.len() {
        6 => {
            let r =
                u8::from_str_radix(&hex[0..2], 16).map_err(|_| VersionError::invalid_hex(hex))?;
            let g =
                u8::from_str_radix(&hex[2..4], 16).map_err(|_| VersionError::invalid_hex(hex))?;
            let b =
                u8::from_str_radix(&hex[4..6], 16).map_err(|_| VersionError::invalid_hex(hex))?;
            Ok((r, g, b))
        }
        3 => {
            // Expand #RGB to #RRGGBB
            let r = u8::from_str_radix(&hex[0..1].repeat(2), 16)
                .map_err(|_| VersionError::invalid_hex(hex))?;
            let g = u8::from_str_radix(&hex[1..2].repeat(2), 16)
                .map_err(|_| VersionError::invalid_hex(hex))?;
            let b = u8::from_str_radix(&hex[2..3].repeat(2), 16)
                .map_err(|_| VersionError::invalid_hex(hex))?;
            Ok((r, g, b))
        }
        _ => Err(VersionError::invalid_hex(hex)),
    }
}

// NOTE: Main macros are now in src/macros.rs and re-exported
// This ensures they're available to users while keeping code organized

/// Extension trait for clap::Command to add colorful version flag
pub trait ColorfulVersionExt {
    /// Adds a version flag that will display colorful output when used
    ///
    /// # Examples
    /// ```
    /// use clap::Command;
    /// use clap_version_flag::{ColorfulVersion, ColorfulVersionExt};
    ///
    /// let version = ColorfulVersion::new("myapp", "1.0.0", "John Doe");
    /// let cmd = Command::new("myapp").with_colorful_version(&version);
    /// ```
    fn with_colorful_version(self, version: &ColorfulVersion) -> Self;
}

impl ColorfulVersionExt for Command {
    fn with_colorful_version(self, _version: &ColorfulVersion) -> Self {
        // Disable clap's built-in version flag and add our custom one
        self.disable_version_flag(true).arg(
            Arg::new("clap_version_flag_version")
                .short('V')
                .long("version")
                .action(ArgAction::SetTrue)
                .help("Print version information")
                .global(true),
        )
    }
}

/// Helper function to parse command-line arguments with version handling
///
/// This function should be used instead of directly calling `get_matches()`.
/// If the version flag is found, it prints the colorful version and exits.
///
/// # Examples
/// ```no_run
/// use clap::{Parser, CommandFactory};
/// use clap_version_flag::{colorful_version, parse_with_version};
///
/// #[derive(Parser)]
/// struct Cli {
///     input: String,
/// }
///
/// fn main() -> Result<(), Box<dyn std::error::Error>> {
///     let version = colorful_version!();
///     let cli: Cli = parse_with_version(Cli::command(), &version)?;
///     
///     // Normal program execution continues here
///     Ok(())
/// }
/// ```
pub fn parse_with_version<T: FromArgMatches>(
    command: Command,
    version: &ColorfulVersion,
) -> Result<T, clap::Error> {
    let command = command.with_colorful_version(version);
    let matches = command.get_matches();

    // Check if version flag was used
    if matches.get_flag("clap_version_flag_version") {
        version.print();
        process::exit(0);
    }

    T::from_arg_matches(&matches)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hex_parsing() {
        assert_eq!(parse_hex("#FFFFFF").unwrap(), (255, 255, 255));
        assert_eq!(parse_hex("#000000").unwrap(), (0, 0, 0));
        assert_eq!(parse_hex("#FF0000").unwrap(), (255, 0, 0));
        assert_eq!(parse_hex("#FFF").unwrap(), (255, 255, 255));
        assert_eq!(parse_hex("#F00").unwrap(), (255, 0, 0));
        assert_eq!(parse_hex("FFFFFF").unwrap(), (255, 255, 255));

        assert!(parse_hex("INVALID").is_err());
        assert!(parse_hex("#GGG").is_err());
        assert!(parse_hex("#12345").is_err());
    }

    #[test]
    fn test_default_colors() {
        let version = ColorfulVersion::new("test", "1.0.0", "author");
        assert_eq!(version.colors.name_fg, (255, 255, 255));
        assert_eq!(version.colors.name_bg, (170, 0, 255));
        assert_eq!(version.colors.version_color, (255, 255, 0));
        assert_eq!(version.colors.author_color, (0, 255, 255));
    }

    #[test]
    fn test_macro() {
        let version = colorful_version!();
        // In tests, this will use clap-version-flag's own package info
        assert_eq!(version.package_name(), env!("CARGO_PKG_NAME"));
        assert!(!version.package_name().is_empty());

        let custom = colorful_version!("#FFFFFF", "#AA00FF", "#FFFF00", "#00FFFF");
        assert_eq!(custom.colors.name_fg, (255, 255, 255));
        assert_eq!(custom.colors.name_bg, (170, 0, 255));
    }

    #[test]
    fn test_command_extension() {
        let version = colorful_version!();
        let cmd = Command::new("testapp").with_colorful_version(&version);

        // Should compile without errors
        assert_eq!(cmd.get_name(), "testapp");
    }

    #[test]
    fn test_version_string_format() {
        let version = ColorfulVersion::new("testapp", "1.2.3", "Test Author");
        let plain = version.to_string();

        // Verify format: "{name} v{version} by {author}"
        assert_eq!(plain, "testapp v1.2.3 by Test Author");
    }

    #[test]
    fn test_display_trait() {
        let version = ColorfulVersion::new("testapp", "1.2.3", "Test Author");
        let display = format!("{}", version);
        assert_eq!(display, "testapp v1.2.3 by Test Author");
    }

    #[test]
    fn test_as_plain_string() {
        let version = ColorfulVersion::new("myapp", "2.0.0", "John Doe");
        let plain = version.as_plain_string();
        assert_eq!(plain, "myapp v2.0.0 by John Doe");
    }

    #[test]
    fn test_getters() {
        let version = ColorfulVersion::new("myapp", "1.0.0", "John Doe");
        assert_eq!(version.package_name(), "myapp");
        assert_eq!(version.version(), "1.0.0");
        assert_eq!(version.author(), "John Doe");
    }

    #[test]
    fn test_custom_colors_rgb() {
        let version = ColorfulVersion::new("test", "1.0.0", "author").with_rgb_colors(
            (255, 0, 0),
            (0, 255, 0),
            (0, 0, 255),
            (255, 255, 0),
        );

        assert_eq!(version.colors.name_fg, (255, 0, 0));
        assert_eq!(version.colors.name_bg, (0, 255, 0));
        assert_eq!(version.colors.version_color, (0, 0, 255));
        assert_eq!(version.colors.author_color, (255, 255, 0));
    }

    #[test]
    fn test_custom_colors_hex() {
        let version = ColorfulVersion::new("test", "1.0.0", "author")
            .with_hex_colors("#FF0000", "#00FF00", "#0000FF", "#FFFF00")
            .unwrap();

        assert_eq!(version.colors.name_fg, (255, 0, 0));
        assert_eq!(version.colors.name_bg, (0, 255, 0));
        assert_eq!(version.colors.version_color, (0, 0, 255));
        assert_eq!(version.colors.author_color, (255, 255, 0));
    }

    #[test]
    fn test_short_hex() {
        let version = ColorfulVersion::new("test", "1.0.0", "author")
            .with_hex_colors("#F00", "#0F0", "#00F", "#FF0")
            .unwrap();

        assert_eq!(version.colors.name_fg, (255, 0, 0));
        assert_eq!(version.colors.name_bg, (0, 255, 0));
        assert_eq!(version.colors.version_color, (0, 0, 255));
        assert_eq!(version.colors.author_color, (255, 255, 0));
    }
}
