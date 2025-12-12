// tests/color_test.rs
// Color and formatting tests for clap-version-flag

use clap::Command;
use clap_version_flag::{ColorfulVersion, ColorfulVersionExt, VersionError, colorful_version};

#[test]
fn test_default_colors_correct() {
    // Test that default ColorfulVersion can be created using macro
    let version = colorful_version!();

    // Test with custom colors to verify parsing works
    let result = version.with_hex_colors("#FFFFFF", "#AA00FF", "#FFFF00", "#00FFFF");
    assert!(result.is_ok(), "Should accept default color values");
}

#[test]
fn test_hex_color_validation() {
    // Valid cases
    let valid_cases = ["#FFFFFF", "#ffffff", "#FFF", "#fff", "FFFFFF", "FFF"];

    for hex in valid_cases {
        let result = ColorfulVersion::new("test", "1.0.0", "author")
            .with_hex_colors(hex, "#000", "#000", "#000");
        assert!(result.is_ok(), "Should accept: {}", hex);
    }

    // Invalid cases
    let invalid_cases = ["NOT_A_COLOR", "#GGG", "#12345", "#1234567", ""];

    for hex in invalid_cases {
        let result = ColorfulVersion::new("test", "1.0.0", "author")
            .with_hex_colors(hex, "#000", "#000", "#000");
        assert!(result.is_err(), "Should reject: {}", hex);
    }
}

#[test]
fn test_rgb_color_setting() {
    let version = ColorfulVersion::new("test", "1.0.0", "author").with_rgb_colors(
        (255, 0, 0),   // Red name foreground
        (0, 255, 0),   // Green name background
        (0, 0, 255),   // Blue version
        (255, 255, 0), // Yellow author
    );

    // Verify the version object was created successfully
    assert_eq!(version.package_name(), "test");
    assert_eq!(version.version(), "1.0.0");
    assert_eq!(version.author(), "author");
}

#[test]
fn test_version_string_format() {
    let version = ColorfulVersion::new("testapp", "1.2.3", "Test Author");
    let plain = version.to_string();

    // Verify format: "{name} v{version} by {author}"
    assert_eq!(plain, "testapp v1.2.3 by Test Author");
    assert!(
        !plain.contains('\x1b'),
        "Plain string should not contain ANSI codes"
    );
}

#[test]
fn test_as_plain_string() {
    let version = ColorfulVersion::new("myapp", "2.0.0", "John Doe");
    let plain = version.as_plain_string();

    assert_eq!(plain, "myapp v2.0.0 by John Doe");
    assert!(!plain.contains('\x1b'), "Should not contain ANSI codes");
}

#[test]
fn test_command_extension() {
    // Use colorful_version!() macro which correctly gets caller's package info
    let version = colorful_version!();
    let cmd = Command::new("testapp").with_colorful_version(&version);

    // Command should have our version flag
    let result = cmd.try_get_matches_from(&["testapp", "--version"]);
    assert!(result.is_ok());

    if let Ok(matches) = result {
        assert!(matches.get_flag("clap_version_flag_version"));
    }
}

#[test]
fn test_command_extension_disables_default_version() {
    let version = colorful_version!();
    let cmd = Command::new("testapp").with_colorful_version(&version);

    // The extension should disable clap's built-in version flag
    let result = cmd.try_get_matches_from(&["testapp", "-V"]);
    assert!(result.is_ok(), "Our custom -V flag should work");
}

#[test]
fn test_macro_with_default_colors() {
    // Test macro with default colors
    let version = colorful_version!();

    // Verify it was created successfully
    assert!(!version.package_name().is_empty());
    assert!(!version.version().is_empty());
    // In test context, this will be "clap-version-flag"
    assert_eq!(version.package_name(), env!("CARGO_PKG_NAME"));
}

#[test]
fn test_macro_with_custom_colors() {
    // Test macro with custom colors
    let version = colorful_version!("#FFFFFF", "#AA00FF", "#FFFF00", "#00FFFF");

    // Verify it was created successfully
    assert!(!version.package_name().is_empty());
    assert!(!version.version().is_empty());
    assert_eq!(version.package_name(), env!("CARGO_PKG_NAME"));
}

#[test]
fn test_error_messages() {
    let err = VersionError::invalid_hex("#INVALID");
    let msg = err.to_string();
    assert!(msg.contains("Invalid hex color format"));
    assert!(msg.contains("#INVALID"));
    assert!(msg.contains("Expected format"));
}

#[test]
fn test_colored_string_contains_ansi() {
    let version = ColorfulVersion::new("testapp", "1.0.0", "Author");
    let colored = version.to_colored_string();

    // Colored string should contain ANSI codes
    let plain = version.to_string();
    assert!(!colored.is_empty());
    assert!(!plain.is_empty());

    // The colored version should be longer due to ANSI codes (unless colors are disabled)
    // We don't assert this strictly as it depends on terminal support
    assert!(colored.len() >= plain.len());
}

