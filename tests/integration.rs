use clap::{Parser, CommandFactory};
use clap_version_flag::{colorful_version, ColorfulVersionExt, parse_with_version};
use std::process::Command as StdCommand;

#[derive(Parser)]
#[command(name = "integration-test-app")]
struct TestApp {
    #[arg(short, long)]
    value: Option<String>,
}

#[test]
fn test_version_flag_triggers_exit() {
    let version = colorful_version!();
    let cmd = TestApp::command()
        .with_colorful_version(&version);
    
    // This should succeed (flag is present)
    let matches = cmd.try_get_matches_from(&["test", "--version"]).unwrap();
    assert!(matches.get_flag("clap_version_flag_version"));
}

#[test]
fn test_normal_parsing_works() {
    let version = colorful_version!();
    
    // Test normal parsing without version flag
    let result = parse_with_version::<TestApp>(
        TestApp::command(),
        &version
    );
    
    // Without --version flag, it should fail because required arg is missing
    assert!(result.is_err());
    
    // With proper args, should work
    let cmd = TestApp::command().with_colorful_version(&version);
    let matches = cmd.try_get_matches_from(&["test", "--value", "hello"]).unwrap();
    assert!(!matches.get_flag("clap_version_flag_version"));
}

#[test]
fn test_example_binary() {
    // Test that the example binaries compile and run
    let output = StdCommand::new("cargo")
        .args(["run", "--example", "basic", "--", "--version"])
        .output()
        .expect("Failed to run example");
    
    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains(env!("CARGO_PKG_NAME")));
}

#[test]
fn test_no_color_environment() {
    // Test that we can still run without colors
    let version = colorful_version!();
    let plain = version.to_string();
    
    // Plain string should not contain ANSI codes
    assert!(!plain.contains('\x1b'));
    assert!(!plain.contains("["));
    
    // Should contain the expected format
    assert!(plain.contains("v"));
    assert!(plain.contains("by"));
}