use std::{
    fs::OpenOptions,
    io::{BufRead, BufReader, Write},
};

use chrono::{DateTime, Utc};
use miette::IntoDiagnostic;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{
    app::{AppPaths, AppResult},
    storage::state::SafetyCheckSummary,
};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum EventKind {
    SafetyCheck,
    Enable,
    Disable,
    Recover,
    Install,
    Update,
    Reinstall,
    Error,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Severity {
    Info,
    Warning,
    Error,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EventRecord {
    pub id: String,
    pub timestamp: DateTime<Utc>,
    pub kind: EventKind,
    pub severity: Severity,
    pub message: String,
}

impl EventRecord {
    pub fn new(kind: EventKind, severity: Severity, message: impl Into<String>) -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
            timestamp: Utc::now(),
            kind,
            severity,
            message: message.into(),
        }
    }
}

pub struct EventStore {
    paths: AppPaths,
}

impl EventStore {
    pub fn new(paths: AppPaths) -> Self {
        Self { paths }
    }

    pub fn append(&self, event: EventRecord) -> AppResult<()> {
        let mut file = OpenOptions::new()
            .create(true)
            .append(true)
            .open(&self.paths.events_file)
            .into_diagnostic()?;
        writeln!(file, "{}", serde_json::to_string(&event).into_diagnostic()?)
            .into_diagnostic()?;
        Ok(())
    }

    pub fn record_safety(&self, summary: &SafetyCheckSummary) -> AppResult<()> {
        let severity = match summary.status {
            crate::storage::state::SafetyStatus::Pass => Severity::Info,
            crate::storage::state::SafetyStatus::Warn => Severity::Warning,
            crate::storage::state::SafetyStatus::Fail => Severity::Error,
        };
        self.append(EventRecord::new(
            EventKind::SafetyCheck,
            severity,
            summary.recommended_action.clone(),
        ))
    }

    #[allow(dead_code)]
    pub fn read_recent(&self, limit: usize) -> AppResult<Vec<EventRecord>> {
        if !self.paths.events_file.exists() {
            return Ok(Vec::new());
        }

        let file = OpenOptions::new()
            .read(true)
            .open(&self.paths.events_file)
            .into_diagnostic()?;
        let reader = BufReader::new(file);
        let mut items = Vec::new();
        for line in reader.lines() {
            let line = line.into_diagnostic()?;
            if line.trim().is_empty() {
                continue;
            }
            items.push(serde_json::from_str::<EventRecord>(&line).into_diagnostic()?);
        }
        items.reverse();
        items.truncate(limit);
        Ok(items)
    }
}
