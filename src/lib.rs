//! A production-ready crate for adding colorful version output to clap applications.
//! 
//! This crate provides utilities to override the default `-V`/`--version` flag behavior
//! in clap applications with colorful output using hex color codes.

mod error;

pub use error::VersionError;

use colored::Colorize;
use clap::{Arg, ArgAction, ArgMatches, Command, FromArgMatches};
use std::process;

/// Configuration for colorful version output
#[derive(Clone, Debug)]
pub struct ColorfulVersion {
    package_name: &'static str,
    version: &'static str,
    author: &'static str,
    colors: Colors,
}

#[derive(Clone, Debug)]
struct Colors {
    name_fg: (u8, u8, u8),      // RGB for name foreground
    name_bg: (u8, u8, u8),      // RGB for name background
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
            name_fg: (255, 255, 255),   // #FFFFFF
            name_bg: (170, 0, 255),     // #AA00FF
            version_color: (255, 255, 0), // #FFFF00
            author_color: (0, 255, 255),  // #00FFFF
        }
    }
}

impl ColorfulVersion {
    /// Creates a new ColorfulVersion using values from Cargo.toml
    #[must_use]
    pub fn from_cargo() -> Self {
        Self {
            package_name: env!("CARGO_PKG_NAME"),
            version: env!("CARGO_PKG_VERSION"),
            author: env!("CARGO_PKG_AUTHORS"),
            colors: Colors::default(),
        }
    }
    
