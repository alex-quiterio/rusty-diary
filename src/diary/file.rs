use std::path::{Path, PathBuf};
use walkdir::WalkDir;
use std::fs;

use crate::error::{Result, RustyDiaryError};
use crate::storage::models::DiaryEntry;
use super::processor::MarkdownProcessor;

/// FileRepository handles all file system operations.
/// It follows the Repository pattern to provide a clean abstraction
/// over file system interactions.
pub struct FileRepository {
    root_dir: PathBuf,
    markdown_processor: MarkdownProcessor,
}

impl FileRepository {
    pub fn new<P: AsRef<Path>>(root_dir: P, date_pattern: &str) -> Result<Self> {
        let root_dir = root_dir.as_ref().to_path_buf();
        let markdown_processor = MarkdownProcessor::new(date_pattern)?;

        Ok(Self {
            root_dir,
            markdown_processor,
        })
    }

    /// Collects all markdown files that match our date pattern
    pub fn collect_diary_files(&self) -> Result<Vec<PathBuf>> {
        let entries: Vec<PathBuf> = WalkDir::new(&self.root_dir)
            .min_depth(0)
            .max_depth(1)  // Only look in the immediate directory
            .into_iter()
            .filter_map(|entry| entry.ok())
            .map(|entry| entry.path().to_path_buf())
            .filter(|path| self.is_valid_diary_file(path))
            .collect();

        if entries.is_empty() {
            return Err(RustyDiaryError::NoFilesFound(self.root_dir.clone()));
        }

        Ok(entries)
    }

    /// Process a set of files into DiaryEntries
    pub fn process_files(&self, files: &[PathBuf], exec_version: i64) -> Result<Vec<DiaryEntry>> {
        let mut entries = Vec::new();
        let mut errors = Vec::new();

        for file in files {
            match self.process_single_file(file, exec_version) {
                Ok(entry) => entries.push(entry),
                Err(e) => errors.push((file.clone(), e)),
            }
        }

        // Log any errors encountered during processing
        if !errors.is_empty() {
            for (file, error) in &errors {
                tracing::warn!("Error processing file {:?}: {}", file, error);
            }
        }

        Ok(entries)
    }

    /// Clean up processed files
    pub fn cleanup_files(&self, files: &[PathBuf]) -> Result<()> {
        for file in files {
            if let Err(e) = fs::remove_file(file) {
                tracing::warn!("Failed to remove file {:?}: {}", file, e);
            }
        }
        Ok(())
    }

    // Private helper methods

    fn is_valid_diary_file(&self, path: &Path) -> bool {
        if !path.is_file() {
            return false;
        }

        // Check if it's a markdown file
        match path.extension() {
            Some(ext) if ext == "md" => (),
            _ => return false,
        }

        // Verify the filename matches our date pattern
        self.markdown_processor
            .extract_date(path)
            .is_ok()
    }

    fn process_single_file(&self, path: &Path, exec_version: i64) -> Result<DiaryEntry> {
        let content = fs::read_to_string(path)?;

        // Validate content before processing
        self.markdown_processor.validate_content(&content)?;

        // Extract date from filename
        let date = self.markdown_processor.extract_date(path)?;

        Ok(DiaryEntry::new(exec_version, date, content))
    }

    pub fn backup_file<P: AsRef<Path>>(&self, path: P) -> Result<PathBuf> {
        let path = path.as_ref();
        let backup_dir = self.root_dir.join(".backup");
        fs::create_dir_all(&backup_dir)?;

        let filename = path.file_name()
            .ok_or_else(|| RustyDiaryError::ContentIntegrity(
                "Invalid filename".to_string()
            ))?;

        let timestamp = chrono::Local::now().format("%Y%m%d_%H%M%S");
        let backup_path = backup_dir.join(format!("{}_{}", timestamp, filename.to_string_lossy()));

        fs::copy(path, &backup_path)?;
        Ok(backup_path)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;
    use chrono::NaiveDate;
    use std::fs::File;
    use std::io::Write;

    fn create_test_file(dir: &Path, name: &str, content: &str) -> Result<PathBuf> {
        let path = dir.join(name);
        let mut file = File::create(&path)?;
        writeln!(file, "{}", content)?;
        Ok(path)
    }

    #[test]
    fn test_file_collection() -> Result<()> {
        let temp_dir = TempDir::new()?;
        let repo = FileRepository::new(temp_dir.path(), r"^(\d{4}-\d{2}-\d{2})(\.md)?$")?;

        // Create test files
        create_test_file(temp_dir.path(), "2024-01-01.md", "Test content 1")?;
        create_test_file(temp_dir.path(), "2024-01-02.md", "Test content 2")?;
        create_test_file(temp_dir.path(), "invalid.md", "Invalid file")?;

        let files = repo.collect_diary_files()?;
        assert_eq!(files.len(), 2);

        // Verify files are sorted correctly
        let filenames: Vec<_> = files.iter()
            .filter_map(|p| p.file_name())
            .filter_map(|n| n.to_str())
            .collect();

        assert!(filenames.contains(&"2024-01-01.md"));
        assert!(filenames.contains(&"2024-01-02.md"));

        Ok(())
    }

    #[test]
    fn test_file_processing() -> Result<()> {
        let temp_dir = TempDir::new()?;
        let repo = FileRepository::new(temp_dir.path(), r"^(\d{4}-\d{2}-\d{2})(\.md)?$")?;

        let test_file = create_test_file(
            temp_dir.path(),
            "2024-01-01.md",
            "Test content",
        )?;

        let entries = repo.process_files(&[test_file], 1)?;
        assert_eq!(entries.len(), 1);

        let entry = &entries[0];
        assert_eq!(entry.exec_version, 1);
        assert_eq!(entry.date, NaiveDate::from_ymd_opt(2024, 1, 1).unwrap());
        assert_eq!(entry.content.trim(), "Test content");

        Ok(())
    }
}