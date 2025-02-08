pub mod file;
pub mod processor;

use chrono::NaiveDate;

use crate::error::Result;
use crate::config::Config;
use crate::storage::StorageManager;
use self::file::FileRepository;

pub struct RustyDiary {
    file_repo: FileRepository,
    storage: StorageManager,
}

impl RustyDiary {
    pub fn new(config: Config) -> Result<Self> {
        let file_repo = FileRepository::new(
            &config.directory,
            config.output_file_prefix,
            &config.date_pattern
        )?;
        let storage = StorageManager::new(&config.db_path)?;

        Ok(Self {
            file_repo,
            storage,
        })
    }

    pub fn synchronize(&self) -> Result<(NaiveDate, NaiveDate)> {
        // Collect files that match our pattern
        let files = self.file_repo.collect_diary_files()?;

        // Get the next execution version
        let exec_version = self.storage.latest_exec_version()? + 1;

        // Process files into domain entries
        let file_entries = self.file_repo.process_files(&files, exec_version)?;
        let start_date = file_entries.last().unwrap().date;
        let end_date = file_entries.first().unwrap().date;

        // Fetch all stored entries
        let stored_entries = self.storage.entries_by_date_range(
            start_date,
            end_date
        )?;

        // Filter out new entries with the same content
        let new_entries: Vec<_> = file_entries.into_iter()
            .filter(|entry| {
            !stored_entries.iter().any(|stored_entry| {
                stored_entry.date == entry.date && stored_entry.content == entry.content
            })
            })
            .collect();

        // Store new entries in database
        self.storage.store_entries(new_entries)?;

        // Clean up processed files
        self.file_repo.cleanup_files(&files)?;

        Ok((start_date, end_date))
    }


    pub fn write_journal(&self, start_date: NaiveDate, end_date: NaiveDate) -> Result<()> {
        let entries = self.storage.entries_by_date_range(start_date, end_date)?;
        self.file_repo.write_entries(entries)?;

        Ok(())
    }
}