pub mod file;
pub mod processor;

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
        let file_repo = FileRepository::new(&config.directory, &config.date_pattern)?;
        let storage = StorageManager::new(&config.db_path)?;

        Ok(Self {
            file_repo,
            storage,
        })
    }

    pub fn merge(&self) -> Result<()> {
        // Collect files that match our pattern
        let files = self.file_repo.collect_diary_files()?;

        // Get the next execution version
        let exec_version = self.storage.latest_exec_version()? + 1;

        // Process files into domain entries
        let entries = self.file_repo.process_files(&files, exec_version)?;

        // Store entries in database
        self.storage.store_entries(entries)?;

        // Clean up processed files
        self.file_repo.cleanup_files(&files)?;

        Ok(())
    }
}