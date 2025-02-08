use std::path::PathBuf;

#[derive(Clone, Debug)]
pub struct Config {
    pub directory: PathBuf,
    pub date_pattern: String,
    pub output_file_prefix: String,
    pub db_path: PathBuf,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            directory: PathBuf::from("."),
            date_pattern: String::from(r"^(\d{4}-\d{2}-\d{2})(\.md)?$"),
            db_path: PathBuf::from("diary.db"),
            output_file_prefix: String::from("rusty-diary-log"),
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

    pub fn with_output_file_prefix(mut self, name: &str) -> Self {
        self.output_file_prefix = name.to_string();
        self
    }
}