#[test]
fn test_getters() {
    let version = ColorfulVersion::new("myapp", "2.0.0", "John Doe");

    assert_eq!(version.package_name(), "myapp");
    assert_eq!(version.version(), "2.0.0");
    assert_eq!(version.author(), "John Doe");
}

#[test]
fn test_cargo_env_output_format() {
    // Use the macro which correctly expands env! at test location
    let version = colorful_version!();
    let output = version.to_string();

    // Verify format matches: "{CARGO_PKG_NAME} v{CARGO_PKG_VERSION} by {CARGO_PKG_AUTHORS}"
    let expected = format!(
        "{} v{} by {}",
        env!("CARGO_PKG_NAME"),
        env!("CARGO_PKG_VERSION"),
        env!("CARGO_PKG_AUTHORS")
    );

    assert_eq!(output, expected);
}

#[test]
fn test_short_hex_expansion() {
    // Test that 3-digit hex codes are expanded correctly
    let version = ColorfulVersion::new("test", "1.0.0", "author")
        .with_hex_colors("#FFF", "#000", "#F00", "#0F0")
        .unwrap();

    // Just verify it was created without error
    assert_eq!(version.package_name(), "test");
}

#[test]
fn test_hex_without_hash() {
    // Test that hex codes without # are accepted
    let version = ColorfulVersion::new("test", "1.0.0", "author")
        .with_hex_colors("FFFFFF", "000000", "FF0000", "00FF00")
        .unwrap();

    assert_eq!(version.package_name(), "test");
}

#[test]
fn test_mixed_case_hex() {
    // Test that mixed case hex codes work
    let version = ColorfulVersion::new("test", "1.0.0", "author")
        .with_hex_colors("#FfFfFf", "#AaBbCc", "#123456", "#abcdef")
        .unwrap();

    assert_eq!(version.package_name(), "test");
}

#[test]
fn test_display_trait() {
    let version = ColorfulVersion::new("testapp", "1.2.3", "Test Author");
    let display = format!("{}", version);
    assert_eq!(display, "testapp v1.2.3 by Test Author");
}

#[test]
fn test_clone() {
    let version1 = ColorfulVersion::new("test", "1.0.0", "author");
    let version2 = version1.clone();

    assert_eq!(version1.package_name(), version2.package_name());
    assert_eq!(version1.version(), version2.version());
    assert_eq!(version1.author(), version2.author());
}

#[test]
fn test_debug() {
    let version = ColorfulVersion::new("test", "1.0.0", "author");
    let debug = format!("{:?}", version);

    // Debug output should contain the struct name
    assert!(debug.contains("ColorfulVersion"));
}

#[test]
fn test_invalid_hex_lengths() {
    // Too short
    assert!(
        ColorfulVersion::new("t", "1", "a")
            .with_hex_colors("#FF", "#000000", "#000000", "#000000")
            .is_err()
    );

    // Too long
    assert!(
        ColorfulVersion::new("t", "1", "a")
            .with_hex_colors("#FFFFFFF", "#000000", "#000000", "#000000")
            .is_err()
    );

    // Invalid length (4 digits)
    assert!(
        ColorfulVersion::new("t", "1", "a")
            .with_hex_colors("#FFFF", "#000000", "#000000", "#000000")
            .is_err()
    );
}

#[test]
fn test_invalid_hex_characters() {
    // Contains non-hex characters
    assert!(
        ColorfulVersion::new("t", "1", "a")
            .with_hex_colors("#GGGGGG", "#000000", "#000000", "#000000")
            .is_err()
    );

    assert!(
        ColorfulVersion::new("t", "1", "a")
            .with_hex_colors("#ZZZZZZ", "#000000", "#000000", "#000000")
            .is_err()
    );
}

#[test]
fn test_empty_values() {
    // Empty strings should work (though not recommended in practice)
    let version = ColorfulVersion::new("", "", "");
    assert_eq!(version.package_name(), "");
    assert_eq!(version.version(), "");
    assert_eq!(version.author(), "");
}

#[test]
fn test_unicode_in_values() {
    // Unicode characters should be handled correctly
    let version = ColorfulVersion::new("测试应用", "1.0.0", "作者名字");
    assert_eq!(version.package_name(), "测试应用");
    assert_eq!(version.author(), "作者名字");
}

#[test]
fn test_special_characters_in_values() {
    let version = ColorfulVersion::new("my-app_v2", "1.0.0-beta.1", "John Doe <john@example.com>");
    assert_eq!(version.package_name(), "my-app_v2");
    assert_eq!(version.version(), "1.0.0-beta.1");
    assert_eq!(version.author(), "John Doe <john@example.com>");
}
