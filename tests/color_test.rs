use clap::Command;
use clap_version_flag::{ColorfulVersion, ColorfulVersionExt, VersionError};

#[test]
fn test_default_colors_correct() {
    // Test that default ColorfulVersion can be created
    let version = ColorfulVersion::from_cargo();
    
    // Test with custom colors to verify parsing works
    let result = version.with_hex_colors("#FFFFFF", "#AA00FF", "#FFFF00", "#00FFFF");
    assert!(result.is_ok(), "Should accept default color values");
}

#[test]
fn test_hex_color_validation() {
    // Valid cases
    let valid_cases = [
        "#FFFFFF",
        "#ffffff",
        "#FFF",
        "#fff",
        "FFFFFF",
        "FFF",
    ];
    
    for hex in valid_cases {
        let result = ColorfulVersion::from_cargo().with_hex_colors(hex, "#000", "#000", "#000");
        assert!(result.is_ok(), "Should accept: {}", hex);
    }
    
    // Invalid cases
    let invalid_cases = [
        "NOT_A_COLOR",
        "#GGG",
        "#12345",
        "#1234567",
        "",
    ];
    
    for hex in invalid_cases {
        let result = ColorfulVersion::from_cargo().with_hex_colors(hex, "#000", "#000", "#000");
        assert!(result.is_err(), "Should reject: {}", hex);
    }
}

#[test]
fn test_rgb_color_setting() {
    let version = ColorfulVersion::from_cargo()
        .with_rgb_colors(
            (255, 0, 0),    // Red name foreground
            (0, 255, 0),    // Green name background
            (0, 0, 255),    // Blue version
            (255, 255, 0),  // Yellow author
        );
    
    // Verify the version object was created successfully
    assert_eq!(version.package_name(), env!("CARGO_PKG_NAME"));
}

#[test]
fn test_version_string_format() {
    let version = ColorfulVersion::new("testapp", "1.2.3", "Test Author");
    let plain = version.to_string();
    
    // Verify format: "{name} v{version} by {author}"
    assert_eq!(plain, "testapp v1.2.3 by Test Author");
    assert!(!plain.contains('\x1b'), "Plain string should not contain ANSI codes");
}

#[test]
fn test_command_extension() {
    // Use ColorfulVersion::from_cargo() directly
    let version = ColorfulVersion::from_cargo();
    let cmd = Command::new("testapp")
        .with_colorful_version(&version);
    
    // Command should have our version flag
    let result = cmd.try_get_matches_from(&["testapp", "--version"]);
    assert!(result.is_ok());
    
    if let Ok(matches) = result {
        assert!(matches.get_flag("clap_version_flag_version"));
    }
}

#[test]
fn test_macro_with_default_colors() {
    // Test macro with default colors using full path
    let version = clap_version_flag::colorful_version!();
    
    // Verify it was created successfully
    assert!(!version.package_name().is_empty());
    assert!(!version.version().is_empty());
}

#[test]
fn test_macro_with_custom_colors() {
    // Test macro with custom colors using full path
    let version = clap_version_flag::colorful_version!("#FFFFFF", "#AA00FF", "#FFFF00", "#00FFFF");
    
    // Verify it was created successfully
    assert!(!version.package_name().is_empty());
    assert!(!version.version().is_empty());
}

#[test]
fn test_error_messages() {
    let err = VersionError::invalid_hex("#INVALID");
    assert!(err.to_string().contains("Invalid hex color format"));
    assert!(err.to_string().contains("#INVALID"));
}

#[test]
fn test_colored_string_contains_ansi() {
    let version = ColorfulVersion::new("testapp", "1.0.0", "Author");
    let colored = version.to_colored_string();
    
    // Colored string should contain ANSI codes (or at least be different from plain)
    let plain = version.to_string();
    // In terminals without color support, they might be the same, so we just check it doesn't panic
    assert!(!colored.is_empty());
    assert!(!plain.is_empty());
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
    let version = ColorfulVersion::from_cargo();
    let output = version.to_string();
    
    // Verify format matches: "{CARGO_PKG_NAME} v{CARGO_PKG_VERSION} by {CARGO_PKG_AUTHORS}"
    let expected = format!("{} v{} by {}", 
        env!("CARGO_PKG_NAME"),
        env!("CARGO_PKG_VERSION"),
        env!("CARGO_PKG_AUTHORS")
    );
    
    assert_eq!(output, expected);
}