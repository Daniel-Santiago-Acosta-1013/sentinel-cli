use std::{
    collections::{BTreeMap, BTreeSet},
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
    BlockedDomain,
    Error,
}

impl EventKind {
    pub fn label(&self) -> &'static str {
        match self {
            Self::SafetyCheck => "Chequeo de seguridad",
            Self::Enable => "Activacion de Sentinel",
            Self::Disable => "Desactivacion de Sentinel",
            Self::Recover => "Recuperacion de red",
            Self::Install => "Instalacion",
            Self::Update => "Actualizacion",
            Self::Reinstall => "Reinstalacion",
            Self::BlockedDomain => "Bloqueo de dominio",
            Self::Error => "Error operativo",
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Severity {
    Info,
    Warning,
    Error,
}

impl Severity {
    pub fn label(&self) -> &'static str {
        match self {
            Self::Info => "Info",
            Self::Warning => "Advertencia",
            Self::Error => "Error",
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EventRecord {
    pub id: String,
    pub timestamp: DateTime<Utc>,
    pub kind: EventKind,
    pub severity: Severity,
    #[serde(default)]
    pub blocked_domain: Option<String>,
    pub message: String,
}

impl EventRecord {
    pub fn new(kind: EventKind, severity: Severity, message: impl Into<String>) -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
            timestamp: Utc::now(),
            kind,
            severity,
            blocked_domain: None,
            message: message.into(),
        }
    }

    pub fn blocked_domain(domain: impl Into<String>) -> Self {
        let domain = domain.into();
        Self {
            id: Uuid::new_v4().to_string(),
            timestamp: Utc::now(),
            kind: EventKind::BlockedDomain,
            severity: Severity::Info,
            blocked_domain: Some(domain.clone()),
            message: format!("Sentinel bloqueo una consulta DNS para {domain}"),
        }
    }
}

#[derive(Debug, Clone, Default)]
pub struct BlockActivitySummary {
    pub blocked_since_activation: usize,
    pub unique_blocked_domains: usize,
    pub last_blocked_at: Option<DateTime<Utc>>,
    pub top_blocked_domains: Vec<(String, usize)>,
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
        let message = if summary.issues.is_empty() {
            summary.recommended_action.clone()
        } else {
            summary.issues.join(" | ")
        };
        self.append(EventRecord::new(EventKind::SafetyCheck, severity, message))
    }

    pub fn read_recent(&self, limit: usize) -> AppResult<Vec<EventRecord>> {
        let mut items = self.read_all()?;
        items.reverse();
        items.truncate(limit);
        Ok(items)
    }

    pub fn read_all(&self) -> AppResult<Vec<EventRecord>> {
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
        Ok(items)
    }

    pub fn record_blocked_domain(&self, domain: &str) -> AppResult<()> {
        self.append(EventRecord::blocked_domain(domain))
    }

    pub fn block_activity_since_activation(&self) -> AppResult<BlockActivitySummary> {
        let items = self.read_all()?;
        let last_enable_at = items
            .iter()
            .rev()
            .find(|event| matches!(event.kind, EventKind::Enable))
            .map(|event| event.timestamp);

        let blocked_events = items
            .into_iter()
            .filter(|event| matches!(event.kind, EventKind::BlockedDomain))
            .filter(|event| {
                last_enable_at.is_some_and(|timestamp| event.timestamp >= timestamp)
            })
            .collect::<Vec<_>>();

        if blocked_events.is_empty() {
            return Ok(BlockActivitySummary::default());
        }

        let mut unique_domains = BTreeSet::new();
        let mut counts = BTreeMap::<String, usize>::new();
        let mut last_blocked_at = None;
        for event in &blocked_events {
            if let Some(domain) = event.blocked_domain.as_ref() {
                unique_domains.insert(domain.clone());
                *counts.entry(domain.clone()).or_default() += 1;
            }
            last_blocked_at = Some(event.timestamp);
        }

        let mut top_blocked_domains = counts.into_iter().collect::<Vec<_>>();
        top_blocked_domains.sort_by(|left, right| {
            right
                .1
                .cmp(&left.1)
                .then_with(|| left.0.cmp(&right.0))
        });

        Ok(BlockActivitySummary {
            blocked_since_activation: blocked_events.len(),
            unique_blocked_domains: unique_domains.len(),
            last_blocked_at,
            top_blocked_domains,
        })
    }
}
