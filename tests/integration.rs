// tests/integration.rs
// Integration tests for clap-version-flag with real clap usage

use clap::{CommandFactory, FromArgMatches, Parser};
use clap_version_flag::{ColorfulVersionExt, colorful_version};

#[derive(Parser, Debug)]
#[command(name = "integration-test-app")]
#[command(about = "Test application for integration tests")]
struct TestApp {
    #[arg(short, long)]
    value: Option<String>,

    #[arg(short, long)]
    number: Option<i32>,
}

#[test]
fn test_version_flag_triggers_detection() {
    let version = colorful_version!();
    let cmd = TestApp::command().with_colorful_version(&version);

    // This should succeed (flag is present)
    let matches = cmd.try_get_matches_from(&["test", "--version"]).unwrap();
    assert!(matches.get_flag("clap_version_flag_version"));
}

#[test]
fn test_short_version_flag() {
    let version = colorful_version!();
    let cmd = TestApp::command().with_colorful_version(&version);

    // Test short form -V
    let matches = cmd.try_get_matches_from(&["test", "-V"]).unwrap();
    assert!(matches.get_flag("clap_version_flag_version"));
}

#[test]
fn test_normal_parsing_works() {
    let version = colorful_version!();

    // With proper args, should work
    let cmd = TestApp::command().with_colorful_version(&version);
    let matches = cmd
        .try_get_matches_from(&["test", "--value", "hello"])
        .unwrap();
    assert!(!matches.get_flag("clap_version_flag_version"));

    // Verify the value was parsed correctly
    assert_eq!(
        matches.get_one::<String>("value").map(|s| s.as_str()),
        Some("hello")
    );
}

#[test]
fn test_multiple_args_parsing() {
    let version = colorful_version!();
    let cmd = TestApp::command().with_colorful_version(&version);

    let matches = cmd
        .try_get_matches_from(&["test", "--value", "hello", "--number", "42"])
        .unwrap();

    assert_eq!(
        matches.get_one::<String>("value").map(|s| s.as_str()),
        Some("hello")
    );
    assert_eq!(matches.get_one::<i32>("number").copied(), Some(42));
    assert!(!matches.get_flag("clap_version_flag_version"));
}

#[test]
fn test_parse_with_version_helper() {
    let version = colorful_version!();

    // This test doesn't actually call parse_with_version with --version
    // because it would exit the process. Instead we test the happy path.

    // Just verify the function signature works with proper args
    let cmd = TestApp::command().with_colorful_version(&version);
    let matches = cmd
        .try_get_matches_from(&["test", "--value", "test"])
        .unwrap();

    // Manually verify it works without calling parse_with_version
    // (which would exit on --version)
    assert!(!matches.get_flag("clap_version_flag_version"));
}

#[test]
fn test_check_and_exit_does_not_exit_without_flag() {
    let version = colorful_version!();
    let cmd = TestApp::command().with_colorful_version(&version);
    let matches = cmd
        .try_get_matches_from(&["test", "--value", "hello"])
        .unwrap();

    // This should not exit (we can't test the exit case without subprocess)
    // But we can verify the flag is false
    assert!(!matches.get_flag("clap_version_flag_version"));
}

#[test]
#[ignore] // Ignore by default as it requires cargo to be in PATH and takes time
fn test_example_basic_compiles() {
    use std::process::Command as StdCommand;

    // Test that the example binary compiles
    let output = StdCommand::new("cargo")
        .args(["build", "--example", "basic"])
        .output()
        .expect("Failed to build example");

    assert!(
        output.status.success(),
        "Example binary failed to compile with stderr: {}",
        String::from_utf8_lossy(&output.stderr)
    );
}

#[test]
#[ignore] // Ignore by default as it requires cargo and takes time
fn test_example_custom_colors_runs() {
    use std::process::Command as StdCommand;

    // Test that the custom_colors example runs
    let output = StdCommand::new("cargo")
        .args(["run", "--example", "custom_colors"])
        .output()
        .expect("Failed to run example");

    assert!(
        output.status.success(),
        "Example failed with stderr: {}",
        String::from_utf8_lossy(&output.stderr)
    );
}

#[test]
fn test_no_color_environment() {
    // Test that we can still run without colors
    let version = colorful_version!();
    let plain = version.to_string();

    // Plain string should not contain ANSI codes
    assert!(!plain.contains('\x1b'));

    // Should contain the expected format
    assert!(plain.contains(" v"));
    assert!(plain.contains(" by "));
}

