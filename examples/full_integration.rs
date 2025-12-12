// Example: Full integration with clap
// Run with: cargo run --example full_integration -- --version
// Run with: cargo run --example full_integration -- --name "John"

use clap::{CommandFactory, Parser};
use clap_version_flag::{colorful_version, parse_with_version};

#[derive(Parser, Debug)]
#[command(name = "full-integration")]
#[command(about = "Full integration example with automatic version handling")]
struct Cli {
    /// Your name
    #[arg(short, long)]
    name: String,

    /// Age
    #[arg(short, long)]
    age: Option<u32>,

    /// Enable debug mode
    #[arg(short, long)]
    debug: bool,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create colorful version
    let version = colorful_version!();

    // Parse with automatic version handling
    // If user passes --version, it will print colorful version and exit
    let cli: Cli = parse_with_version(Cli::command(), &version)?;

    // If we reach here, version flag was not used
    // Continue with normal application logic

    if cli.debug {
        println!("Debug mode enabled");
        println!("Parsed CLI: {:#?}", cli);
    }

    println!("Hello, {}!", cli.name);

    if let Some(age) = cli.age {
        println!("You are {} years old.", age);
    }

    Ok(())
}
