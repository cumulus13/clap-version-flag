// Example: Basic usage of clap-version-flag
// Run with: cargo run --example basic -- --version

use clap::Parser;
use clap_version_flag::colorful_version;

#[derive(Parser)]
#[command(name = "basic-example")]
#[command(about = "A basic example of clap-version-flag", long_about = None)]
struct Cli {
    /// Your name
    #[arg(short, long)]
    name: String,

    /// Enable verbose output
    #[arg(short, long)]
    verbose: bool,
}

fn main() {
    // Create colorful version from Cargo.toml
    let version = colorful_version!();

    // Print the version (for demonstration)
    println!("=== Colorful Version ===");
    version.print();
    println!();

    // Parse command line arguments
    let cli = Cli::parse();

    // Your application logic
    if cli.verbose {
        println!("Verbose mode enabled!");
    }

    println!("Hello, {}!", cli.name);
}
