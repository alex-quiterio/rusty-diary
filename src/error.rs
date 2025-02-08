use std::path::PathBuf;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum RustyDiaryError {
    #[error("IO operation failed: {0}")]
    Io(#[from] std::io::Error),

    #[error("Database error: {0}")]
    Database(#[from] rusqlite::Error),

    #[error("Date parsing failed: {0}")]
    DateParse(#[from] chrono::ParseError),

    #[error("Invalid directory path: {0}")]
    InvalidDirectory(PathBuf),

    #[error("Invalid date pattern: {0}")]
    InvalidPattern(#[from] regex::Error),

    #[error("No matching files found in {0}")]
    NoFilesFound(PathBuf),

    #[error("Content integrity error: {0}")]
    ContentIntegrity(String),
}

pub type Result<T> = std::result::Result<T, RustyDiaryError>;