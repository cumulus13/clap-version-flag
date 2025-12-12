use thiserror::Error;

/// Error type for clap-version-flag
#[derive(Error, Debug)]
pub enum VersionError {
    /// Invalid hex color format
    #[error("Invalid hex color format: '{0}'. Expected format: #RRGGBB or #RGB")]
    InvalidHexColor(String),

    /// I/O error
    #[error("I/O error: {0}")]
    IoError(#[from] std::io::Error),
}

impl VersionError {
    /// Creates a new InvalidHexColor error
    pub fn invalid_hex(color: &str) -> Self {
        Self::InvalidHexColor(color.to_string())
    }
}
