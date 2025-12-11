//! Example using the extension trait
//!
//! Run with: cargo run --example derive_helper -- --version

use clap::{Parser, CommandFactory, FromArgMatches};
use clap_version_flag::{colorful_version, ColorfulVersionExt};

#[derive(Parser)]
#[command(name = "my-tool")]
struct Cli {
    /// Target to process
    target: String,
    
    /// Force operation
    #[arg(short, long)]
    force: bool,
    
    /// Silent mode
    #[arg(short, long)]
    silent: bool,
}

fn main() {
    // Create colorful version
    let version = colorful_version!();
    
    // Build command with version flag
    let cmd = Cli::command()
        .with_colorful_version(&version);
    
    // Get matches
    let matches = cmd.get_matches();
    
    // Check for version flag
    if matches.get_flag("clap_version_flag_version") {
        version.print_and_exit();
    }
    
    // Parse normally using FromArgMatches trait
    let cli = Cli::from_arg_matches(&matches)
        .unwrap_or_else(|e| e.exit());
    
    // Application logic
    if !cli.silent {
        println!("Processing target: {}", cli.target);
        println!("Force mode: {}", cli.force);
    }
    
    // Simulate work
    if cli.force {
        println!("Forcefully processing...");
    } else {
        println!("Processing normally...");
    }
}