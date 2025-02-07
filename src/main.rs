use std::fs::{self, File};
use std::io::{self, Write};
use std::path::PathBuf;
use std::env;
use regex::Regex;
use thiserror::Error;

#[derive(Error, core::fmt::Debug)]
pub enum MergerError {
    #[error("IO error: {0}")]
    Io(#[from] io::Error),
    #[error("Invalid directory path: {0}")]
    InvalidDirectory(String),
    #[error("Invalid date pattern: {0}")]
    InvalidPattern(#[from] regex::Error),
    #[error("No files found matching the pattern")]
    NoFilesFound,
    #[error("Failed to remove files after merging")]
    CleanupError,
}

pub struct Config {
    pub directory: PathBuf,
    pub date_pattern: String,
    pub output_filename: String,
    pub separator: String,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            directory: PathBuf::from("."),
            date_pattern: String::from(r"^\d{4}-\d{2}-\d{2}(\.md)?$"),
            output_filename: String::from("writing-log.md"),
            separator: String::from("\n***\n"),
        }
    }
}

pub struct FileMerger {
    config: Config,
}

impl FileMerger {
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    pub fn run(&self) -> Result<(), MergerError> {
        self.verify_directory()?;
        let date_pattern = self.compile_pattern()?;
        let files = self.collect_files(&date_pattern)?;
        self.merge_files(&files)?;
        self.cleanup_files(&files)?;
        Ok(())
    }

    fn verify_directory(&self) -> Result<(), MergerError> {
        if !self.config.directory.is_dir() {
            return Err(MergerError::InvalidDirectory(
                self.config.directory.display().to_string(),
            ));
        }
        Ok(())
    }

    fn compile_pattern(&self) -> Result<Regex, MergerError> {
        Regex::new(&self.config.date_pattern).map_err(MergerError::InvalidPattern)
    }

    fn collect_files(&self, date_pattern: &Regex) -> Result<Vec<PathBuf>, MergerError> {
        let mut files: Vec<_> = fs::read_dir(&self.config.directory)?
            .filter_map(Result::ok)
            .map(|entry| entry.path())
            .filter(|path| {
                path.is_file() && path.file_name()
                    .and_then(|s| s.to_str())
                    .map_or(false, |filename| date_pattern.is_match(filename))
            })
            .collect();

        if files.is_empty() {
            return Err(MergerError::NoFilesFound);
        }

        files.sort_by(|a, b| b.cmp(a));

        Ok(files)
    }

    fn merge_files(&self, files: &[PathBuf]) -> Result<(), MergerError> {
        let output_path = self.config.directory.join(&self.config.output_filename);
        let existing_content = fs::read_to_string(&output_path).unwrap_or_else(|_| String::new());
        let mut output = File::create(&output_path)?;

        for (i, file_path) in files.iter().enumerate() {
            let file_content = fs::read_to_string(file_path)?;
            writeln!(output, "{}", file_content)?;
            // Write separator only if it's not the last file
            if i < files.len() - 1 {
                write!(output, "{}", self.config.separator)?;
            }
        }

        write!(output, "{}", existing_content)?;
        Ok(())
    }

    fn cleanup_files(&self, files: &[PathBuf]) -> Result<(), MergerError> {
        for file_path in files {
            // Skip the output file if it's in the same directory
            if file_path.file_name() == Some(self.config.output_filename.as_ref()) {
                continue;
            }

            if let Err(e) = fs::remove_file(file_path) {
                eprintln!("Failed to remove file {}: {}", file_path.display(), e);
                return Err(MergerError::CleanupError);
            }
        }
        Ok(())
    }
}

fn main() -> Result<(), MergerError> {
    let config = Config {
        directory: env::args()
            .nth(1)
            .map(PathBuf::from)
            .unwrap_or_else(|| PathBuf::from(".")),
        ..Config::default()
    };

    let merger = FileMerger::new(config);
    merger.run()?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;
    use std::fs::File;
    use std::io::Write;
    use std::path::Path;

    fn create_test_file(dir: &Path, name: &str, content: &str) -> io::Result<()> {
        let path = dir.join(name);
        let mut file = File::create(path)?;
        write!(file, "{}", content)?;
        Ok(())
    }

    #[test]
    fn test_file_merger() -> Result<(), Box<dyn std::error::Error>> {
        let temp_dir = TempDir::new()?;

        // Create test files
        create_test_file(&temp_dir.path(), "2024-01-01.md", "Test content 1")?;
        create_test_file(&temp_dir.path(), "2024-01-02.md", "Test content 2")?;

        let config = Config {
            directory: temp_dir.path().to_path_buf(),
            ..Config::default()
        };

        let merger = FileMerger::new(config);
        merger.run()?;

        // Verify output
        let output_content = fs::read_to_string(temp_dir.path().join("writing-log.md"))?;
        assert!(output_content.contains("Test content 1"));
        assert!(output_content.contains("Test content 2"));

        Ok(())
    }

    #[test]
    fn test_empty_directory() {
        let temp_dir = TempDir::new().unwrap();
        let config = Config {
            directory: temp_dir.path().to_path_buf(),
            ..Config::default()
        };

        let merger = FileMerger::new(config);
        assert!(matches!(merger.run(), Err(MergerError::NoFilesFound)));
    }
}
