use rusqlite::{Connection, Transaction, params, Result as SqlResult};
use chrono::{NaiveDate};
use std::path::Path;
use std::sync::Arc;
use parking_lot::RwLock;

use crate::error::{Result, RustyDiaryError};
use super::models::{DiaryEntry, EntryMetadata};

const PRAGMAS: &str = "
    PRAGMA foreign_keys = ON;
    PRAGMA journal_mode = WAL;
    PRAGMA synchronous = NORMAL;
";

const MIGRATIONS: &[&str] = &[
    // V1: Initial schema
    "CREATE TABLE IF NOT EXISTS diary_entries (
        exec_version INTEGER NOT NULL,
        date TEXT NOT NULL,
        content TEXT NOT NULL,
        created_at TEXT NOT NULL,
        updated_at TEXT,
        PRIMARY KEY (exec_version, date)
    );

    CREATE INDEX IF NOT EXISTS idx_diary_entries_date
    ON diary_entries(date);",

    // V2: Add metadata table
    "CREATE TABLE IF NOT EXISTS entry_metadata (
        entry_id INTEGER PRIMARY KEY,
        exec_version INTEGER NOT NULL,
        date TEXT NOT NULL,
        word_count INTEGER NOT NULL,
        FOREIGN KEY (exec_version, date)
        REFERENCES diary_entries(exec_version, date)
        ON DELETE CASCADE
    );"
];

/// Repository implementation for diary entries
/// Follows the Repository pattern to provide a clean persistence abstraction
pub struct DiaryRepository {
    conn: Arc<RwLock<Connection>>,
}

impl DiaryRepository {
    pub fn new<P: AsRef<Path>>(db_path: P) -> Result<Self> {
        let conn = Connection::open(db_path)?;

        // Initialize database with optimal settings
        conn.execute_batch(PRAGMAS)?;

        let repo = Self {
            conn: Arc::new(RwLock::new(conn)),
        };

        repo.migrate()?;
        Ok(repo)
    }

    /// Stores a batch of entries atomically
    pub fn store_batch(&self, entries: Vec<DiaryEntry>) -> Result<()> {
        let mut conn = self.conn.write();
        let tx = conn.transaction()?;

        println!("Storing batch of entries: {:#?}", entries);

        for entry in entries {
            self.store_entry_internal(&tx, &entry)?;
        }

        tx.commit()?;
        Ok(())
    }

    /// Retrieves entries within a date range
    pub fn get_entries_by_date_range(
        &self,
        start_date: NaiveDate,
        end_date: NaiveDate,
    ) -> Result<Vec<DiaryEntry>> {
        let conn = self.conn.read();
        let mut stmt = conn.prepare(
            "SELECT
                exec_version, date, content, created_at, updated_at
             FROM diary_entries
             WHERE date BETWEEN ?1 AND ?2
             ORDER BY date DESC, exec_version DESC"
        )?;

        let entries = stmt.query_map(
            params![start_date.to_string(), end_date.to_string()],
            |row| self.map_row_to_entry(row)
        )?;

        entries.collect::<SqlResult<Vec<_>>>()
            .map_err(RustyDiaryError::from)
    }

    /// Retrieves entries within a date range
    pub fn get_entries_by_exec_version(
        &self,
        exec_version: i64,
    ) -> Result<Vec<DiaryEntry>> {
        let conn = self.conn.read();
        let mut stmt = conn.prepare(
            "SELECT
                exec_version, date, content, created_at, updated_at
             FROM diary_entries
             WHERE exec_version = :exec_version
             ORDER BY date DESC"
        )?;

        let entries = stmt.query_map(
            &[(":exec_version", &exec_version)],
            |row| self.map_row_to_entry(row)
        )?;

        entries.collect::<SqlResult<Vec<_>>>()
            .map_err(RustyDiaryError::from)
    }

    /// Gets the latest execution version
    pub fn get_latest_exec_version(&self) -> Result<i64> {
        self.conn.read()
            .query_row(
                "SELECT COALESCE(MAX(exec_version), 0) FROM diary_entries",
                [],
                |row| row.get(0)
            )
            .map_err(RustyDiaryError::from)
    }

