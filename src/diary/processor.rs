use chrono::NaiveDate;
use regex::Regex;
use std::path::Path;

use crate::error::{Result, RustyDiaryError};
use crate::storage::models::DiaryEntry;

pub struct MarkdownProcessor {
    date_pattern: Regex,
}

impl MarkdownProcessor {
    pub fn new(date_pattern: &str) -> Result<Self> {
        Ok(Self {
            date_pattern: Regex::new(date_pattern)
                .map_err(RustyDiaryError::InvalidPattern)?,
        })
    }

    pub fn process_file<P: AsRef<Path>>(&self, path: P, exec_version: i64) -> Result<DiaryEntry> {
        let path = path.as_ref();
        let content = std::fs::read_to_string(path)?;

        let date = self.extract_date(path)?;

        Ok(DiaryEntry::new(exec_version, date, content))
    }

    pub fn extract_date<P: AsRef<Path>>(&self, path: P) -> Result<NaiveDate> {
        let filename = path
            .as_ref()
            .file_name()
            .and_then(|name| name.to_str())
            .ok_or_else(|| RustyDiaryError::ContentIntegrity(
                "Invalid filename".to_string()
            ))?;

        let date_str = self
            .date_pattern
            .captures(filename)
            .and_then(|caps| caps.get(1))
            .map(|m| m.as_str())
            .ok_or_else(|| RustyDiaryError::ContentIntegrity(
                format!("Filename does not match pattern: {}", filename)
            ))?;

        NaiveDate::parse_from_str(date_str, "%Y-%m-%d")
            .map_err(RustyDiaryError::from)
    }

    pub fn validate_content(&self, content: &str) -> Result<()> {
        if content.trim().is_empty() {
            return Err(RustyDiaryError::ContentIntegrity(
                "Empty content".to_string()
            ));
        }

        // Add more content validation as needed
        Ok(())
    }
}