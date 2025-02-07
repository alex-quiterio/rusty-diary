use std::path::PathBuf;

#[derive(Clone, Debug)]
pub struct Config {
    pub directory: PathBuf,
    pub date_pattern: String,
    pub db_path: PathBuf,
    pub retention_days: Option<u32>, // How long memories persist
}

impl Default for Config {
    fn default() -> Self {
        Self {
            directory: PathBuf::from("."),
            date_pattern: String::from(r"^(\d{4}-\d{2}-\d{2})(\.md)?$"),
            db_path: PathBuf::from("diary.db"),
            retention_days: None,
        }
    }
}

impl Config {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_directory<P: Into<PathBuf>>(mut self, path: P) -> Self {
        self.directory = path.into();
        self
    }

    pub fn with_db<P: Into<PathBuf>>(mut self, path: P) -> Self {
        self.db_path = path.into();
        self
    }

    pub fn with_date_pattern(mut self, pattern: &str) -> Self {
        self.date_pattern = pattern.to_string();
        self
    }
}