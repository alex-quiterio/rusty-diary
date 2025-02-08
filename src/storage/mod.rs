mod db;
pub mod models;

use std::path::Path;
use chrono::NaiveDate;

pub use self::models::{DiaryEntry, EntryMetadata};
use crate::error::Result;

/// StorageManager provides a clean facade over our persistence operations.
/// It encapsulates the complexity of storage while providing a clear,
/// focused interface to clients.
pub struct StorageManager {
    repository: db::DiaryRepository,
}

impl StorageManager {
    /// Creates a new StorageManager with the given database path
    pub fn new<P: AsRef<Path>>(db_path: P) -> Result<Self> {
        Ok(Self {
            repository: db::DiaryRepository::new(db_path)?,
        })
    }


    /// Retrieves the latest execution version from storage
    pub fn latest_exec_version(&self) -> Result<i64> {
        self.repository.get_latest_exec_version()
    }

    /// Stores a batch of diary entries atomically
    pub fn store_entries(&self, entries: Vec<DiaryEntry>) -> Result<()> {
        // Pre-validate all entries before storage
        for entry in &entries {
            self.validate_entry(entry)?;
        }

        self.repository.store_batch(entries)
    }

    /// Retrieves entries within a date range
    pub fn entries_by_date_range(
        &self,
        start_date: NaiveDate,
        end_date: NaiveDate,
    ) -> Result<Vec<DiaryEntry>> {
        self.repository.get_entries_by_date_range(start_date, end_date)
    }

    /// Retrieves entries within a date range
    pub fn get_entries_by_exec_version(
        &self,
        exec_version: i64,
    ) -> Result<Vec<DiaryEntry>> {
        self.repository.get_entries_by_exec_version(exec_version)
    }

    /// Retrieves metadata for all entries
    pub fn get_metadata(&self) -> Result<Vec<EntryMetadata>> {
        self.repository.get_metadata()
    }

    // Private helper methods

    fn validate_entry(&self, entry: &DiaryEntry) -> Result<()> {
        if entry.content.trim().is_empty() {
            return Err(crate::error::RustyDiaryError::ContentIntegrity(
                "Empty content".to_string(),
            ));
        }
        Ok(())
    }
}


// Re-export essential types for convenience
pub use self::db::DiaryRepository;

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    fn create_test_entry(exec_version: i64, date: NaiveDate, content: &str) -> DiaryEntry {
        DiaryEntry::new(exec_version, date, content.to_string())
    }

    #[test]
    fn test_storage_manager() -> Result<()> {
        let temp_dir = TempDir::new()?;
        let db_path = temp_dir.path().join("test.db");

        let manager = StorageManager::new(&db_path)?;

        // Test storing and retrieving entries
        let test_date = NaiveDate::from_ymd_opt(2024, 1, 1).unwrap();
        let entry = create_test_entry(1, test_date, "Test content");

        manager.store_entries(vec![entry])?;

        let retrieved = manager.entries_by_date_range(
            test_date,
            test_date,
        )?;

        assert_eq!(retrieved.len(), 1);
        assert_eq!(retrieved[0].content, "Test content");

        Ok(())
    }
}