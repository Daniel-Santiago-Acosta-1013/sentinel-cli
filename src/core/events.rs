use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum OperationEventKind {
    Activate,
    Disable,
    Status,
    Allow,
    RemoveAllow,
    Error,
    Recover,
}

impl OperationEventKind {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Activate => "activate",
            Self::Disable => "disable",
            Self::Status => "status",
            Self::Allow => "allow",
            Self::RemoveAllow => "remove_allow",
            Self::Error => "error",
            Self::Recover => "recover",
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
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Info => "info",
            Self::Warning => "warning",
            Self::Error => "error",
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OperationEvent {
    pub id: String,
    pub timestamp: DateTime<Utc>,
    pub kind: OperationEventKind,
    pub severity: Severity,
    pub message: String,
}

impl OperationEvent {
    pub fn new(
        kind: OperationEventKind,
        severity: Severity,
        message: impl Into<String>,
    ) -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
            timestamp: Utc::now(),
            kind,
            severity,
            message: message.into(),
        }
    }
}
