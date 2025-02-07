pub mod config;
pub mod error;
pub mod storage;
pub mod diary;

// Re-export the essential types, like stars made visible
pub use config::Config;
pub use error::RustyDiaryError;
pub use diary::RustyDiary;

// Version whispers
pub const VERSION: &str = env!("CARGO_PKG_VERSION");