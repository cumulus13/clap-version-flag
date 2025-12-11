//! Macro tambahan untuk kemudahan penggunaan

/// Macro untuk membuat colorful version dengan konfigurasi lengkap
#[macro_export]
macro_rules! colorful_version_full {
    ($name:expr, $version:expr, $author:expr) => {
        $crate::ColorfulVersion::new($name, $version, $author)
    };
    
    ($name:expr, $version:expr, $author:expr,
     $name_fg:expr, $name_bg:expr, $version_color:expr, $author_color:expr) => {
        $crate::ColorfulVersion::new($name, $version, $author)
            .with_hex_colors($name_fg, $name_bg, $version_color, $author_color)
            .expect("Invalid hex color format")
    };
}

/// Macro untuk derive helper (jika ingin membuat proc macro nanti)
#[macro_export]
macro_rules! derive_colorful_version {
    ($struct_name:ident) => {
        impl $struct_name {
            /// Get colorful version for this struct
            pub fn colorful_version() -> $crate::ColorfulVersion {
                $crate::colorful_version!()
            }
            
            /// Parse command with colorful version handling
            pub fn parse() -> Result<Self, clap::Error> {
                let version = Self::colorful_version();
                $crate::parse_with_version(Self::command(), &version)
            }
        }
    };
}