    /// Creates a new ColorfulVersion with custom values
    #[must_use]
    pub fn new(
        package_name: &'static str,
        version: &'static str,
        author: &'static str,
    ) -> Self {
        Self {
            package_name,
            version,
            author,
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
    pub fn print_and_exit(&self) -> ! {
        self.print();
        process::exit(0);
    }
    
    /// Prints the colorful version to stdout
    /// Format: "{package_name} v{version} by {author}"
    pub fn print(&self) {
        let name = self.package_name
            .truecolor(self.colors.name_fg.0, self.colors.name_fg.1, self.colors.name_fg.2)
            .on_truecolor(self.colors.name_bg.0, self.colors.name_bg.1, self.colors.name_bg.2);
        
        let version_text = format!(" v{}", self.version)
            .truecolor(self.colors.version_color.0, self.colors.version_color.1, self.colors.version_color.2);
        
        let author_text = format!(" by {}", self.author)
            .truecolor(self.colors.author_color.0, self.colors.author_color.1, self.colors.author_color.2);
        
        println!("{name}{version_text}{author_text}");
    }
    
    /// Returns a plain text version string (for clap's version flag)
    /// Format: "{package_name} v{version} by {author}"
    #[must_use]
    pub fn to_string(&self) -> String {
        format!("{} v{} by {}", self.package_name, self.version, self.author)
    }
    
    /// Returns a colored version string if terminal supports colors
    /// Format: "{package_name} v{version} by {author}"
    #[must_use]
    pub fn to_colored_string(&self) -> String {
        format!("{}{}{}",
            self.package_name
                .truecolor(self.colors.name_fg.0, self.colors.name_fg.1, self.colors.name_fg.2)
                .on_truecolor(self.colors.name_bg.0, self.colors.name_bg.1, self.colors.name_bg.2),
            format!(" v{}", self.version)
                .truecolor(self.colors.version_color.0, self.colors.version_color.1, self.colors.version_color.2),
            format!(" by {}", self.author)
                .truecolor(self.colors.author_color.0, self.colors.author_color.1, self.colors.author_color.2)
        )
    }
    
    /// Checks if the version flag was used and handles it
    ///
    /// This method should be called after parsing command-line arguments.
    /// If the version flag is found, it prints the colorful version and exits.
    pub fn check_and_exit(&self, matches: &ArgMatches) {
        if matches.get_flag("clap_version_flag_version") {
            self.print_and_exit();
        }
    }
    
    /// Returns the package name
    #[must_use]
    pub fn package_name(&self) -> &str {
        self.package_name
    }
    
    /// Returns the version
    #[must_use]
    pub fn version(&self) -> &str {
        self.version
    }
    
    /// Returns the author
    #[must_use]
    pub fn author(&self) -> &str {
        self.author
    }
}

/// Parses a hex color string to RGB values
///
/// # Arguments
/// * `hex` - Hex color string (e.g., "#FFFFFF", "#FFF", or "FFFFFF")
///
/// # Errors
/// Returns `VersionError::InvalidHexColor` if the hex string is invalid
fn parse_hex(hex: &str) -> Result<(u8, u8, u8), VersionError> {
    let hex = hex.trim_start_matches('#');
    
    match hex.len() {
        6 => {
            let r = u8::from_str_radix(&hex[0..2], 16)
                .map_err(|_| VersionError::invalid_hex(hex))?;
            let g = u8::from_str_radix(&hex[2..4], 16)
                .map_err(|_| VersionError::invalid_hex(hex))?;
            let b = u8::from_str_radix(&hex[4..6], 16)
                .map_err(|_| VersionError::invalid_hex(hex))?;
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

/// Macro for easy creation of ColorfulVersion from Cargo.toml
///
/// # Examples
/// ```
/// use clap_version_flag::colorful_version;
///
/// // With default colors
/// let version = colorful_version!();
///
/// // With custom hex colors
/// let version = colorful_version!("#FFFFFF", "#AA00FF", "#FFFF00", "#00FFFF");
/// ```
#[macro_export]
macro_rules! colorful_version {
    () => {
        $crate::ColorfulVersion::from_cargo()
    };
    
    ($name_fg:expr, $name_bg:expr, $version:expr, $author:expr) => {
        $crate::ColorfulVersion::from_cargo()
            .with_hex_colors($name_fg, $name_bg, $version, $author)
            .unwrap_or_else(|e| {
                panic!("clap-version-flag: Invalid hex color format: {}", e)
            })
    };
}

/// Extension trait for clap::Command to add colorful version flag
pub trait ColorfulVersionExt {
    /// Adds a version flag that will display colorful output when used
    fn with_colorful_version(self, version: &ColorfulVersion) -> Self;
}

impl ColorfulVersionExt for Command {
    fn with_colorful_version(self, _version: &ColorfulVersion) -> Self {
        // Just add the flag - actual handling is done in parse_with_version
        self.arg(
            Arg::new("clap_version_flag_version")
                .short('V')
                .long("version")
                .action(ArgAction::SetTrue)
                .help("Print version information")
                .global(true)
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
        let version = ColorfulVersion::from_cargo();
        assert_eq!(version.colors.name_fg, (255, 255, 255));
        assert_eq!(version.colors.name_bg, (170, 0, 255));
        assert_eq!(version.colors.version_color, (255, 255, 0));
        assert_eq!(version.colors.author_color, (0, 255, 255));
    }
    
    #[test]
    fn test_macro() {
        let version = colorful_version!();
        assert!(!version.package_name().is_empty());
        
        let custom = colorful_version!("#FFFFFF", "#AA00FF", "#FFFF00", "#00FFFF");
        assert_eq!(custom.colors.name_fg, (255, 255, 255));
    }
    
    #[test]
    fn test_command_extension() {
        let version = colorful_version!();
        let cmd = Command::new("testapp")
            .with_colorful_version(&version);
        
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
    fn test_cargo_env_format() {
        let version = ColorfulVersion::from_cargo();
        let plain = version.to_string();
        
        // Should follow format: "{CARGO_PKG_NAME} v{CARGO_PKG_VERSION} by {CARGO_PKG_AUTHORS}"
        assert!(plain.contains(" v"));
        assert!(plain.contains(" by "));
        assert_eq!(plain, format!("{} v{} by {}", 
            env!("CARGO_PKG_NAME"), 
            env!("CARGO_PKG_VERSION"), 
            env!("CARGO_PKG_AUTHORS")
        ));
    }
}