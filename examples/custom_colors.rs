//! Example with custom hex colors
//!
//! Run with: cargo run --example custom_colors -- --version

use clap::{CommandFactory, Parser};
use clap_version_flag::{colorful_version, parse_with_version};

#[derive(Parser, Debug)]
#[command(name = "rainbow-app", about = "An app with custom color scheme")]
struct Cli {
    /// Path to input file
    // #[arg(short, long)]
    // file: String,

    /// Output format
    #[arg(short, long, default_value = "json")]
    format: String,

    /// Dry run - don't actually process
    #[arg(long)]
    dry_run: bool,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create colorful version with custom hex colors
    let version = colorful_version!(
        "#FF5733", // Name foreground: Orange red
        "#581845", // Name background: Dark purple
        "#FFC300", // Version: Vivid yellow
        "#DAF7A6"  // Author: Light lime
    );

    // Parse command-line arguments
    let cli: Cli = parse_with_version(Cli::command(), &version)?;

    // Application logic
    println!("Application: Rainbow App");
    // println!("Input file: {}", cli.file);
    println!("Output format: {}", cli.format);
    println!("Dry run: {}", cli.dry_run);

    if !cli.dry_run {
        println!("Processing file...");
        // Actual processing would go here
    }

    Ok(())
}
