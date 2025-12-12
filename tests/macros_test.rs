// tests/macros_test.rs
// Tests for macros in clap-version-flag

use clap::Parser;
use clap_version_flag::{colorful_version, colorful_version_full, colorful_version_rgb};

#[test]
fn test_colorful_version_macro() {
    // Test basic macro
    let version = colorful_version!();
    assert!(!version.package_name().is_empty());
    assert!(!version.version().is_empty());
    assert_eq!(version.package_name(), env!("CARGO_PKG_NAME"));
}

#[test]
fn test_colorful_version_macro_with_colors() {
    // Test macro with custom colors
    let version = colorful_version!("#FFFFFF", "#000000", "#FF0000", "#00FF00");
    assert_eq!(version.package_name(), env!("CARGO_PKG_NAME"));
}

#[test]
fn test_colorful_version_full_without_colors() {
    // Test full macro without colors
    let version = colorful_version_full!("myapp", "1.0.0", "John Doe");
    assert_eq!(version.package_name(), "myapp");
    assert_eq!(version.version(), "1.0.0");
    assert_eq!(version.author(), "John Doe");
}

#[test]
fn test_colorful_version_full_with_colors() {
    // Test full macro with colors
    let version = colorful_version_full!(
        "myapp", "1.0.0", "John Doe", "#FFFFFF", "#AA00FF", "#FFFF00", "#00FFFF"
    );
    assert_eq!(version.package_name(), "myapp");
    assert_eq!(version.version(), "1.0.0");
    assert_eq!(version.author(), "John Doe");
}

#[test]
fn test_colorful_version_rgb_macro() {
    // Test RGB color macro
    let version = colorful_version_rgb!(
        (255, 255, 255), // White foreground
        (0, 0, 0),       // Black background
        (255, 0, 0),     // Red version
        (0, 255, 0)      // Green author
    );
    assert_eq!(version.package_name(), env!("CARGO_PKG_NAME"));
}

#[test]
fn test_colorful_version_from_different_context() {
    // Test that macro expansion works correctly
    // The macro should get THIS crate's package info when used in tests
    #[derive(Parser)]
    struct TestCli {
        #[arg(short, long)]
        name: String,
    }

    // Use the macro
    let version = colorful_version!();

    // In tests, this will be the crate being tested
    assert_eq!(version.package_name(), env!("CARGO_PKG_NAME"));
    assert!(!version.package_name().is_empty());
}

#[test]
fn test_multiple_macros_together() {
    // Test that different macros can be used together
    let v1 = colorful_version!();
    let v2 = colorful_version_full!("app", "1.0", "author");
    let v3 = colorful_version_rgb!((255, 0, 0), (0, 255, 0), (0, 0, 255), (255, 255, 0));

    assert!(!v1.package_name().is_empty());
    assert_eq!(v2.package_name(), "app");
    assert!(!v3.package_name().is_empty());
}

#[test]
fn test_macro_with_short_hex() {
    // Test macro with short hex codes
    let version = colorful_version!("#FFF", "#000", "#F00", "#0F0");
    assert_eq!(version.package_name(), env!("CARGO_PKG_NAME"));
}

#[test]
fn test_macro_with_hex_without_hash() {
    // Test macro with hex codes without #
    let version = colorful_version!("FFFFFF", "000000", "FF0000", "00FF00");
    assert_eq!(version.package_name(), env!("CARGO_PKG_NAME"));
}

#[test]
#[should_panic(expected = "Invalid hex color format")]
fn test_macro_with_invalid_hex_panics() {
    // This should panic with invalid hex
    let _version = colorful_version!("INVALID", "#000000", "#000000", "#000000");
}

#[test]
fn test_colorful_version_full_string_types() {
    // Test that different string types work
    let version1 = colorful_version_full!("myapp", "1.0.0", "John");
    let version2 = colorful_version_full!(
        String::from("myapp"),
        String::from("1.0.0"),
        String::from("John")
    );

    assert_eq!(version1.package_name(), version2.package_name());
    assert_eq!(version1.version(), version2.version());
    assert_eq!(version1.author(), version2.author());
}

#[test]
fn test_macro_with_complex_struct() {
    // Test with more complex CLI structure
    #[derive(Parser)]
    struct ComplexCli {
        #[arg(short, long)]
        input: String,

        #[arg(short, long)]
        output: Option<String>,

        #[arg(short, long, default_value = "false")]
        verbose: bool,
    }

    // Use the macro
    let version = colorful_version!();
    assert_eq!(version.package_name(), env!("CARGO_PKG_NAME"));
    assert!(!version.package_name().is_empty());
}

#[test]
fn test_rgb_macro_with_edge_values() {
    // Test RGB with min and max values
    let version = colorful_version_rgb!(
        (0, 0, 0),       // Black
        (255, 255, 255), // White
        (128, 128, 128), // Gray
        (255, 0, 255)    // Magenta
    );
    assert!(!version.package_name().is_empty());
}

#[test]
fn test_macro_expansion_uses_caller_env() {
    // This test verifies that env! expands at caller's location
    let version = colorful_version!();

    // In this test file, CARGO_PKG_NAME should be "clap-version-flag"
    // because we're testing the library itself
    assert_eq!(version.package_name(), env!("CARGO_PKG_NAME"));
    assert_eq!(version.version(), env!("CARGO_PKG_VERSION"));

    // The key point is that when used in OTHER crates, it will get THEIR info
}

#[test]
fn test_colorful_version_full_with_special_characters() {
    let version =
        colorful_version_full!("my-app_v2", "1.0.0-beta.1", "John Doe <john@example.com>");

    assert_eq!(version.package_name(), "my-app_v2");
    assert_eq!(version.version(), "1.0.0-beta.1");
    assert_eq!(version.author(), "John Doe <john@example.com>");
}

#[test]
fn test_colorful_version_full_with_unicode() {
    let version = colorful_version_full!("我的应用", "1.0.0", "作者姓名");

    assert_eq!(version.package_name(), "我的应用");
    assert_eq!(version.author(), "作者姓名");
}

#[test]
fn test_all_macros_produce_valid_output() {
    let v1 = colorful_version!();
    let v2 = colorful_version_full!("app", "1.0", "author");
    let v3 = colorful_version_rgb!((255, 0, 0), (0, 255, 0), (0, 0, 255), (255, 255, 0));

    // All should produce valid string output
    assert!(!v1.to_string().is_empty());
    assert!(!v2.to_string().is_empty());
    assert!(!v3.to_string().is_empty());

    // All should have proper format
    assert!(v1.to_string().contains(" v"));
    assert!(v2.to_string().contains(" v"));
    assert!(v3.to_string().contains(" v"));

    assert!(v1.to_string().contains(" by "));
    assert!(v2.to_string().contains(" by "));
    assert!(v3.to_string().contains(" by "));
}
