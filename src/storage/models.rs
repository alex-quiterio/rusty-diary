use std::cmp::PartialEq;
use chrono::{NaiveDate, NaiveDateTime};
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
        let content = content.lines().skip(1).collect::<Vec<&str>>().join("\n");
        Self {
            exec_version,
            date,
            content,
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