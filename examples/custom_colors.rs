// Example: Custom colors usage
// Run with: cargo run --example custom_colors

use clap_version_flag::{ColorfulVersion, colorful_version};

fn main() {
    println!("=== Default Colors ===");
    let default = colorful_version!();
    default.print();
    println!();

    println!("=== Custom Hex Colors ===");
    let custom_hex = colorful_version!(
        "#FF0000", // Red foreground for name
        "#000000", // Black background for name
        "#00FF00", // Green for version
        "#0000FF"  // Blue for author
    );
    custom_hex.print();
    println!();

    println!("=== Short Hex Format ===");
    let short_hex = colorful_version!(
        "#F00", // Red (expands to #FF0000)
        "#000", // Black
        "#0F0", // Green
        "#00F"  // Blue
    );
    short_hex.print();
    println!();

    println!("=== RGB Colors ===");
    let rgb = ColorfulVersion::new("myapp", "1.0.0", "John Doe").with_rgb_colors(
        (255, 165, 0), // Orange foreground
        (75, 0, 130),  // Indigo background
        (255, 215, 0), // Gold version
        (0, 255, 127), // Spring green author
    );
    rgb.print();
    println!();

    println!("=== High Contrast (Accessibility) ===");
    let high_contrast = ColorfulVersion::new("myapp", "1.0.0", "John Doe")
        .with_hex_colors("#FFFFFF", "#000000", "#FFFF00", "#00FF00")
        .unwrap();
    high_contrast.print();
    println!();

    println!("=== Dark Theme ===");
    let dark = ColorfulVersion::new("myapp", "1.0.0", "John Doe")
        .with_hex_colors("#E0E0E0", "#1A1A1A", "#FFA500", "#87CEEB")
        .unwrap();
    dark.print();
    println!();

    println!("=== Brand Colors Example (GitHub-like) ===");
    let github = ColorfulVersion::new("myapp", "1.0.0", "John Doe")
        .with_hex_colors("#FFFFFF", "#24292E", "#0366D6", "#6F42C1")
        .unwrap();
    github.print();
}
