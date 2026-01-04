use std::cmp::PartialEq;
use chrono::{NaiveDate, NaiveDateTime};
use regex::Regex;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, PartialEq, Deserialize)]
pub struct DiaryEntry {
    pub exec_version: i64,
    pub date: NaiveDate,
    pub content: String,
    pub created_at: NaiveDateTime,
    pub updated_at: Option<NaiveDateTime>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EntryMetadata {
    pub date: NaiveDate,
    pub word_count: usize,
    pub exec_version: i64,
}

impl DiaryEntry {
    pub fn new(exec_version: i64, date: NaiveDate, content: String) -> Self {
        let now = chrono::Local::now().naive_local();
        let metadata_pattern = Regex::new(
            r"(?s)\A---\s*(?:.*\n)*?tags:\s*(?:.*\n)*?(?:date:\s*[^\n]+\n)?(?:.*\n)*?---\s*"
        ).unwrap();

        let stripped_content = metadata_pattern.replace(&content, "").to_string();
        Self {
            exec_version,
            date,
            content: stripped_content.lines().collect::<Vec<&str>>().join("\n"),
            created_at: now,
            updated_at: Some(now),
        }
    }

    pub fn eq(&self, other: &Self) -> bool {
        self.date == other.date && self.content == other.content
    }

    pub fn word_count(&self) -> usize {
        self.content.split_whitespace().count()
    }

    pub fn metadata(&self) -> EntryMetadata {
        EntryMetadata {
            date: self.date,
            word_count: self.word_count(),
            exec_version: self.exec_version,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_diary_entry_new() {
        let date = NaiveDate::from_ymd_opt(2023, 1, 1).unwrap();
        let content = "Some diary content";
        let entry = DiaryEntry::new(1, date, content.to_string());

        assert_eq!(entry.exec_version, 1);
        assert_eq!(entry.date, date);
        assert_eq!(entry.content, content);
        assert!(entry.updated_at.is_some());
    }

    #[test]
    fn test_strip_metadata() {
        let date = NaiveDate::from_ymd_opt(2023, 1, 1).unwrap();
        let content = "---\ntags:\n  - reflections\ndate: 2025-06-07\n---\n##Actual content here";
        let entry = DiaryEntry::new(1, date, content.to_string());

        assert_eq!(entry.content, "##Actual content here");
    }

    #[test]
    fn test_metadata_extraction() {
        let date = NaiveDate::from_ymd_opt(2023, 1, 1).unwrap();
        let content = "This is a test with multiple words";
        let entry = DiaryEntry::new(1, date, content.to_string());
        let metadata = entry.metadata();

        assert_eq!(metadata.date, date);
        assert_eq!(metadata.word_count, 7);
        assert_eq!(metadata.exec_version, 1);
    }

    #[test]
    fn test_eq_implementation() {
        let date = NaiveDate::from_ymd_opt(2023, 1, 1).unwrap();
        let entry1 = DiaryEntry::new(1, date.clone(), "Content".to_string());
        let entry2 = DiaryEntry::new(2, date.clone(), "Content".to_string());
        let entry3 = DiaryEntry::new(1, date, "Different content".to_string());

        assert!(entry1.eq(&entry2)); // Same content and date
        assert!(!entry1.eq(&entry3)); // Different content
    }

    #[test]
    fn test_word_count() {
        let date = NaiveDate::from_ymd_opt(2023, 1, 1).unwrap();
        let entry = DiaryEntry::new(1, date, "One two three\nfour five".to_string());

        assert_eq!(entry.word_count(), 5);
    }
}
