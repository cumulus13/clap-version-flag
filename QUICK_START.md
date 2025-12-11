use clap::{Parser, CommandFactory};
use clap_version_flag::{colorful_version, parse_with_version};

#[derive(Parser)]
struct Cli {
    input: String,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let version = colorful_version!();
    let cli: Cli = parse_with_version(Cli::command(), &version)?;
    
    println!("Processing: {}", cli.input);
    Ok(())
}