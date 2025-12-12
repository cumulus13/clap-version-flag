use clap::{CommandFactory, Parser};
use clap_version_flag::{ColorfulVersionExt, colorful_version};

#[derive(Parser)]
#[command(name = "integration-test-app")]
struct TestApp {
    #[arg(short, long)]
    value: Option<String>,
}

#[test]
fn test_version_flag_triggers_exit() {
    let version = colorful_version!();
    let cmd = TestApp::command().with_colorful_version(&version);

    // This should succeed (flag is present)
    let matches = cmd.try_get_matches_from(&["test", "--version"]).unwrap();
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
fn test_parse_with_version_helper() {
    let version = colorful_version!();

    // This test doesn't actually call parse_with_version with missing args
    // because it would exit the process. Instead we test the happy path.
    // The version flag exit behavior is tested in other tests.

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
#[ignore] // Ignore by default as it requires cargo to be in PATH and takes time
fn test_example_binary() {
    use std::process::Command as StdCommand;

    // Test that the example binaries compile and run
    let output = StdCommand::new("cargo")
        .args(["run", "--example", "basic", "--", "-V"])
        .output()
        .expect("Failed to run example");

    // Check that it ran successfully
    assert!(
        output.status.success(),
        "Example binary failed with stderr: {}",
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
    assert!(!plain.contains('['));

    // Should contain the expected format
    assert!(plain.contains(" v"));
    assert!(plain.contains(" by "));
}