    /// Retrieves metadata for statistical analysis
    pub fn get_metadata(&self) -> Result<Vec<EntryMetadata>> {
        let conn = self.conn.read();
        let mut stmt = conn.prepare(
            "SELECT
                e.date,
                m.word_count,
                e.exec_version
             FROM diary_entries e
             JOIN entry_metadata m ON
                e.exec_version = m.exec_version AND
                e.date = m.date
             ORDER BY 1,3 DESC"
        )?;

        let metadata = stmt.query_map([], |row| {
            Ok(EntryMetadata {
                date: NaiveDate::parse_from_str(&row.get::<_, String>(0)?, "%Y-%m-%d")
                    .map_err(|e| rusqlite::Error::FromSqlConversionFailure(0, rusqlite::types::Type::Text, Box::new(e)))?,
                word_count: row.get(1)?,
                exec_version: row.get(2)?,
            })
        })?;

        metadata.collect::<SqlResult<Vec<_>>>()
            .map_err(RustyDiaryError::from)
    }

    // Private helper methods

    fn migrate(&self) -> Result<()> {
        let mut conn = self.conn.write();

        // Create migration table if it doesn't exist
        conn.execute(
            "CREATE TABLE IF NOT EXISTS schema_migrations (
                version INTEGER PRIMARY KEY
            )",
            [],
        )?;

        // Get current version
        let current_version: i32 = conn
            .query_row(
                "SELECT COALESCE(MAX(version), 0) FROM schema_migrations",
                [],
                |row| row.get(0),
            )?;

        // Apply pending migrations
        let tx = conn.transaction()?;
        for (i, migration) in MIGRATIONS.iter().enumerate() {
            let version = (i + 1) as i32;
            if version > current_version {
                tx.execute_batch(migration)?;
                tx.execute(
                    "INSERT INTO schema_migrations (version) VALUES (?1)",
                    params![version],
                )?;
            }
        }
        tx.commit()?;

        Ok(())
    }

    fn store_entry_internal(&self, tx: &Transaction, entry: &DiaryEntry) -> Result<()> {
        // Store main entry
        tx.execute(
            "INSERT OR REPLACE INTO diary_entries
                (exec_version, date, content, created_at, updated_at)
             VALUES (?1, ?2, ?3, ?4, ?5)",
            params![
                entry.exec_version,
                entry.date.to_string(),
                entry.content,
                entry.created_at.to_string(),
                entry.updated_at.map(|dt| dt.to_string()),
            ],
        )?;

        // Store metadata
        tx.execute(
            "INSERT OR REPLACE INTO entry_metadata
                (exec_version, date, word_count)
             VALUES (?1, ?2, ?3)",
            params![
                entry.exec_version,
                entry.date.to_string(),
                entry.word_count(),
            ],
        )?;

        Ok(())
    }

    fn map_row_to_entry(&self, row: &rusqlite::Row) -> SqlResult<DiaryEntry> {
        Ok(DiaryEntry {
            exec_version: row.get(0)?,
            date: row.get(1)?,
            content: row.get(2)?,
            created_at: row.get(3)?,
            updated_at: row.get(4)?,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    #[test]
    fn test_entry_lifecycle() -> Result<()> {
        let temp_dir = TempDir::new()?;
        let db_path = temp_dir.path().join("test.db");
        let repo = DiaryRepository::new(&db_path)?;

        let test_date = NaiveDate::from_ymd_opt(2024, 1, 1).unwrap();
        let entry = DiaryEntry::new(1, test_date, "Test content".to_string());

        // Store entry
        repo.store_batch(vec![entry.clone()])?;

        // Retrieve and verify
        let entries = repo.get_entries_by_date_range(test_date, test_date)?;
        assert_eq!(entries.len(), 1);
        assert_eq!(entries[0].content, "Test content");

        // Verify metadata
        let metadata = repo.get_metadata()?;
        assert_eq!(metadata.len(), 1);
        assert_eq!(metadata[0].word_count, 2); // "Test content" has 2 words

        Ok(())
    }
}