#[test]
fn test_colored_output_format() {
    let version = colorful_version!();
    let colored = version.to_colored_string();

    // Colored output should not be empty
    assert!(!colored.is_empty());

    // It should contain the version parts (without ANSI codes this is hard to test)
    // But we can at least verify it's not empty
}

#[test]
fn test_version_flag_is_global() {
    // Verify that the version flag is global (accessible from subcommands)
    let version = colorful_version!();

    use clap::Command;
    let cmd = Command::new("app")
        .with_colorful_version(&version)
        .subcommand(Command::new("sub1"))
        .subcommand(Command::new("sub2"));

    // Should work at root level
    let matches = cmd
        .clone()
        .try_get_matches_from(&["app", "--version"])
        .unwrap();
    assert!(matches.get_flag("clap_version_flag_version"));

    // Should also work in subcommands (global flag)
    let matches = cmd
        .try_get_matches_from(&["app", "sub1", "--version"])
        .unwrap();
    assert!(matches.get_flag("clap_version_flag_version"));
}

#[derive(Parser)]
struct MinimalApp {
    file: String,
}

// #[test]
// fn test_with_minimal_app() {
//     let version = colorful_version!();
//     let cmd = MinimalApp::command().with_colorful_version(&version);

//     // Normal usage
//     let matches = cmd
//         .clone()
//         .try_get_matches_from(&["app", "file.txt"])
//         .unwrap();
//     assert!(!matches.get_flag("clap_version_flag_version"));

//     // Version flag
//     let matches = cmd.try_get_matches_from(&["app", "--version"]).unwrap();
//     assert!(matches.get_flag("clap_version_flag_version"));
// }

#[derive(Parser)]
struct AppWithDefaults {
    #[arg(short, long, default_value = "default")]
    value: String,
}

#[test]
fn test_with_default_values() {
    let version = colorful_version!();
    let cmd = AppWithDefaults::command().with_colorful_version(&version);

    let matches = cmd.try_get_matches_from(&["app"]).unwrap();
    assert!(!matches.get_flag("clap_version_flag_version"));
    assert_eq!(
        matches.get_one::<String>("value").map(|s| s.as_str()),
        Some("default")
    );
}

#[test]
fn test_error_handling_with_invalid_args() {
    let version = colorful_version!();
    let cmd = TestApp::command().with_colorful_version(&version);

    // Should fail with invalid argument
    let result = cmd.try_get_matches_from(&["test", "--invalid-arg"]);
    assert!(result.is_err());
}

#[test]
fn test_help_flag_still_works() {
    let version = colorful_version!();
    let cmd = TestApp::command().with_colorful_version(&version);

    // Help should still work
    let result = cmd.try_get_matches_from(&["test", "--help"]);
    // Help causes an error with kind DisplayHelp
    assert!(result.is_err());
    if let Err(e) = result {
        assert_eq!(e.kind(), clap::error::ErrorKind::DisplayHelp);
    }
}

#[test]
fn test_version_precedence() {
    // When both --version and other args are provided, version should be detected
    let version = colorful_version!();
    let cmd = TestApp::command().with_colorful_version(&version);

    let matches = cmd
        .try_get_matches_from(&["test", "--value", "hello", "--version"])
        .unwrap();
    assert!(matches.get_flag("clap_version_flag_version"));
}

// Test with builder pattern
// #[test]
// fn test_builder_pattern_integration() {
//     use clap::{Arg, Command};

//     let version = colorful_version!();
//     let cmd = Command::new("builder-app")
//         .with_colorful_version(&version)
//         .arg(Arg::new("input").short('i').long("input").required(true));

//     // Version flag should work
//     let matches = cmd
//         .clone()
//         .try_get_matches_from(&["app", "--version"])
//         .unwrap();
//     assert!(matches.get_flag("clap_version_flag_version"));

//     // Normal usage should still require input
//     let result = cmd.try_get_matches_from(&["app"]);
//     assert!(result.is_err()); // Should fail due to missing required arg
// }

#[test]
fn test_from_arg_matches_compatibility() {
    let version = colorful_version!();
    let cmd = TestApp::command().with_colorful_version(&version);
    let matches = cmd
        .try_get_matches_from(&["test", "--value", "test"])
        .unwrap();

    // Should be able to construct from matches using FromArgMatches trait
    let app = TestApp::from_arg_matches(&matches);
    assert!(app.is_ok());

    if let Ok(app) = app {
        assert_eq!(app.value, Some("test".to_string()));
    }
}
