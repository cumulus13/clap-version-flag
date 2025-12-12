//! Basic example of using clap-version-flag
//!
//! Run with: cargo run --example basic -- --version

use clap::{CommandFactory, Parser};
use clap_version_flag::{colorful_version, parse_with_version};

#[derive(Parser, Debug)]
#[command(name = "myapp", about = "A sample application using colorful version")]
struct Cli {
    /// Input file to process
    // input: String,

    /// Enable verbose output
    #[arg(short, long)]
    verbose: bool,

    /// Number of times to process
    #[arg(short, long, default_value_t = 1)]
    count: u32,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create colorful version with default colors
    let version = colorful_version!();

    // Parse command-line arguments with version handling
    let cli: Cli = parse_with_version(Cli::command(), &version)?;

    // Normal program execution
    if cli.verbose {
        println!("Processing {} time(s)", cli.count);
    }

    for i in 0..cli.count {
        println!("Processing iteration {}...", i + 1);
        // Simulate work
        std::thread::sleep(std::time::Duration::from_millis(50));
    }

    println!("Done!");
    Ok(())
}
