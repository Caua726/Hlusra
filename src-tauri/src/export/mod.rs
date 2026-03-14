pub mod audio;
pub mod commands;
pub mod transcript;
pub mod types;
pub mod video;

use std::path::PathBuf;

/// Error type for export operations.
#[derive(Debug, thiserror::Error)]
pub enum ExportError {
    #[error("Source file not found: {0}")]
    SourceNotFound(PathBuf),
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    #[error("FFmpeg failed: {0}")]
    FfmpegFailed(String),
    #[error("Invalid transcript data: {0}")]
    InvalidTranscript(String),
    #[error("PDF generation error: {0}")]
    PdfGeneration(String),
    #[error("Library error: {0}")]
    Library(String),
}

impl serde::Serialize for ExportError {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}
